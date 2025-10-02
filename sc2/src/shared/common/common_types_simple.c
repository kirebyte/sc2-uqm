/*
 * Common Types Implementation
 * Basic implementation of platform detection and utility functions
 * 
 * The Ur-Quan Masters - Refactored
 * Created: December 2024
 */

#include "common_types.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// ============================================================================
// PLATFORM DETECTION IMPLEMENTATION
// ============================================================================

const char* get_platform_name(void) {
    #if defined(UQM_PLATFORM_WINDOWS)
        return "Windows";
    #elif defined(UQM_PLATFORM_MACOS)
        return "macOS";
    #elif defined(UQM_PLATFORM_LINUX)
        return "Linux";
    #elif defined(UQM_PLATFORM_UNIX)
        return "Unix";
    #else
        return "Unknown";
    #endif
}

const char* get_compiler_name(void) {
    #if defined(UQM_COMPILER_GCC)
        return "GCC " __VERSION__;
    #elif defined(UQM_COMPILER_CLANG)
        return "Clang " __VERSION__;
    #elif defined(UQM_COMPILER_MSVC)
        return "MSVC";
    #else
        return "Unknown";
    #endif
}

// ============================================================================
// STRING UTILITIES IMPLEMENTATION  
// ============================================================================

char* str_trim(const char* str) {
    if (!str) return NULL;
    
    // Find start of trimmed string
    while (*str && isspace((unsigned char)*str)) {
        str++;
    }
    
    if (*str == 0) {
        // String is all whitespace
        char* result = malloc(1);
        if (result) result[0] = '\0';
        return result;
    }
    
    // Find end of trimmed string
    const char* end = str + strlen(str) - 1;
    while (end > str && isspace((unsigned char)*end)) {
        end--;
    }
    
    // Copy trimmed string
    size_t len = end - str + 1;
    char* result = malloc(len + 1);
    if (result) {
        memcpy(result, str, len);
        result[len] = '\0';
    }
    
    return result;
}

bool str_safe_copy(char* dest, size_t dest_size, const char* src) {
    if (!dest || !src || dest_size == 0) return false;
    
    size_t src_len = strlen(src);
    if (src_len >= dest_size) {
        // Source too long, truncate
        memcpy(dest, src, dest_size - 1);
        dest[dest_size - 1] = '\0';
        return false;
    }
    
    // Safe to copy
    strcpy(dest, src);
    return true;
}

// ============================================================================
// TIME UTILITIES IMPLEMENTATION
// ============================================================================

uint64_t get_timestamp_ms(void) {
    #if defined(UQM_PLATFORM_WINDOWS)
        // Windows implementation would go here
        return (uint64_t)time(NULL) * 1000;
    #else
        // Unix-like systems
        struct timespec ts;
        if (clock_gettime(CLOCK_MONOTONIC, &ts) == 0) {
            return (uint64_t)ts.tv_sec * 1000 + (uint64_t)ts.tv_nsec / 1000000;
        }
        return (uint64_t)time(NULL) * 1000;
    #endif
}

// ============================================================================
// MEMORY UTILITIES IMPLEMENTATION
// ============================================================================

void* safe_malloc(size_t size) {
    if (size == 0) return NULL;
    
    void* ptr = malloc(size);
    if (ptr) {
        memset(ptr, 0, size);
    }
    return ptr;
}

void* safe_realloc(void* ptr, size_t new_size) {
    if (new_size == 0) {
        free(ptr);
        return NULL;
    }
    
    return realloc(ptr, new_size);
}

void safe_free(void** ptr) {
    if (ptr && *ptr) {
        free(*ptr);
        *ptr = NULL;
    }
}

// ============================================================================
// MATH UTILITIES IMPLEMENTATION
// ============================================================================

UQM_f32 uqm_lerp(UQM_f32 a, UQM_f32 b, UQM_f32 t) {
    return a + t * (b - a);
}

UQM_f32 uqm_clamp(UQM_f32 value, UQM_f32 min, UQM_f32 max) {
    if (value < min) return min;
    if (value > max) return max;
    return value;
}

// ============================================================================
// DEBUG UTILITIES IMPLEMENTATION
// ============================================================================

void uqm_debug_print(const char* format, ...) {
    #ifdef UQM_DEBUG_BUILD
        va_list args;
        va_start(args, format);
        printf("[DEBUG] ");
        vprintf(format, args);
        printf("\n");
        va_end(args);
    #else
        (void)format; // Suppress unused parameter warning
    #endif
}

// ============================================================================
// SYSTEM INFO IMPLEMENTATION
// ============================================================================

void print_system_info(void) {
    printf("System Information:\n");
    printf("  Platform: %s\n", get_platform_name());
    printf("  Compiler: %s\n", get_compiler_name());
    
    #ifdef UQM_BIG_ENDIAN
    printf("  Endianness: Big Endian\n");
    #else
    printf("  Endianness: Little Endian\n");
    #endif
    
    printf("  Pointer size: %zu bytes\n", sizeof(void*));
    printf("  Build type: ");
    #ifdef UQM_DEBUG_BUILD
    printf("Debug\n");
    #else
    printf("Release\n");
    #endif
    
    printf("  Timestamp: %lu ms\n", get_timestamp_ms());
}