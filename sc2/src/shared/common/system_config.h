/**
 * @file system_config.h
 * @brief Centralized System Configuration for UQM
 * 
 * This header provides centralized configuration management for the entire
 * UQM system, replacing scattered configuration options with a unified,
 * type-safe, and well-documented configuration system.
 * 
 * Features:
 * - Centralized configuration management
 * - Type-safe configuration values
 * - Runtime configuration validation
 * - Configuration persistence and loading
 * - Environment variable support
 * - Default value management
 * - Configuration change notifications
 * 
 * @author The Ur-Quan Masters Refactoring Team
 * @version 2.0.0
 * @date 2025-01-27
 * 
 * @copyright Copyright (c) 2002 Toys for Bob, Inc.
 * @license GNU General Public License v2.0
 */

#ifndef UQM_SYSTEM_CONFIG_H_
#define UQM_SYSTEM_CONFIG_H_

#include "common_types.h"
#include "error_handling.h"
#include <stdbool.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ========================================================================== */
/*                            CONFIGURATION TYPES                            */
/* ========================================================================== */

/**
 * @brief Configuration value types
 */
typedef enum {
    UQM_CONFIG_TYPE_BOOL,
    UQM_CONFIG_TYPE_INT,
    UQM_CONFIG_TYPE_FLOAT,
    UQM_CONFIG_TYPE_STRING,
    UQM_CONFIG_TYPE_ENUM,
    UQM_CONFIG_TYPE_RESOLUTION,
    UQM_CONFIG_TYPE_COLOR
} UQM_ConfigType;

/**
 * @brief Configuration categories for organization
 */
typedef enum {
    UQM_CONFIG_CATEGORY_GRAPHICS,
    UQM_CONFIG_CATEGORY_AUDIO,
    UQM_CONFIG_CATEGORY_INPUT,
    UQM_CONFIG_CATEGORY_GAMEPLAY,
    UQM_CONFIG_CATEGORY_NETWORK,
    UQM_CONFIG_CATEGORY_SYSTEM,
    UQM_CONFIG_CATEGORY_DEBUG,
    UQM_CONFIG_CATEGORY_COUNT
} UQM_ConfigCategory;

/**
 * @brief Configuration value flags
 */
typedef enum UQM_ConfigFlags {
    UQM_CONFIG_FLAG_NONE            = 0,
    UQM_CONFIG_FLAG_READONLY        = (1 << 0), /**< Cannot be modified at runtime */
    UQM_CONFIG_FLAG_REQUIRES_RESTART = (1 << 1), /**< Requires application restart */
    UQM_CONFIG_FLAG_HIDDEN          = (1 << 2), /**< Not shown in UI */
    UQM_CONFIG_FLAG_ADVANCED        = (1 << 3), /**< Advanced option */
    UQM_CONFIG_FLAG_EXPERIMENTAL    = (1 << 4), /**< Experimental feature */
    UQM_CONFIG_FLAG_DEPRECATED      = (1 << 5), /**< Deprecated option */
} UQM_ConfigFlags;

/**
 * @brief Resolution configuration
 */
typedef struct {
    int width;      /**< Screen width */
    int height;     /**< Screen height */
    int refresh;    /**< Refresh rate (0 for default) */
} UQM_Resolution;

/**
 * @brief Color configuration
 */
typedef struct {
    uint8_t r, g, b, a; /**< RGBA color components */
} UQM_Color;

/**
 * @brief Configuration value union
 */
typedef union {
    bool bool_val;              /**< Boolean value */
    int32_t int_val;            /**< Integer value */
    double float_val;           /**< Floating point value */
    char* string_val;           /**< String value */
    int32_t enum_val;           /**< Enumeration value */
    UQM_Resolution resolution;  /**< Resolution value */
    UQM_Color color;            /**< Color value */
} UQM_ConfigValue;

/**
 * @brief Configuration entry validator function
 * @param value Value to validate
 * @param user_data User-provided validation data
 * @return UQM_Result indicating validation success or failure
 */
typedef UQM_Result(bool, UQM_ConfigError) (*UQM_ConfigValidator)(
    const UQM_ConfigValue* value, void* user_data);

/**
 * @brief Configuration change notification callback
 * @param key Configuration key that changed
 * @param old_value Previous value
 * @param new_value New value
 * @param user_data User-provided callback data
 */
typedef void (*UQM_ConfigChangeCallback)(
    const char* key, 
    const UQM_ConfigValue* old_value,
    const UQM_ConfigValue* new_value,
    void* user_data);

