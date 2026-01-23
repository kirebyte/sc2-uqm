/**
 * @file system_config.c
 * @brief Implementation of Centralized System Configuration for UQM
 * 
 * This implementation provides centralized configuration management for the entire
 * UQM system, implementing type-safe configuration values with validation,
 * persistence, and change notifications.
 * 
 * @author The Ur-Quan Masters Refactoring Team
 * @version 2.0.0
 * @date 2025-01-27
 * 
 * @copyright Copyright (c) 2002 Toys for Bob, Inc.
 * @license GNU General Public License v2.0
 */

#include "system_config.h"
#include "../logging/modern_logger.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdatomic.h>
#include <pthread.h>

/* ========================================================================== */
/*                               CONSTANTS                                    */
/* ========================================================================== */

/** @brief Maximum number of configuration entries */
#define UQM_MAX_CONFIG_ENTRIES 256

/** @brief Maximum number of change callbacks */
#define UQM_MAX_CONFIG_CALLBACKS 64

/** @brief Maximum length for configuration strings */
#define UQM_MAX_CONFIG_STRING_LENGTH 512

/* ========================================================================== */
/*                            INTERNAL STRUCTURES                            */
/* ========================================================================== */

/**
 * @brief Internal configuration entry
 */
typedef struct {
    UQM_ConfigEntry definition;     /**< Configuration entry definition */
    UQM_ConfigValue current_value;  /**< Current value */
    bool is_set;                    /**< Whether value has been set */
    bool is_active;                 /**< Whether entry is active */
} UQM_InternalConfigEntry;

/**
 * @brief Configuration change callback registration
 */
typedef struct {
    uint32_t id;                         /**< Callback ID */
    char* key_filter;                    /**< Key filter (NULL for all) */
    UQM_ConfigChangeCallback callback;   /**< Callback function */
    void* user_data;                     /**< User data */
    bool is_active;                      /**< Whether callback is active */
} UQM_ConfigCallbackRegistration;

/**
 * @brief Configuration system state
 */
typedef struct {
    UQM_InternalConfigEntry entries[UQM_MAX_CONFIG_ENTRIES];  /**< Configuration entries */
    UQM_ConfigCallbackRegistration callbacks[UQM_MAX_CONFIG_CALLBACKS]; /**< Callbacks */
    
    atomic_int entry_count;        /**< Number of active entries */
    atomic_int callback_count;     /**< Number of active callbacks */
    atomic_uint next_callback_id;  /**< Next callback ID */
    
    pthread_mutex_t mutex;         /**< Thread safety mutex */
    bool is_initialized;           /**< Initialization flag */
    
    char* config_file_path;        /**< Current config file path */
} UQM_ConfigSystem;

/* ========================================================================== */
/*                              GLOBAL STATE                                 */
/* ========================================================================== */

/** @brief Global configuration system instance */
static UQM_ConfigSystem g_config_system = {0};

/* ========================================================================== */
/*                           INTERNAL FUNCTIONS                              */
/* ========================================================================== */

/**
 * @brief Find configuration entry by key
 * @param key Configuration key
 * @return Pointer to entry or NULL if not found
 */
static UQM_InternalConfigEntry* find_config_entry(const char* key) {
    if (!key) return NULL;
    
    for (int i = 0; i < UQM_MAX_CONFIG_ENTRIES; i++) {
        if (g_config_system.entries[i].is_active &&
            strcmp(g_config_system.entries[i].definition.key, key) == 0) {
            return &g_config_system.entries[i];
        }
    }
    
    return NULL;
}

/**
 * @brief Find empty configuration entry slot
 * @return Pointer to empty entry or NULL if no slots available
 */
static UQM_InternalConfigEntry* find_empty_config_entry(void) {
    for (int i = 0; i < UQM_MAX_CONFIG_ENTRIES; i++) {
        if (!g_config_system.entries[i].is_active) {
            return &g_config_system.entries[i];
        }
    }
    
    return NULL;
}

/**
 * @brief Validate configuration value
 * @param entry Configuration entry
 * @param value Value to validate
 * @return true if valid, false otherwise
 */
