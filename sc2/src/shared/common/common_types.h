/**
 * @file common_types.h
 * @brief Common types, constants, and utilities for The Ur-Quan Masters
 * 
 * This header provides standardized types, constants, and utility macros
 * used throughout the codebase. It ensures consistency, improves readability,
 * and provides modern C11/C17 features in a portable way.
 * 
 * @author The Ur-Quan Masters Development Team
 * @date 2025
 * @version 1.0.0
 * 
 * ## Features
 * - Platform-independent fixed-width integer types
 * - Mathematical constants and utility functions
 * - String handling utilities with safety guarantees
 * - Time and timestamp handling
 * - Compiler-specific optimizations and attributes
 * - Debug and release build conditionals
 * 
 * @copyright Copyright (c) 2025 The Ur-Quan Masters Team
 * @license GNU General Public License v2.0
 */

#ifndef UQM_COMMON_TYPES_H
#define UQM_COMMON_TYPES_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#include <limits.h>
#include <float.h>
#include <assert.h>

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// COMPILER AND PLATFORM DETECTION
// ============================================================================

// Compiler detection
#if defined(__GNUC__)
    #define UQM_COMPILER_GCC 1
    #define UQM_COMPILER_VERSION (__GNUC__ * 10000 + __GNUC_MINOR__ * 100 + __GNUC_PATCHLEVEL__)
#elif defined(__clang__)
    #define UQM_COMPILER_CLANG 1
    #define UQM_COMPILER_VERSION (__clang_major__ * 10000 + __clang_minor__ * 100 + __clang_patchlevel__)
#elif defined(_MSC_VER)
    #define UQM_COMPILER_MSVC 1
    #define UQM_COMPILER_VERSION _MSC_VER
#else
    #define UQM_COMPILER_UNKNOWN 1
    #define UQM_COMPILER_VERSION 0
#endif

// Platform detection
#if defined(_WIN32) || defined(_WIN64)
    #define UQM_PLATFORM_WINDOWS 1
#elif defined(__linux__)
    #define UQM_PLATFORM_LINUX 1
#elif defined(__APPLE__)
    #define UQM_PLATFORM_MACOS 1
#elif defined(__unix__)
    #define UQM_PLATFORM_UNIX 1
#else
    #define UQM_PLATFORM_UNKNOWN 1
#endif

// Architecture detection
#if defined(_M_X64) || defined(__x86_64__)
    #define UQM_ARCH_X64 1
    #define UQM_ARCH_64BIT 1
#elif defined(_M_IX86) || defined(__i386__)
    #define UQM_ARCH_X86 1
    #define UQM_ARCH_32BIT 1
#elif defined(__aarch64__)
    #define UQM_ARCH_ARM64 1
    #define UQM_ARCH_64BIT 1
#elif defined(__arm__)
    #define UQM_ARCH_ARM 1
    #define UQM_ARCH_32BIT 1
#else
    #define UQM_ARCH_UNKNOWN 1
#endif

// ============================================================================
// COMPILER ATTRIBUTES AND OPTIMIZATIONS
// ============================================================================

// Function attributes
#if defined(UQM_COMPILER_GCC) || defined(UQM_COMPILER_CLANG)
    #define UQM_INLINE        inline __attribute__((always_inline))
    #define UQM_NOINLINE      __attribute__((noinline))
    #define UQM_PURE          __attribute__((pure))
    #define UQM_CONST         __attribute__((const))
    #define UQM_HOT           __attribute__((hot))
    #define UQM_COLD          __attribute__((cold))
    #define UQM_LIKELY(x)     __builtin_expect(!!(x), 1)
    #define UQM_UNLIKELY(x)   __builtin_expect(!!(x), 0)
    #define UQM_RESTRICT      __restrict__
    #define UQM_NORETURN      __attribute__((noreturn))
    #define UQM_DEPRECATED    __attribute__((deprecated))
    #define UQM_UNUSED        __attribute__((unused))
    #define UQM_PACKED        __attribute__((packed))
    #define UQM_ALIGNED(n)    __attribute__((aligned(n)))
    #define UQM_PRINTF_FORMAT(fmt_idx, arg_idx) __attribute__((format(printf, fmt_idx, arg_idx)))