/**
 * @brief Configuration entry definition
 */
typedef struct {
    const char* key;                /**< Configuration key */
    const char* name;               /**< Human-readable name */
    const char* description;        /**< Description */
    UQM_ConfigType type;            /**< Value type */
    UQM_ConfigCategory category;    /**< Category */
    UQM_ConfigFlags flags;          /**< Configuration flags */
    
    UQM_ConfigValue default_value;  /**< Default value */
    UQM_ConfigValue min_value;      /**< Minimum value (for numeric types) */
    UQM_ConfigValue max_value;      /**< Maximum value (for numeric types) */
    
    const char** enum_values;       /**< Valid enum values (for enum type) */
    const char* env_var;            /**< Environment variable name */
    
    UQM_ConfigValidator validator;  /**< Custom validator function */
    void* validator_data;           /**< Data for validator */
} UQM_ConfigEntry;

/* ========================================================================== */
/*                           CONFIGURATION SYSTEM                            */
/* ========================================================================== */

/**
 * @brief Initialize the configuration system
 * @param config_file Path to configuration file (NULL for default)
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(void*, UQM_ConfigError) 
UQM_Config_Initialize(const char* config_file);

/**
 * @brief Shutdown the configuration system
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(void*, UQM_ConfigError) 
UQM_Config_Shutdown(void);

/**
 * @brief Register a configuration entry
 * @param entry Configuration entry to register
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(void*, UQM_ConfigError) 
UQM_Config_Register(const UQM_ConfigEntry* entry);

/**
 * @brief Unregister a configuration entry
 * @param key Configuration key to unregister
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(void*, UQM_ConfigError) 
UQM_Config_Unregister(const char* key);

/* ========================================================================== */
/*                           VALUE MANIPULATION                              */
/* ========================================================================== */

/**
 * @brief Get a boolean configuration value
 * @param key Configuration key
 * @param value Pointer to store the value
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(bool, UQM_ConfigError) 
UQM_Config_GetBool(const char* key, bool* value);

/**
 * @brief Set a boolean configuration value
 * @param key Configuration key
 * @param value Value to set
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(void*, UQM_ConfigError) 
UQM_Config_SetBool(const char* key, bool value);

/**
 * @brief Get an integer configuration value
 * @param key Configuration key
 * @param value Pointer to store the value
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(int32_t, UQM_ConfigError) 
UQM_Config_GetInt(const char* key, int32_t* value);

/**
 * @brief Set an integer configuration value
 * @param key Configuration key
 * @param value Value to set
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(void*, UQM_ConfigError) 
UQM_Config_SetInt(const char* key, int32_t value);

/**
 * @brief Get a float configuration value
 * @param key Configuration key
 * @param value Pointer to store the value
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(double, UQM_ConfigError) 
UQM_Config_GetFloat(const char* key, double* value);

/**
 * @brief Set a float configuration value
 * @param key Configuration key
 * @param value Value to set
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(void*, UQM_ConfigError) 
UQM_Config_SetFloat(const char* key, double value);

/**
 * @brief Get a string configuration value
 * @param key Configuration key
 * @param buffer Buffer to store the value
 * @param buffer_size Size of the buffer
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(size_t, UQM_ConfigError) 
UQM_Config_GetString(const char* key, char* buffer, size_t buffer_size);

/**
 * @brief Set a string configuration value
 * @param key Configuration key
 * @param value Value to set
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(void*, UQM_ConfigError) 
UQM_Config_SetString(const char* key, const char* value);

/**
 * @brief Get a resolution configuration value
 * @param key Configuration key
 * @param resolution Pointer to store the resolution
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(UQM_Resolution, UQM_ConfigError) 
UQM_Config_GetResolution(const char* key, UQM_Resolution* resolution);

/**
 * @brief Set a resolution configuration value
 * @param key Configuration key
 * @param resolution Resolution to set
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(void*, UQM_ConfigError) 
UQM_Config_SetResolution(const char* key, const UQM_Resolution* resolution);

/* ========================================================================== */
/*                           BULK OPERATIONS                                 */
/* ========================================================================== */

/**
 * @brief Load configuration from file
 * @param filename Configuration file path
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(void*, UQM_ConfigError) 
UQM_Config_LoadFromFile(const char* filename);

/**
 * @brief Save configuration to file
 * @param filename Configuration file path (NULL for default)
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(void*, UQM_ConfigError) 
UQM_Config_SaveToFile(const char* filename);

/**
 * @brief Reset all configuration values to defaults
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(void*, UQM_ConfigError) 
UQM_Config_ResetToDefaults(void);

/**
 * @brief Reset specific category to defaults
 * @param category Category to reset
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(void*, UQM_ConfigError) 
UQM_Config_ResetCategoryToDefaults(UQM_ConfigCategory category);

/* ========================================================================== */
/*                              NOTIFICATIONS                                */
/* ========================================================================== */