static bool validate_config_value(const UQM_InternalConfigEntry* entry, const UQM_ConfigValue* value) {
    if (!entry || !value) return false;
    
    const UQM_ConfigEntry* def = &entry->definition;
    
    // Type-specific validation
    switch (def->type) {
        case UQM_CONFIG_TYPE_INT:
            if (value->int_val < def->min_value.int_val || 
                value->int_val > def->max_value.int_val) {
                return false;
            }
            break;
            
        case UQM_CONFIG_TYPE_FLOAT:
            if (value->float_val < def->min_value.float_val || 
                value->float_val > def->max_value.float_val) {
                return false;
            }
            break;
            
        case UQM_CONFIG_TYPE_STRING:
            if (value->string_val && strlen(value->string_val) >= UQM_MAX_CONFIG_STRING_LENGTH) {
                return false;
            }
            break;
            
        case UQM_CONFIG_TYPE_ENUM:
            if (def->enum_values) {
                bool found = false;
                for (int i = 0; def->enum_values[i]; i++) {
                    if (value->enum_val == i) {
                        found = true;
                        break;
                    }
                }
                if (!found) return false;
            }
            break;
            
        case UQM_CONFIG_TYPE_RESOLUTION:
            if (value->resolution.width <= 0 || value->resolution.height <= 0) {
                return false;
            }
            break;
            
        default:
            break;
    }
    
    // Custom validator
    if (def->validator) {
        UQM_Result(bool, UQM_ConfigError) result = def->validator(value, def->validator_data);
        if (UQM_IS_ERROR(result)) {
            return false;
        }
    }
    
    return true;
}

/**
 * @brief Copy configuration value
 * @param dst Destination value
 * @param src Source value
 * @param type Value type
 */
static void copy_config_value(UQM_ConfigValue* dst, const UQM_ConfigValue* src, UQM_ConfigType type) {
    if (!dst || !src) return;
    
    switch (type) {
        case UQM_CONFIG_TYPE_STRING:
            if (dst->string_val) {
                free(dst->string_val);
                dst->string_val = NULL;
            }
            if (src->string_val) {
                dst->string_val = strdup(src->string_val);
            }
            break;
            
        default:
            *dst = *src;
            break;
    }
}

/**
 * @brief Free configuration value
 * @param value Value to free
 * @param type Value type
 */
static void free_config_value(UQM_ConfigValue* value, UQM_ConfigType type) {
    if (!value) return;
    
    switch (type) {
        case UQM_CONFIG_TYPE_STRING:
            if (value->string_val) {
                free(value->string_val);
                value->string_val = NULL;
            }
            break;
            
        default:
            break;
    }
}

/**
 * @brief Notify callbacks of configuration change
 * @param key Configuration key that changed
 * @param old_value Previous value
 * @param new_value New value
 */
static void notify_config_callbacks(const char* key, const UQM_ConfigValue* old_value, 
                                   const UQM_ConfigValue* new_value) {
    if (!key) return;
    
    for (int i = 0; i < UQM_MAX_CONFIG_CALLBACKS; i++) {
        UQM_ConfigCallbackRegistration* cb = &g_config_system.callbacks[i];
        
        if (!cb->is_active || !cb->callback) continue;
        
        // Check key filter
        if (cb->key_filter && strcmp(cb->key_filter, key) != 0) {
            continue;
        }
        
        // Call the callback
        cb->callback(key, old_value, new_value, cb->user_data);
    }
}

/* ========================================================================== */
/*                          SYSTEM INITIALIZATION                            */
/* ========================================================================== */

UQM_Result(void*, UQM_ConfigError) UQM_Config_Initialize(const char* config_file) {
    if (g_config_system.is_initialized) {
        return UQM_OK_VOID();
    }
    
    // Initialize mutex
    if (pthread_mutex_init(&g_config_system.mutex, NULL) != 0) {
        return UQM_ERROR(UQM_CONFIG_SYSTEM_ERROR);
    }
    
    // Initialize state
    memset(g_config_system.entries, 0, sizeof(g_config_system.entries));
    memset(g_config_system.callbacks, 0, sizeof(g_config_system.callbacks));
    
    atomic_store(&g_config_system.entry_count, 0);
    atomic_store(&g_config_system.callback_count, 0);
    atomic_store(&g_config_system.next_callback_id, 1);
    
    // Set config file path
    if (config_file) {
        g_config_system.config_file_path = strdup(config_file);
    }
    
    g_config_system.is_initialized = true;
    
    UQM_LOG_INFO("Configuration system initialized");
    return UQM_OK_VOID();
}