#elif defined(UQM_COMPILER_MSVC)
    #define UQM_INLINE        __forceinline
    #define UQM_NOINLINE      __declspec(noinline)
    #define UQM_PURE          
    #define UQM_CONST         
    #define UQM_HOT           
    #define UQM_COLD          
    #define UQM_LIKELY(x)     (x)
    #define UQM_UNLIKELY(x)   (x)
    #define UQM_RESTRICT      __restrict
    #define UQM_NORETURN      __declspec(noreturn)
    #define UQM_DEPRECATED    __declspec(deprecated)
    #define UQM_UNUSED        
    #define UQM_PACKED        __pragma(pack(push, 1))
    #define UQM_ALIGNED(n)    __declspec(align(n))
    #define UQM_PRINTF_FORMAT(fmt_idx, arg_idx)
#else
    #define UQM_INLINE        inline
    #define UQM_NOINLINE      
    #define UQM_PURE          
    #define UQM_CONST         
    #define UQM_HOT           
    #define UQM_COLD          
    #define UQM_LIKELY(x)     (x)
    #define UQM_UNLIKELY(x)   (x)
    #define UQM_RESTRICT      
    #define UQM_NORETURN      
    #define UQM_DEPRECATED    
    #define UQM_UNUSED        
    #define UQM_PACKED        
    #define UQM_ALIGNED(n)    
    #define UQM_PRINTF_FORMAT(fmt_idx, arg_idx)
#endif

// Memory barriers and atomics
#if defined(UQM_COMPILER_GCC) || defined(UQM_COMPILER_CLANG)
    #define UQM_MEMORY_BARRIER()  __sync_synchronize()
    #define UQM_COMPILER_BARRIER() __asm__ __volatile__("" ::: "memory")
#elif defined(UQM_COMPILER_MSVC)
    #include <intrin.h>
    #define UQM_MEMORY_BARRIER()  MemoryBarrier()
    #define UQM_COMPILER_BARRIER() _ReadWriteBarrier()
#else
    #define UQM_MEMORY_BARRIER()  
    #define UQM_COMPILER_BARRIER() 
#endif

// ============================================================================
// DEBUG AND RELEASE BUILD CONFIGURATION
// ============================================================================

// Debug build detection
#if !defined(NDEBUG) || defined(DEBUG) || defined(_DEBUG)
    #define UQM_DEBUG_BUILD 1
    #define UQM_RELEASE_BUILD 0
#else
    #define UQM_DEBUG_BUILD 0
    #define UQM_RELEASE_BUILD 1
#endif

// Debug-only code
#if UQM_DEBUG_BUILD
    #define UQM_DEBUG_ONLY(code) do { code } while(0)
    #define UQM_RELEASE_ONLY(code)
#else
    #define UQM_DEBUG_ONLY(code)
    #define UQM_RELEASE_ONLY(code) do { code } while(0)
#endif