/**
 * @brief Register a configuration change callback
 * @param key Configuration key to watch (NULL for all)
 * @param callback Callback function
 * @param user_data User data for callback
 * @return Callback ID for unregistration
 */
UQM_EXPORT UQM_Result(uint32_t, UQM_ConfigError) 
UQM_Config_RegisterCallback(const char* key, UQM_ConfigChangeCallback callback, void* user_data);

/**
 * @brief Unregister a configuration change callback
 * @param callback_id Callback ID returned by RegisterCallback
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(void*, UQM_ConfigError) 
UQM_Config_UnregisterCallback(uint32_t callback_id);

/* ========================================================================== */
/*                             INTROSPECTION                                 */
/* ========================================================================== */

/**
 * @brief Get list of all configuration keys in a category
 * @param category Category to query
 * @param keys Array to store key pointers
 * @param max_keys Maximum number of keys to store
 * @return Number of keys found
 */
UQM_EXPORT UQM_Result(size_t, UQM_ConfigError) 
UQM_Config_GetKeysInCategory(UQM_ConfigCategory category, const char** keys, size_t max_keys);

/**
 * @brief Get configuration entry information
 * @param key Configuration key
 * @param entry Pointer to store entry information
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(UQM_ConfigEntry, UQM_ConfigError) 
UQM_Config_GetEntryInfo(const char* key, UQM_ConfigEntry* entry);

/**
 * @brief Check if configuration key exists
 * @param key Configuration key
 * @return True if key exists, false otherwise
 */
UQM_EXPORT bool UQM_Config_KeyExists(const char* key);

/**
 * @brief Get human-readable category name
 * @param category Category enum value
 * @return Category name string
 */
UQM_EXPORT const char* UQM_Config_GetCategoryName(UQM_ConfigCategory category);

/* ========================================================================== */
/*                        PREDEFINED CONFIGURATIONS                          */
/* ========================================================================== */

// Graphics Configuration Keys
#define UQM_CONFIG_GFX_RESOLUTION       "graphics.resolution"
#define UQM_CONFIG_GFX_FULLSCREEN       "graphics.fullscreen"
#define UQM_CONFIG_GFX_VSYNC            "graphics.vsync"
#define UQM_CONFIG_GFX_OPENGL           "graphics.opengl"
#define UQM_CONFIG_GFX_SCALER           "graphics.scaler"
#define UQM_CONFIG_GFX_SCANLINES        "graphics.scanlines"
#define UQM_CONFIG_GFX_GAMMA            "graphics.gamma"
#define UQM_CONFIG_GFX_KEEP_ASPECT      "graphics.keep_aspect_ratio"
#define UQM_CONFIG_GFX_SHOW_FPS         "graphics.show_fps"
#define UQM_CONFIG_GFX_SMOOTH_SCROLL    "graphics.smooth_scroll"

// Audio Configuration Keys
#define UQM_CONFIG_AUDIO_DRIVER         "audio.driver"
#define UQM_CONFIG_AUDIO_QUALITY        "audio.quality"
#define UQM_CONFIG_AUDIO_MUSIC_3DO      "audio.music_3do"
#define UQM_CONFIG_AUDIO_MUSIC_REMIX    "audio.music_remix"
#define UQM_CONFIG_AUDIO_SPEECH         "audio.speech"
#define UQM_CONFIG_AUDIO_SUBTITLES      "audio.subtitles"
#define UQM_CONFIG_AUDIO_STEREO_SFX     "audio.stereo_effects"
#define UQM_CONFIG_AUDIO_MUSIC_VOLUME   "audio.music_volume"
#define UQM_CONFIG_AUDIO_SFX_VOLUME     "audio.sfx_volume"
#define UQM_CONFIG_AUDIO_SPEECH_VOLUME  "audio.speech_volume"

// Input Configuration Keys
#define UQM_CONFIG_INPUT_PLAYER1        "input.player1_device"
#define UQM_CONFIG_INPUT_PLAYER2        "input.player2_device"
#define UQM_CONFIG_INPUT_MOUSE_ENABLED  "input.mouse_enabled"
#define UQM_CONFIG_INPUT_JOYSTICK_THRESHOLD "input.joystick_threshold"