UQM_Result(void*, UQM_ConfigError) UQM_Config_Shutdown(void) {
    if (!g_config_system.is_initialized) {
        return UQM_OK_VOID();
    }
    
    pthread_mutex_lock(&g_config_system.mutex);
    
    // Free all configuration values
    for (int i = 0; i < UQM_MAX_CONFIG_ENTRIES; i++) {
        if (g_config_system.entries[i].is_active) {
            free_config_value(&g_config_system.entries[i].current_value, 
                             g_config_system.entries[i].definition.type);
        }
    }
    
    // Free callback key filters
    for (int i = 0; i < UQM_MAX_CONFIG_CALLBACKS; i++) {
        if (g_config_system.callbacks[i].key_filter) {
            free(g_config_system.callbacks[i].key_filter);
        }
    }
    
    // Free config file path
    if (g_config_system.config_file_path) {
        free(g_config_system.config_file_path);
        g_config_system.config_file_path = NULL;
    }
    
    g_config_system.is_initialized = false;
    
    pthread_mutex_unlock(&g_config_system.mutex);
    pthread_mutex_destroy(&g_config_system.mutex);
    
    UQM_LOG_INFO("Configuration system shutdown completed");
    return UQM_OK_VOID();
}

/* ========================================================================== */
/*                        CONFIGURATION REGISTRATION                         */
/* ========================================================================== */

UQM_Result(void*, UQM_ConfigError) UQM_Config_Register(const UQM_ConfigEntry* entry) {
    if (!g_config_system.is_initialized) {
        return UQM_ERROR(UQM_CONFIG_NOT_INITIALIZED);
    }
    
    if (!entry || !entry->key) {
        return UQM_ERROR(UQM_CONFIG_INVALID_ARGUMENT);
    }
    
    pthread_mutex_lock(&g_config_system.mutex);
    
    // Check if entry already exists
    if (find_config_entry(entry->key)) {
        pthread_mutex_unlock(&g_config_system.mutex);
        return UQM_ERROR(UQM_CONFIG_ALREADY_EXISTS);
    }
    
    // Find empty slot
    UQM_InternalConfigEntry* slot = find_empty_config_entry();
    if (!slot) {
        pthread_mutex_unlock(&g_config_system.mutex);
        return UQM_ERROR(UQM_CONFIG_TABLE_FULL);
    }
    
    // Copy entry definition
    slot->definition = *entry;
    
    // Set default value
    copy_config_value(&slot->current_value, &entry->default_value, entry->type);
    slot->is_set = false;
    slot->is_active = true;
    
    atomic_fetch_add(&g_config_system.entry_count, 1);
    
    pthread_mutex_unlock(&g_config_system.mutex);
    
    UQM_LOG_DEBUG("Registered configuration entry: %s", entry->key);
    return UQM_OK_VOID();
}

UQM_Result(void*, UQM_ConfigError) UQM_Config_Unregister(const char* key) {
    if (!g_config_system.is_initialized) {
        return UQM_ERROR(UQM_CONFIG_NOT_INITIALIZED);
    }
    
    if (!key) {
        return UQM_ERROR(UQM_CONFIG_INVALID_ARGUMENT);
    }
    
    pthread_mutex_lock(&g_config_system.mutex);
    
    UQM_InternalConfigEntry* entry = find_config_entry(key);
    if (!entry) {
        pthread_mutex_unlock(&g_config_system.mutex);
        return UQM_ERROR(UQM_CONFIG_NOT_FOUND);
    }
    
    // Free value
    free_config_value(&entry->current_value, entry->definition.type);
    
    // Mark as inactive
    entry->is_active = false;
    memset(entry, 0, sizeof(UQM_InternalConfigEntry));
    
    atomic_fetch_sub(&g_config_system.entry_count, 1);
    
    pthread_mutex_unlock(&g_config_system.mutex);
    
    UQM_LOG_DEBUG("Unregistered configuration entry: %s", key);
    return UQM_OK_VOID();
}

/* ========================================================================== */
/*                           VALUE MANIPULATION                              */
/* ========================================================================== */

UQM_Result(bool, UQM_ConfigError) UQM_Config_GetBool(const char* key, bool* value) {
    if (!g_config_system.is_initialized) {
        return UQM_ERROR(UQM_CONFIG_NOT_INITIALIZED);
    }
    
    if (!key || !value) {
        return UQM_ERROR(UQM_CONFIG_INVALID_ARGUMENT);
    }
    
    pthread_mutex_lock(&g_config_system.mutex);
    
    UQM_InternalConfigEntry* entry = find_config_entry(key);
    if (!entry) {
        pthread_mutex_unlock(&g_config_system.mutex);
        return UQM_ERROR(UQM_CONFIG_NOT_FOUND);
    }
    
    if (entry->definition.type != UQM_CONFIG_TYPE_BOOL) {
        pthread_mutex_unlock(&g_config_system.mutex);
        return UQM_ERROR(UQM_CONFIG_TYPE_MISMATCH);
    }
    
    *value = entry->current_value.bool_val;
    
    pthread_mutex_unlock(&g_config_system.mutex);
    return UQM_OK(*value);
}