// Assertions
#if UQM_DEBUG_BUILD
    #define UQM_ASSERT(condition) assert(condition)
    #define UQM_ASSERT_MSG(condition, message) \
        do { \
            if (UQM_UNLIKELY(!(condition))) { \
                fprintf(stderr, "Assertion failed: %s at %s:%d: %s\\n", \
                        #condition, __FILE__, __LINE__, (message)); \
                abort(); \
            } \
        } while(0)
#else
    #define UQM_ASSERT(condition) ((void)0)
    #define UQM_ASSERT_MSG(condition, message) ((void)0)
#endif

// Static assertions (compile-time)
#if defined(__STDC_VERSION__) && __STDC_VERSION__ >= 201112L
    #define UQM_STATIC_ASSERT(condition, message) _Static_assert(condition, message)
#elif defined(UQM_COMPILER_GCC) || defined(UQM_COMPILER_CLANG)
    #define UQM_STATIC_ASSERT(condition, message) \
        _Static_assert(condition, message)
#else
    #define UQM_STATIC_ASSERT(condition, message) \
        typedef char UQM_STATIC_ASSERT_[(condition) ? 1 : -1] UQM_UNUSED
#endif

// ============================================================================
// BASIC TYPE DEFINITIONS
// ============================================================================

// Ensure we have fixed-width integer types
UQM_STATIC_ASSERT(sizeof(int8_t) == 1, "int8_t must be 1 byte");
UQM_STATIC_ASSERT(sizeof(int16_t) == 2, "int16_t must be 2 bytes");
UQM_STATIC_ASSERT(sizeof(int32_t) == 4, "int32_t must be 4 bytes");
UQM_STATIC_ASSERT(sizeof(int64_t) == 8, "int64_t must be 8 bytes");

// Convenient type aliases
typedef uint8_t   UQM_u8;    ///< Unsigned 8-bit integer
typedef uint16_t  UQM_u16;   ///< Unsigned 16-bit integer
typedef uint32_t  UQM_u32;   ///< Unsigned 32-bit integer
typedef uint64_t  UQM_u64;   ///< Unsigned 64-bit integer
typedef int8_t    UQM_i8;    ///< Signed 8-bit integer
typedef int16_t   UQM_i16;   ///< Signed 16-bit integer
typedef int32_t   UQM_i32;   ///< Signed 32-bit integer
typedef int64_t   UQM_i64;   ///< Signed 64-bit integer

typedef float     UQM_f32;   ///< 32-bit floating point
typedef double    UQM_f64;   ///< 64-bit floating point

typedef size_t    UQM_size;  ///< Size type for memory and arrays
typedef ptrdiff_t UQM_ssize; ///< Signed size type

// Boolean type (for older C standards)
#if !defined(__STDC_VERSION__) || __STDC_VERSION__ < 199901L
    #ifndef bool
        typedef int bool;
        #define true 1
        #define false 0
    #endif
#endif

// Character and string types
typedef char      UQM_char;   ///< Standard character
typedef UQM_u8    UQM_byte;   ///< Raw byte data
typedef UQM_u32   UQM_rune;   ///< Unicode code point (UTF-32)

// Handle types (opaque pointers)
typedef void*     UQM_Handle; ///< Generic opaque handle

// ============================================================================
// MATHEMATICAL CONSTANTS AND UTILITIES
// ============================================================================

// Mathematical constants
#ifndef M_PI
    #define UQM_PI           3.14159265358979323846
    #define UQM_PI_2         1.57079632679489661923  // PI/2
    #define UQM_PI_4         0.78539816339744830962  // PI/4
    #define UQM_1_PI         0.31830988618379067154  // 1/PI
    #define UQM_2_PI         0.63661977236758134308  // 2/PI
    #define UQM_TAU          6.28318530717958647692  // 2*PI
#else
    #define UQM_PI           M_PI
    #define UQM_PI_2         M_PI_2
    #define UQM_PI_4         M_PI_4
    #define UQM_1_PI         M_1_PI
    #define UQM_2_PI         M_2_PI
    #define UQM_TAU          (2.0 * M_PI)
#endif

#define UQM_E                2.71828182845904523536  // Euler's number
#define UQM_LOG2E            1.44269504088896340736  // log2(e)
#define UQM_LOG10E           0.43429448190325182765  // log10(e)
#define UQM_LN2              0.69314718055994530942  // ln(2)
#define UQM_LN10             2.30258509299404568402  // ln(10)
#define UQM_SQRT2            1.41421356237309504880  // sqrt(2)
#define UQM_SQRT1_2          0.70710678118654752440  // 1/sqrt(2)

// Numeric limits for floating point
#define UQM_F32_EPSILON      FLT_EPSILON
#define UQM_F32_MIN          FLT_MIN
#define UQM_F32_MAX          FLT_MAX
#define UQM_F64_EPSILON      DBL_EPSILON
#define UQM_F64_MIN          DBL_MIN
#define UQM_F64_MAX          DBL_MAX

// Utility macros
#define UQM_MIN(a, b)        ((a) < (b) ? (a) : (b))
#define UQM_MAX(a, b)        ((a) > (b) ? (a) : (b))
#define UQM_CLAMP(x, min, max) UQM_MIN(UQM_MAX((x), (min)), (max))
#define UQM_ABS(x)           ((x) < 0 ? -(x) : (x))
#define UQM_SIGN(x)          ((x) > 0 ? 1 : ((x) < 0 ? -1 : 0))

// Bit manipulation
#define UQM_BIT(n)           (1U << (n))
#define UQM_SET_BIT(x, n)    ((x) |= UQM_BIT(n))
#define UQM_CLEAR_BIT(x, n)  ((x) &= ~UQM_BIT(n))
#define UQM_TOGGLE_BIT(x, n) ((x) ^= UQM_BIT(n))
#define UQM_TEST_BIT(x, n)   (((x) & UQM_BIT(n)) != 0)

// Alignment utilities
#define UQM_ALIGN_UP(x, align)   (((x) + (align) - 1) & ~((align) - 1))
#define UQM_ALIGN_DOWN(x, align) ((x) & ~((align) - 1))
#define UQM_IS_ALIGNED(x, align) (((x) & ((align) - 1)) == 0)
#define UQM_IS_POWER_OF_2(x)     (((x) != 0) && (((x) & ((x) - 1)) == 0))

// ============================================================================
// ARRAY AND CONTAINER UTILITIES
// ============================================================================

// Array size calculation
#define UQM_ARRAY_SIZE(arr) (sizeof(arr) / sizeof((arr)[0]))

// Safe array iteration
#define UQM_FOR_EACH(type, var, array) \
    for (type* var = (array); var < (array) + UQM_ARRAY_SIZE(array); ++var)

// Container offset calculation
#define UQM_OFFSET_OF(type, member) ((size_t)&((type*)0)->member)

// Container from member pointer
#define UQM_CONTAINER_OF(ptr, type, member) \
    ((type*)((char*)(ptr) - UQM_OFFSET_OF(type, member)))

// ============================================================================
// STRING AND MEMORY UTILITIES
// ============================================================================

// String length limits
#define UQM_STRING_MAX_LENGTH        4096   ///< Maximum general string length
#define UQM_PATH_MAX_LENGTH          1024   ///< Maximum file path length
#define UQM_NAME_MAX_LENGTH          256    ///< Maximum name/identifier length

// Safe string operations
#define UQM_SAFE_STRCPY(dest, src) \
    do { \
        strncpy((dest), (src), sizeof(dest) - 1); \
        (dest)[sizeof(dest) - 1] = '\\0'; \
    } while(0)

#define UQM_SAFE_STRCAT(dest, src) \
    do { \
        size_t dest_len = strlen(dest); \
        size_t dest_size = sizeof(dest); \
        if (dest_len < dest_size - 1) { \
            strncpy((dest) + dest_len, (src), dest_size - dest_len - 1); \
            (dest)[dest_size - 1] = '\\0'; \
        } \
    } while(0)

// String comparison macros
#define UQM_STR_EQUAL(a, b)         (strcmp((a), (b)) == 0)
#define UQM_STR_IEQUAL(a, b)        (strcasecmp((a), (b)) == 0)
#define UQM_STR_STARTS_WITH(str, prefix) (strncmp((str), (prefix), strlen(prefix)) == 0)

// Memory operations
#define UQM_ZERO_MEMORY(ptr, size)  memset((ptr), 0, (size))
#define UQM_ZERO_STRUCT(ptr)        UQM_ZERO_MEMORY((ptr), sizeof(*(ptr)))
#define UQM_COPY_MEMORY(dest, src, size) memcpy((dest), (src), (size))
#define UQM_MOVE_MEMORY(dest, src, size) memmove((dest), (src), (size))

// ============================================================================
// TIME AND TIMESTAMP UTILITIES
// ============================================================================

// Time units in various scales
#define UQM_NANOSECONDS_PER_MICROSECOND  1000ULL
#define UQM_MICROSECONDS_PER_MILLISECOND 1000ULL
#define UQM_MILLISECONDS_PER_SECOND      1000ULL
#define UQM_SECONDS_PER_MINUTE           60ULL
#define UQM_MINUTES_PER_HOUR             60ULL
#define UQM_HOURS_PER_DAY                24ULL

#define UQM_NANOSECONDS_PER_MILLISECOND  (UQM_NANOSECONDS_PER_MICROSECOND * UQM_MICROSECONDS_PER_MILLISECOND)
#define UQM_NANOSECONDS_PER_SECOND       (UQM_NANOSECONDS_PER_MILLISECOND * UQM_MILLISECONDS_PER_SECOND)
#define UQM_MICROSECONDS_PER_SECOND      (UQM_MICROSECONDS_PER_MILLISECOND * UQM_MILLISECONDS_PER_SECOND)
#define UQM_SECONDS_PER_HOUR             (UQM_SECONDS_PER_MINUTE * UQM_MINUTES_PER_HOUR)
#define UQM_SECONDS_PER_DAY              (UQM_SECONDS_PER_HOUR * UQM_HOURS_PER_DAY)

// Time conversion macros
#define UQM_MS_TO_NS(ms)   ((ms) * UQM_NANOSECONDS_PER_MILLISECOND)
#define UQM_US_TO_NS(us)   ((us) * UQM_NANOSECONDS_PER_MICROSECOND)
#define UQM_S_TO_NS(s)     ((s) * UQM_NANOSECONDS_PER_SECOND)
#define UQM_NS_TO_US(ns)   ((ns) / UQM_NANOSECONDS_PER_MICROSECOND)
#define UQM_NS_TO_MS(ns)   ((ns) / UQM_NANOSECONDS_PER_MILLISECOND)
#define UQM_NS_TO_S(ns)    ((ns) / UQM_NANOSECONDS_PER_SECOND)

// Timestamp type
typedef UQM_u64 UQM_Timestamp; ///< Nanoseconds since epoch or boot

// ============================================================================
// COORDINATE AND GEOMETRY TYPES
// ============================================================================

/**
 * @brief 2D point with integer coordinates
 */
typedef struct {
    UQM_i32 x, y;
} UQM_Point2i;

/**
 * @brief 2D point with floating-point coordinates
 */
typedef struct {
    UQM_f32 x, y;
} UQM_Point2f;

/**
 * @brief 2D vector with integer components
 */
typedef struct {
    UQM_i32 x, y;
} UQM_Vector2i;

/**
 * @brief 2D vector with floating-point components
 */
typedef struct {
    UQM_f32 x, y;
} UQM_Vector2f;

/**
 * @brief 2D size with integer dimensions
 */
typedef struct {
    UQM_u32 width, height;
} UQM_Size2i;

/**
 * @brief 2D size with floating-point dimensions
 */
typedef struct {
    UQM_f32 width, height;
} UQM_Size2f;

/**
 * @brief 2D rectangle with integer coordinates
 */
typedef struct {
    UQM_i32 x, y;
    UQM_u32 width, height;
} UQM_Rect2i;

/**
 * @brief 2D rectangle with floating-point coordinates
 */
typedef struct {
    UQM_f32 x, y;
    UQM_f32 width, height;
} UQM_Rect2f;

/**
 * @brief RGBA color with 8-bit components
 */
typedef struct {
    UQM_u8 r, g, b, a;
} UQM_Color8;

/**
 * @brief RGBA color with floating-point components (0.0-1.0)
 */
typedef struct {
    UQM_f32 r, g, b, a;
} UQM_Colorf;

// Color constants
#define UQM_COLOR_TRANSPARENT  ((UQM_Color8){0, 0, 0, 0})
#define UQM_COLOR_BLACK        ((UQM_Color8){0, 0, 0, 255})
#define UQM_COLOR_WHITE        ((UQM_Color8){255, 255, 255, 255})
#define UQM_COLOR_RED          ((UQM_Color8){255, 0, 0, 255})
#define UQM_COLOR_GREEN        ((UQM_Color8){0, 255, 0, 255})
#define UQM_COLOR_BLUE         ((UQM_Color8){0, 0, 255, 255})

// ============================================================================
// FUNCTION POINTER TYPES
// ============================================================================

/**
 * @brief Generic callback function type
 * @param user_data User-provided context data
 * @return Application-specific return value
 */
typedef int (*UQM_Callback)(void* user_data);

/**
 * @brief Comparison function for sorting
 * @param a First element
 * @param b Second element
 * @return < 0 if a < b, 0 if a == b, > 0 if a > b
 */
typedef int (*UQM_CompareFunc)(const void* a, const void* b);

/**
 * @brief Hash function for hash tables
 * @param data Data to hash
 * @param size Size of data in bytes
 * @return Hash value
 */
typedef UQM_u32 (*UQM_HashFunc)(const void* data, UQM_size size);

/**
 * @brief Iterator function for collections
 * @param item Current item
 * @param user_data User-provided context
 * @return true to continue iteration, false to stop
 */
typedef bool (*UQM_IteratorFunc)(void* item, void* user_data);

/**
 * @brief Resource cleanup function
 * @param resource Resource to clean up
 */
typedef void (*UQM_CleanupFunc)(void* resource);

// ============================================================================
// UTILITY MACROS FOR COMMON PATTERNS
// ============================================================================

// Swap two values
#define UQM_SWAP(type, a, b) \
    do { \
        type temp = (a); \
        (a) = (b); \
        (b) = temp; \
    } while(0)

// Check if a value is in a range (inclusive)
#define UQM_IN_RANGE(x, min, max) ((x) >= (min) && (x) <= (max))

// Convert degrees to radians and vice versa
#define UQM_DEG_TO_RAD(deg) ((deg) * UQM_PI / 180.0)
#define UQM_RAD_TO_DEG(rad) ((rad) * 180.0 / UQM_PI)

// String literal helpers
#define UQM_STRINGIFY(x) #x
#define UQM_STRINGIFY_EXPAND(x) UQM_STRINGIFY(x)

// Version helpers
#ifndef UQM_VERSION_MAKE
#define UQM_VERSION_MAKE(major, minor, patch) \
    (((major) << 16) | ((minor) << 8) | (patch))
#endif
#ifndef UQM_VERSION_MAJOR
#define UQM_VERSION_MAJOR(version) (((version) >> 16) & 0xFF)
#endif
#ifndef UQM_VERSION_MINOR
#define UQM_VERSION_MINOR(version) (((version) >> 8) & 0xFF)
#endif
#ifndef UQM_VERSION_PATCH
#define UQM_VERSION_PATCH(version) ((version) & 0xFF)
#endif

// Defer pattern for C (poor man's RAII)
#define UQM_DEFER(cleanup_code) \
    __attribute__((cleanup(cleanup_wrapper))) bool UQM_UNUSED cleanup_guard = true; \
    void cleanup_wrapper(bool* guard) { \
        if (*guard) { cleanup_code; } \
    }

// ============================================================================
// VALIDATION AND COMPILE-TIME CHECKS
// ============================================================================

// Ensure basic assumptions about types are correct
UQM_STATIC_ASSERT(sizeof(UQM_u8) == 1, "UQM_u8 must be 1 byte");
UQM_STATIC_ASSERT(sizeof(UQM_u16) == 2, "UQM_u16 must be 2 bytes");
UQM_STATIC_ASSERT(sizeof(UQM_u32) == 4, "UQM_u32 must be 4 bytes");
UQM_STATIC_ASSERT(sizeof(UQM_u64) == 8, "UQM_u64 must be 8 bytes");
UQM_STATIC_ASSERT(sizeof(UQM_f32) == 4, "UQM_f32 must be 4 bytes");
UQM_STATIC_ASSERT(sizeof(UQM_f64) == 8, "UQM_f64 must be 8 bytes");

// Ensure pointer size consistency
UQM_STATIC_ASSERT(sizeof(void*) == sizeof(UQM_size), "Pointer size must match UQM_size");

// Ensure color structures are packed correctly
UQM_STATIC_ASSERT(sizeof(UQM_Color8) == 4, "UQM_Color8 must be 4 bytes");
UQM_STATIC_ASSERT(sizeof(UQM_Colorf) == 16, "UQM_Colorf must be 16 bytes");

// ============================================================================
// FUNCTION DECLARATIONS
// ============================================================================

// Platform detection
const char* get_platform_name(void);
const char* get_compiler_name(void);

// String utilities
char* str_trim(const char* str);
bool str_safe_copy(char* dest, size_t dest_size, const char* src);

// Time utilities
uint64_t get_timestamp_ms(void);

// Memory utilities
void* safe_malloc(size_t size);
void* safe_realloc(void* ptr, size_t new_size);
void safe_free(void** ptr);

// Math utilities
UQM_f32 uqm_lerp(UQM_f32 a, UQM_f32 b, UQM_f32 t);
UQM_f32 uqm_clamp(UQM_f32 value, UQM_f32 min, UQM_f32 max);

// Debug utilities
void uqm_debug_print(const char* format, ...);

#ifdef __cplusplus
}
#endif

#endif // UQM_COMMON_TYPES_H