// Gameplay Configuration Keys
#define UQM_CONFIG_GAME_DIFFICULTY      "gameplay.difficulty"
#define UQM_CONFIG_GAME_COARSE_SCAN     "gameplay.coarse_scan_type"
#define UQM_CONFIG_GAME_MENU_STYLE      "gameplay.menu_style"
#define UQM_CONFIG_GAME_FONT_STYLE      "gameplay.font_style"
#define UQM_CONFIG_GAME_INTRO_STYLE     "gameplay.intro_style"
#define UQM_CONFIG_GAME_SHIELD_STYLE    "gameplay.shield_style"
#define UQM_CONFIG_GAME_MELEE_SCALE     "gameplay.melee_scale"
#define UQM_CONFIG_GAME_EXTENDED_LORE   "gameplay.extended_lore"

// Network Configuration Keys
#define UQM_CONFIG_NET_ENABLED          "network.enabled"
#define UQM_CONFIG_NET_PORT             "network.port"
#define UQM_CONFIG_NET_TIMEOUT          "network.timeout"
#define UQM_CONFIG_NET_PEER_NAME        "network.peer_name"

// System Configuration Keys
#define UQM_CONFIG_SYS_SAFE_MODE        "system.safe_mode"
#define UQM_CONFIG_SYS_CONFIG_DIR       "system.config_directory"
#define UQM_CONFIG_SYS_CONTENT_DIR      "system.content_directory"
#define UQM_CONFIG_SYS_ADDON_DIR        "system.addon_directory"
#define UQM_CONFIG_SYS_SAVE_DIR         "system.save_directory"
#define UQM_CONFIG_SYS_LOG_LEVEL        "system.log_level"
#define UQM_CONFIG_SYS_LOG_FILE         "system.log_file"

// Debug Configuration Keys
#define UQM_CONFIG_DEBUG_ENABLED        "debug.enabled"
#define UQM_CONFIG_DEBUG_SHOW_COLLISION "debug.show_collision_boxes"
#define UQM_CONFIG_DEBUG_SHOW_PATHS     "debug.show_ai_paths"
#define UQM_CONFIG_DEBUG_MEMORY_CHECKS  "debug.memory_checks"
#define UQM_CONFIG_DEBUG_PERFORMANCE    "debug.performance_monitoring"

/**
 * @brief Initialize all predefined configuration entries
 * @return Result indicating success or failure
 */
UQM_EXPORT UQM_Result(void*, UQM_ConfigError) 
UQM_Config_InitializePredefined(void);

/* ========================================================================== */
/*                              CONVENIENCE MACROS                           */
/* ========================================================================== */

/**
 * @brief Convenience macro to define a boolean configuration entry
 */
#define UQM_CONFIG_DEFINE_BOOL(key_name, display_name, desc, cat, flags_val, default_val) \
    { .key = key_name, .name = display_name, .description = desc, \
      .type = UQM_CONFIG_TYPE_BOOL, .category = cat, .flags = flags_val, \
      .default_value.bool_val = default_val, \
      .min_value.bool_val = false, .max_value.bool_val = true, \
      .enum_values = NULL, .env_var = NULL, \
      .validator = NULL, .validator_data = NULL }

/**
 * @brief Convenience macro to define an integer configuration entry
 */
#define UQM_CONFIG_DEFINE_INT(key_name, display_name, desc, cat, flags_val, default_val, min_val, max_val) \
    { .key = key_name, .name = display_name, .description = desc, \
      .type = UQM_CONFIG_TYPE_INT, .category = cat, .flags = flags_val, \
      .default_value.int_val = default_val, \
      .min_value.int_val = min_val, .max_value.int_val = max_val, \
      .enum_values = NULL, .env_var = NULL, \
      .validator = NULL, .validator_data = NULL }

/**
 * @brief Convenience macro to define a float configuration entry
 */
#define UQM_CONFIG_DEFINE_FLOAT(key_name, display_name, desc, cat, flags_val, default_val, min_val, max_val) \
    { .key = key_name, .name = display_name, .description = desc, \
      .type = UQM_CONFIG_TYPE_FLOAT, .category = cat, .flags = flags_val, \
      .default_value.float_val = default_val, \
      .min_value.float_val = min_val, .max_value.float_val = max_val, \
      .enum_values = NULL, .env_var = NULL, \
      .validator = NULL, .validator_data = NULL }

#ifdef __cplusplus
}
#endif

#endif /* UQM_SYSTEM_CONFIG_H_ */