UQM_Result(void*, UQM_ConfigError) UQM_Config_SetBool(const char* key, bool value) {
    if (!g_config_system.is_initialized) {
        return UQM_ERROR(UQM_CONFIG_NOT_INITIALIZED);
    }
    
    if (!key) {
        return UQM_ERROR(UQM_CONFIG_INVALID_ARGUMENT);
    }
    
    pthread_mutex_lock(&g_config_system.mutex);
    
    UQM_InternalConfigEntry* entry = find_config_entry(key);
    if (!entry) {
        pthread_mutex_unlock(&g_config_system.mutex);
        return UQM_ERROR(UQM_CONFIG_NOT_FOUND);
    }
    
    if (entry->definition.type != UQM_CONFIG_TYPE_BOOL) {
        pthread_mutex_unlock(&g_config_system.mutex);
        return UQM_ERROR(UQM_CONFIG_TYPE_MISMATCH);
    }
    
    UQM_ConfigValue old_value = entry->current_value;
    UQM_ConfigValue new_value;
    new_value.bool_val = value;
    
    if (!validate_config_value(entry, &new_value)) {
        pthread_mutex_unlock(&g_config_system.mutex);
        return UQM_ERROR(UQM_CONFIG_VALIDATION_FAILED);
    }
    
    entry->current_value.bool_val = value;
    entry->is_set = true;
    
    pthread_mutex_unlock(&g_config_system.mutex);
    
    // Notify callbacks
    notify_config_callbacks(key, &old_value, &new_value);
    
    return UQM_OK_VOID();
}

UQM_Result(int32_t, UQM_ConfigError) UQM_Config_GetInt(const char* key, int32_t* value) {
    if (!g_config_system.is_initialized) {
        return UQM_ERROR(UQM_CONFIG_NOT_INITIALIZED);
    }
    
    if (!key || !value) {
        return UQM_ERROR(UQM_CONFIG_INVALID_ARGUMENT);
    }
    
    pthread_mutex_lock(&g_config_system.mutex);
    
    UQM_InternalConfigEntry* entry = find_config_entry(key);
    if (!entry) {
        pthread_mutex_unlock(&g_config_system.mutex);
        return UQM_ERROR(UQM_CONFIG_NOT_FOUND);
    }
    
    if (entry->definition.type != UQM_CONFIG_TYPE_INT) {
        pthread_mutex_unlock(&g_config_system.mutex);
        return UQM_ERROR(UQM_CONFIG_TYPE_MISMATCH);
    }
    
    *value = entry->current_value.int_val;
    
    pthread_mutex_unlock(&g_config_system.mutex);
    return UQM_OK(*value);
}

UQM_Result(void*, UQM_ConfigError) UQM_Config_SetInt(const char* key, int32_t value) {
    if (!g_config_system.is_initialized) {
        return UQM_ERROR(UQM_CONFIG_NOT_INITIALIZED);
    }
    
    if (!key) {
        return UQM_ERROR(UQM_CONFIG_INVALID_ARGUMENT);
    }
    
    pthread_mutex_lock(&g_config_system.mutex);
    
    UQM_InternalConfigEntry* entry = find_config_entry(key);
    if (!entry) {
        pthread_mutex_unlock(&g_config_system.mutex);
        return UQM_ERROR(UQM_CONFIG_NOT_FOUND);
    }
    
    if (entry->definition.type != UQM_CONFIG_TYPE_INT) {
        pthread_mutex_unlock(&g_config_system.mutex);
        return UQM_ERROR(UQM_CONFIG_TYPE_MISMATCH);
    }
    
    UQM_ConfigValue old_value = entry->current_value;
    UQM_ConfigValue new_value;
    new_value.int_val = value;
    
    if (!validate_config_value(entry, &new_value)) {
        pthread_mutex_unlock(&g_config_system.mutex);
        return UQM_ERROR(UQM_CONFIG_VALIDATION_FAILED);
    }
    
    entry->current_value.int_val = value;
    entry->is_set = true;
    
    pthread_mutex_unlock(&g_config_system.mutex);
    
    // Notify callbacks
    notify_config_callbacks(key, &old_value, &new_value);
    
    return UQM_OK_VOID();
}

// Placeholder implementations for remaining functions
// These would be implemented similarly to the above pattern

UQM_Result(double, UQM_ConfigError) UQM_Config_GetFloat(const char* key, double* value) {
    (void)key; (void)value;
    return UQM_ERROR(UQM_CONFIG_NOT_IMPLEMENTED);
}

UQM_Result(void*, UQM_ConfigError) UQM_Config_SetFloat(const char* key, double value) {
    (void)key; (void)value;
    return UQM_ERROR(UQM_CONFIG_NOT_IMPLEMENTED);
}

UQM_Result(size_t, UQM_ConfigError) UQM_Config_GetString(const char* key, char* buffer, size_t buffer_size) {
    (void)key; (void)buffer; (void)buffer_size;
    return UQM_ERROR(UQM_CONFIG_NOT_IMPLEMENTED);
}

UQM_Result(void*, UQM_ConfigError) UQM_Config_SetString(const char* key, const char* value) {
    (void)key; (void)value;
    return UQM_ERROR(UQM_CONFIG_NOT_IMPLEMENTED);
}

UQM_Result(UQM_Resolution, UQM_ConfigError) UQM_Config_GetResolution(const char* key, UQM_Resolution* resolution) {
    (void)key; (void)resolution;
    return UQM_ERROR(UQM_CONFIG_NOT_IMPLEMENTED);
}

UQM_Result(void*, UQM_ConfigError) UQM_Config_SetResolution(const char* key, const UQM_Resolution* resolution) {
    (void)key; (void)resolution;
    return UQM_ERROR(UQM_CONFIG_NOT_IMPLEMENTED);
}

UQM_Result(void*, UQM_ConfigError) UQM_Config_LoadFromFile(const char* filename) {
    (void)filename;
    return UQM_ERROR(UQM_CONFIG_NOT_IMPLEMENTED);
}

UQM_Result(void*, UQM_ConfigError) UQM_Config_SaveToFile(const char* filename) {
    (void)filename;
    return UQM_ERROR(UQM_CONFIG_NOT_IMPLEMENTED);
}

UQM_Result(void*, UQM_ConfigError) UQM_Config_ResetToDefaults(void) {
    return UQM_ERROR(UQM_CONFIG_NOT_IMPLEMENTED);
}

UQM_Result(void*, UQM_ConfigError) UQM_Config_ResetCategoryToDefaults(UQM_ConfigCategory category) {
    (void)category;
    return UQM_ERROR(UQM_CONFIG_NOT_IMPLEMENTED);
}

UQM_Result(uint32_t, UQM_ConfigError) UQM_Config_RegisterCallback(const char* key, UQM_ConfigChangeCallback callback, void* user_data) {
    (void)key; (void)callback; (void)user_data;
    return UQM_ERROR(UQM_CONFIG_NOT_IMPLEMENTED);
}

UQM_Result(void*, UQM_ConfigError) UQM_Config_UnregisterCallback(uint32_t callback_id) {
    (void)callback_id;
    return UQM_ERROR(UQM_CONFIG_NOT_IMPLEMENTED);
}

UQM_Result(size_t, UQM_ConfigError) UQM_Config_GetKeysInCategory(UQM_ConfigCategory category, const char** keys, size_t max_keys) {
    (void)category; (void)keys; (void)max_keys;
    return UQM_ERROR(UQM_CONFIG_NOT_IMPLEMENTED);
}

UQM_Result(UQM_ConfigEntry, UQM_ConfigError) UQM_Config_GetEntryInfo(const char* key, UQM_ConfigEntry* entry) {
    (void)key; (void)entry;
    return UQM_ERROR(UQM_CONFIG_NOT_IMPLEMENTED);
}

bool UQM_Config_KeyExists(const char* key) {
    if (!g_config_system.is_initialized || !key) {
        return false;
    }
    
    pthread_mutex_lock(&g_config_system.mutex);
    bool exists = (find_config_entry(key) != NULL);
    pthread_mutex_unlock(&g_config_system.mutex);
    
    return exists;
}

const char* UQM_Config_GetCategoryName(UQM_ConfigCategory category) {
    static const char* category_names[] = {
        "Graphics",
        "Audio", 
        "Input",
        "Gameplay",
        "Network",
        "System",
        "Debug"
    };
    
    if (category >= 0 && category < UQM_CONFIG_CATEGORY_COUNT) {
        return category_names[category];
    }
    
    return "Unknown";
}

UQM_Result(void*, UQM_ConfigError) UQM_Config_InitializePredefined(void) {
    return UQM_ERROR(UQM_CONFIG_NOT_IMPLEMENTED);
}