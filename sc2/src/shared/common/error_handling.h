/**
 * @file error_handling.h
 * @brief Modern error handling system for The Ur-Quan Masters
 * 
 * This header provides a comprehensive error handling framework implementing
 * Result<T, E> patterns, error propagation, and structured error reporting
 * for C. It follows functional programming best practices adapted for C11/C17.
 * 
 * @author The Ur-Quan Masters Development Team
 * @date 2025
 * @version 1.0.0
 * 
 * ## Features
 * - Result<T, E> pattern for explicit error handling
 * - Error code standardization and categorization
 * - Automatic error propagation with context preservation
 * - Thread-safe error reporting and logging integration
 * - Stack trace collection for debugging
 * - Error recovery and fallback mechanisms
 * 
 * ## Usage Example
 * ```c
 * // Function that can fail
 * UQM_Result(FileHandle*) load_game_file(const char* filename) {
 *     if (!filename) {
 *         return UQM_ERR(UQM_ERROR_INVALID_ARGUMENT, "Filename cannot be NULL");
 *     }
 *     
 *     FileHandle* handle = fopen(filename, "rb");
 *     if (!handle) {
 *         return UQM_ERR(UQM_ERROR_FILE_NOT_FOUND, "Could not open file: %s", filename);
 *     }
 *     
 *     return UQM_OK(handle);
 * }
 * 
 * // Using the result
 * UQM_Result(FileHandle*) result = load_game_file("save.dat");
 * if (UQM_IS_OK(result)) {
 *     FileHandle* file = UQM_UNWRAP(result);
 *     // Use file...
 *     fclose(file);
 * } else {
 *     UQM_Error error = UQM_GET_ERROR(result);
 *     printf("Error: %s\\n", error.message);
 * }
 * ```
 * 
 * @copyright Copyright (c) 2025 The Ur-Quan Masters Team
 * @license GNU General Public License v2.0
 */

#ifndef UQM_ERROR_HANDLING_H
#define UQM_ERROR_HANDLING_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdarg.h>

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// CONFIGURATION AND CONSTANTS
// ============================================================================

/**
 * @brief Maximum length of an error message
 * @details Includes space for formatting and context information.
 *          Messages longer than this will be truncated.
 */
#define UQM_ERROR_MESSAGE_MAX_LENGTH 512

/**
 * @brief Maximum length of error context strings
 * @details Context provides additional information about where
 *          and why an error occurred.
 */
#define UQM_ERROR_CONTEXT_MAX_LENGTH 128

/**
 * @brief Maximum number of stack frames to capture
 * @details Stack traces help identify the call chain that led
 *          to an error. Only available in debug builds.
 */
#define UQM_ERROR_MAX_STACK_FRAMES 32

/**
 * @brief Maximum length of a stack frame description
 * @details Each frame includes function name, file, and line number.
 */
#define UQM_ERROR_STACK_FRAME_MAX_LENGTH 256

// ============================================================================
// ERROR CODES AND CATEGORIES
// ============================================================================

/**
 * @brief Standard error codes used throughout the system
 * @details Hierarchical organization allows for both specific error
 *          identification and category-based handling.
 */
typedef enum {
    // Success (not an error)
    UQM_ERROR_SUCCESS = 0,
    
    // General errors (1-99)
    UQM_ERROR_UNKNOWN = 1,           ///< Unknown or unspecified error
    UQM_ERROR_NOT_IMPLEMENTED,       ///< Functionality not yet implemented
    UQM_ERROR_INVALID_STATE,         ///< Object is in an invalid state
    UQM_ERROR_OPERATION_FAILED,      ///< General operation failure
    UQM_ERROR_TIMEOUT,               ///< Operation timed out
    UQM_ERROR_CANCELLED,             ///< Operation was cancelled
    UQM_ERROR_BUSY,                  ///< Resource is currently busy
    UQM_ERROR_ALREADY_EXISTS,        ///< Resource already exists
    UQM_ERROR_NOT_FOUND,             ///< Resource not found
    UQM_ERROR_ACCESS_DENIED,         ///< Access to resource denied
    
    // Argument and validation errors (100-199)
    UQM_ERROR_INVALID_ARGUMENT = 100, ///< Invalid function argument
    UQM_ERROR_NULL_POINTER,           ///< Unexpected NULL pointer
    UQM_ERROR_OUT_OF_RANGE,           ///< Value is out of valid range
    UQM_ERROR_INVALID_FORMAT,         ///< Invalid data format
    UQM_ERROR_INVALID_SIZE,           ///< Invalid size parameter
    UQM_ERROR_BUFFER_TOO_SMALL,       ///< Provided buffer is too small
    UQM_ERROR_STRING_TOO_LONG,        ///< String exceeds maximum length
    UQM_ERROR_VALIDATION_FAILED,      ///< Data validation failed
    
    // Memory errors (200-299)
    UQM_ERROR_OUT_OF_MEMORY = 200,    ///< Memory allocation failed
    UQM_ERROR_MEMORY_LEAK,            ///< Memory leak detected
    UQM_ERROR_DOUBLE_FREE,            ///< Attempt to free already freed memory
    UQM_ERROR_BUFFER_OVERFLOW,        ///< Buffer overflow detected
    UQM_ERROR_BUFFER_UNDERFLOW,       ///< Buffer underflow detected
    UQM_ERROR_MEMORY_CORRUPTION,      ///< Memory corruption detected
    UQM_ERROR_ALIGNMENT_ERROR,        ///< Memory alignment requirements not met
    
    // File system errors (300-399)
    UQM_ERROR_FILE_NOT_FOUND = 300,   ///< File does not exist
    UQM_ERROR_FILE_ACCESS_DENIED,     ///< Permission denied for file operation
    UQM_ERROR_FILE_ALREADY_EXISTS,    ///< File already exists
    UQM_ERROR_FILE_TOO_LARGE,         ///< File size exceeds limits
    UQM_ERROR_DISK_FULL,              ///< Insufficient disk space
    UQM_ERROR_IO_ERROR,               ///< General I/O error
    UQM_ERROR_FILE_CORRUPTED,         ///< File data is corrupted
    UQM_ERROR_DIRECTORY_NOT_EMPTY,    ///< Directory is not empty
    UQM_ERROR_PATH_TOO_LONG,          ///< File path exceeds system limits
    UQM_ERROR_INVALID_PATH,           ///< Invalid file path format
    
    // Network errors (400-499)
    UQM_ERROR_NETWORK_UNAVAILABLE = 400, ///< Network is not available
    UQM_ERROR_CONNECTION_FAILED,         ///< Failed to establish connection
    UQM_ERROR_CONNECTION_LOST,           ///< Connection was lost
    UQM_ERROR_CONNECTION_REFUSED,        ///< Connection was refused
    UQM_ERROR_HOST_NOT_FOUND,            ///< Host could not be resolved
    UQM_ERROR_PROTOCOL_ERROR,            ///< Network protocol error
    UQM_ERROR_NETWORK_TIMEOUT,           ///< Network operation timed out
    
    // Graphics errors (500-599)
    UQM_ERROR_GRAPHICS_INIT_FAILED = 500, ///< Graphics initialization failed
    UQM_ERROR_SHADER_COMPILE_FAILED,      ///< Shader compilation error
    UQM_ERROR_TEXTURE_LOAD_FAILED,        ///< Texture loading failed
    UQM_ERROR_RENDER_TARGET_FAILED,       ///< Render target creation failed
    UQM_ERROR_GRAPHICS_DRIVER_ERROR,      ///< Graphics driver error
    UQM_ERROR_UNSUPPORTED_FORMAT,         ///< Unsupported graphics format
    
    // Audio errors (600-699)
    UQM_ERROR_AUDIO_INIT_FAILED = 600,    ///< Audio initialization failed
    UQM_ERROR_AUDIO_DEVICE_NOT_FOUND,     ///< Audio device not available
    UQM_ERROR_AUDIO_FORMAT_UNSUPPORTED,   ///< Unsupported audio format
    UQM_ERROR_AUDIO_BUFFER_UNDERRUN,      ///< Audio buffer underrun
    UQM_ERROR_SOUND_LOAD_FAILED,          ///< Sound file loading failed
    
    // Game logic errors (700-799)
    UQM_ERROR_SAVE_GAME_CORRUPT = 700,    ///< Save game file is corrupted
    UQM_ERROR_SAVE_GAME_VERSION,          ///< Incompatible save game version
    UQM_ERROR_GAME_STATE_INVALID,         ///< Invalid game state
    UQM_ERROR_RESOURCE_NOT_LOADED,        ///< Required resource not loaded
    UQM_ERROR_LEVEL_LOAD_FAILED,          ///< Game level loading failed
    
    // Thread and synchronization errors (800-899)
    UQM_ERROR_THREAD_CREATE_FAILED = 800, ///< Thread creation failed
    UQM_ERROR_MUTEX_LOCK_FAILED,          ///< Mutex lock operation failed
    UQM_ERROR_CONDITION_WAIT_FAILED,      ///< Condition variable wait failed
    UQM_ERROR_DEADLOCK_DETECTED,          ///< Potential deadlock detected
    UQM_ERROR_RACE_CONDITION,             ///< Race condition detected
    
    // Configuration and initialization errors (900-999)
    UQM_ERROR_CONFIG_PARSE_FAILED = 900,  ///< Configuration parsing failed
    UQM_ERROR_INIT_FAILED,                ///< System initialization failed
    UQM_ERROR_SHUTDOWN_FAILED,            ///< System shutdown failed
    UQM_ERROR_DEPENDENCY_MISSING,         ///< Required dependency missing
    UQM_ERROR_VERSION_MISMATCH,           ///< Version compatibility issue
    
    UQM_ERROR_COUNT                       ///< Total number of error codes
} UQM_ErrorCode;

/**
 * @brief Error severity levels
 * @details Indicates the severity and required response to an error.
 */
typedef enum {
    UQM_ERROR_SEVERITY_INFO,       ///< Informational - no action required
    UQM_ERROR_SEVERITY_WARNING,    ///< Warning - may cause issues
    UQM_ERROR_SEVERITY_ERROR,      ///< Error - functionality impaired
    UQM_ERROR_SEVERITY_CRITICAL,   ///< Critical - system stability at risk
    UQM_ERROR_SEVERITY_FATAL       ///< Fatal - immediate shutdown required
} UQM_ErrorSeverity;

/**
 * @brief Error categories for grouping related errors
 * @details Allows handling of error groups without checking specific codes.
 */
typedef enum {
    UQM_ERROR_CATEGORY_SUCCESS,        ///< Success (not an error)
    UQM_ERROR_CATEGORY_GENERAL,        ///< General purpose errors
    UQM_ERROR_CATEGORY_ARGUMENT,       ///< Argument validation errors
    UQM_ERROR_CATEGORY_MEMORY,         ///< Memory management errors
    UQM_ERROR_CATEGORY_FILESYSTEM,     ///< File system operation errors
    UQM_ERROR_CATEGORY_NETWORK,        ///< Network operation errors
    UQM_ERROR_CATEGORY_GRAPHICS,       ///< Graphics system errors
    UQM_ERROR_CATEGORY_AUDIO,          ///< Audio system errors
    UQM_ERROR_CATEGORY_GAME_LOGIC,     ///< Game logic errors
    UQM_ERROR_CATEGORY_THREADING,      ///< Threading and synchronization errors
    UQM_ERROR_CATEGORY_CONFIGURATION   ///< Configuration and initialization errors
} UQM_ErrorCategory;

// ============================================================================
// ERROR STRUCTURES
// ============================================================================

/**
 * @brief Stack frame information for error tracing
 * @details Contains information about a single function call in the stack.
 */
typedef struct {
    const char* function;                                    ///< Function name
    const char* file;                                        ///< Source file name
    uint32_t line;                                           ///< Line number
    char description[UQM_ERROR_STACK_FRAME_MAX_LENGTH];      ///< Formatted description
} UQM_StackFrame;

/**
 * @brief Complete error information structure
 * @details Contains all information about an error, including context,
 *          stack trace, and metadata for debugging and recovery.
 */
typedef struct {
    UQM_ErrorCode code;                                      ///< Specific error code
    UQM_ErrorCategory category;                              ///< Error category
    UQM_ErrorSeverity severity;                              ///< Error severity level
    char message[UQM_ERROR_MESSAGE_MAX_LENGTH];              ///< Human-readable message
    char context[UQM_ERROR_CONTEXT_MAX_LENGTH];              ///< Additional context
    
    // Location information
    const char* file;                                        ///< Source file where error occurred
    const char* function;                                    ///< Function where error occurred
    uint32_t line;                                           ///< Line number where error occurred
    
    // Timing and threading
    uint64_t timestamp;                                      ///< When the error occurred
    uint32_t thread_id;                                      ///< Thread where error occurred
    
    // Stack trace (debug builds only)
    UQM_StackFrame stack_trace[UQM_ERROR_MAX_STACK_FRAMES]; ///< Call stack
    uint32_t stack_depth;                                    ///< Number of valid frames
    
    // Chaining for error propagation
    struct UQM_Error* inner_error;                          ///< Underlying cause (if any)
} UQM_Error;

// ============================================================================
// RESULT TYPE SYSTEM
// ============================================================================

/**
 * @brief Result type for functions that can fail
 * @details Generic container that holds either a success value or an error.
 *          Inspired by Rust's Result<T, E> but adapted for C.
 */
#define UQM_Result(T) struct { \
    bool is_ok; \
    union { \
        T value; \
        UQM_Error error; \
    } data; \
}

/**
 * @brief Create a successful result
 * @param value Success value to wrap
 * @return Result containing the success value
 */
#define UQM_OK(value) { \
    .is_ok = true, \
    .data = { .value = (value) } \
}

/**
 * @brief Create an error result with formatted message
 * @param code Error code
 * @param format Printf-style format string
 * @param ... Format arguments
 * @return Result containing the error
 */
#define UQM_ERR(code, format, ...) \
    uqm_error_create_result((code), __FILE__, __func__, __LINE__, (format), ##__VA_ARGS__)

/**
 * @brief Create an error result from an existing error
 * @param error Existing error to wrap
 * @return Result containing the error
 */
#define UQM_ERR_FROM(error) { \
    .is_ok = false, \
    .data = { .error = (error) } \
}

/**
 * @brief Check if a result is successful
 * @param result Result to check
 * @return true if result contains a success value
 */
#define UQM_IS_OK(result) ((result).is_ok)

/**
 * @brief Check if a result is an error
 * @param result Result to check
 * @return true if result contains an error
 */
#define UQM_IS_ERR(result) (!(result).is_ok)

/**
 * @brief Extract the success value from a result
 * @param result Result to extract from
 * @return The success value
 * @warning Only call this if UQM_IS_OK(result) is true
 */
#define UQM_UNWRAP(result) ((result).data.value)

/**
 * @brief Extract the error from a result
 * @param result Result to extract from
 * @return The error
 * @warning Only call this if UQM_IS_ERR(result) is true
 */
#define UQM_GET_ERROR(result) ((result).data.error)

/**
 * @brief Extract value with default fallback
 * @param result Result to extract from
 * @param default_value Value to return if result is an error
 * @return The success value or the default value
 */
#define UQM_UNWRAP_OR(result, default_value) \
    (UQM_IS_OK(result) ? UQM_UNWRAP(result) : (default_value))

/**
 * @brief Propagate error if result is an error
 * @param result Result to check
 * @details If result is an error, returns from the current function
 *          with that error. Otherwise, continues execution.
 */
#define UQM_TRY(result) \
    do { \
        if (UQM_IS_ERR(result)) { \
            return UQM_ERR_FROM(UQM_GET_ERROR(result)); \
        } \
    } while(0)

/**
 * @brief Extract value or propagate error
 * @param result Result to extract from
 * @details If result is successful, evaluates to the success value.
 *          If result is an error, returns from the current function
 *          with that error.
 */
#define UQM_TRY_UNWRAP(result) \
    ({ \
        typeof(result) _r = (result); \
        if (UQM_IS_ERR(_r)) { \
            return UQM_ERR_FROM(UQM_GET_ERROR(_r)); \
        } \
        UQM_UNWRAP(_r); \
    })

// ============================================================================
// CORE ERROR HANDLING API
// ============================================================================

/**
 * @brief Initialize the error handling system
 * @return true on success, false on failure
 * 
 * @details Sets up global error handling state, initializes stack tracing
 *          (if available), and configures default error handlers.
 */
bool uqm_error_init(void);

/**
 * @brief Shutdown the error handling system
 * 
 * @details Cleans up global state and flushes any pending error reports.
 *          Should be called during program shutdown.
 */
void uqm_error_shutdown(void);

/**
 * @brief Create a new error with detailed information
 * @param code Error code
 * @param severity Error severity level
 * @param file Source file name (__FILE__)
 * @param function Source function name (__func__)
 * @param line Source line number (__LINE__)
 * @param format Printf-style format string for message
 * @param ... Format arguments
 * @return Initialized error structure
 * 
 * @details Creates a complete error with all metadata, including optional
 *          stack trace in debug builds.
 */
UQM_Error uqm_error_create(UQM_ErrorCode code, UQM_ErrorSeverity severity,
                           const char* file, const char* function, uint32_t line,
                           const char* format, ...) __attribute__((format(printf, 6, 7)));

/**
 * @brief Create an error result (internal implementation for UQM_ERR macro)
 * @param code Error code
 * @param file Source file name
 * @param function Source function name
 * @param line Source line number
 * @param format Printf-style format string
 * @param ... Format arguments
 * @return Error result structure
 */
#define UQM_Result_Error struct { bool is_ok; UQM_Error data; }
UQM_Result_Error uqm_error_create_result(UQM_ErrorCode code, const char* file,
                                         const char* function, uint32_t line,
                                         const char* format, ...)
                                         __attribute__((format(printf, 5, 6)));

/**
 * @brief Create an error with va_list arguments
 * @param code Error code
 * @param severity Error severity level
 * @param file Source file name
 * @param function Source function name
 * @param line Source line number
 * @param format Printf-style format string
 * @param args Variable argument list
 * @return Initialized error structure
 */
UQM_Error uqm_error_create_va(UQM_ErrorCode code, UQM_ErrorSeverity severity,
                              const char* file, const char* function, uint32_t line,
                              const char* format, va_list args);

/**
 * @brief Wrap an existing error with additional context
 * @param inner_error The underlying error to wrap
 * @param code New error code (or use inner error's code if UQM_ERROR_SUCCESS)
 * @param file Source file name
 * @param function Source function name
 * @param line Source line number
 * @param format Printf-style format string for additional context
 * @param ... Format arguments
 * @return New error with wrapped context
 * 
 * @details Creates a new error that includes the original error as context.
 *          Useful for adding higher-level context to lower-level errors.
 */
UQM_Error uqm_error_wrap(const UQM_Error* inner_error, UQM_ErrorCode code,
                         const char* file, const char* function, uint32_t line,
                         const char* format, ...) __attribute__((format(printf, 6, 7)));

/**
 * @brief Create a simple error with automatic severity detection
 * @param code Error code
 * @param message Error message
 * @return Initialized error structure
 * 
 * @details Convenience function that automatically determines severity
 *          based on the error code and doesn't capture location information.
 */
UQM_Error uqm_error_simple(UQM_ErrorCode code, const char* message);

// ============================================================================
// ERROR INFORMATION AND UTILITIES
// ============================================================================

/**
 * @brief Get the category for an error code
 * @param code Error code
 * @return Error category
 * 
 * @details Maps error codes to their logical categories for
 *          category-based error handling.
 */
UQM_ErrorCategory uqm_error_get_category(UQM_ErrorCode code);

/**
 * @brief Get the default severity for an error code
 * @param code Error code
 * @return Default severity level
 * 
 * @details Provides reasonable default severity levels that can
 *          be overridden when creating specific error instances.
 */
UQM_ErrorSeverity uqm_error_get_default_severity(UQM_ErrorCode code);

/**
 * @brief Get a human-readable description of an error code
 * @param code Error code
 * @return Static string describing the error code
 * 
 * @details Returns a brief, generic description of what the error code
 *          represents. Does not include specific instance information.
 */
const char* uqm_error_code_to_string(UQM_ErrorCode code);

/**
 * @brief Get a human-readable name for an error category
 * @param category Error category
 * @return Static string naming the category
 */
const char* uqm_error_category_to_string(UQM_ErrorCategory category);

/**
 * @brief Get a human-readable name for a severity level
 * @param severity Severity level
 * @return Static string naming the severity
 */
const char* uqm_error_severity_to_string(UQM_ErrorSeverity severity);

/**
 * @brief Check if an error code belongs to a specific category
 * @param code Error code to check
 * @param category Category to test against
 * @return true if the error code belongs to the category
 */
bool uqm_error_is_category(UQM_ErrorCode code, UQM_ErrorCategory category);

/**
 * @brief Check if an error meets a minimum severity threshold
 * @param error Error to check
 * @param min_severity Minimum severity level
 * @return true if error severity >= min_severity
 */
bool uqm_error_is_severity_at_least(const UQM_Error* error, UQM_ErrorSeverity min_severity);

// ============================================================================
// ERROR REPORTING AND LOGGING
// ============================================================================

/**
 * @brief Report an error to the logging system
 * @param error Error to report
 * @param context Additional context for the report
 * 
 * @details Sends the error to the configured logging system with
 *          appropriate log level based on error severity.
 */
void uqm_error_report(const UQM_Error* error, const char* context);

/**
 * @brief Print a detailed error report to a file
 * @param error Error to print
 * @param file File to write to (or NULL for stderr)
 * 
 * @details Outputs a comprehensive error report including all metadata,
 *          stack trace (if available), and any chained errors.
 */
void uqm_error_print_detailed(const UQM_Error* error, FILE* file);

/**
 * @brief Print a brief error summary
 * @param error Error to print
 * @param file File to write to (or NULL for stderr)
 * 
 * @details Outputs a concise error summary suitable for user-facing
 *          error messages or log entries.
 */
void uqm_error_print_summary(const UQM_Error* error, FILE* file);

/**
 * @brief Format an error as a string
 * @param error Error to format
 * @param buffer Output buffer
 * @param buffer_size Size of output buffer
 * @param include_details Whether to include detailed information
 * @return Number of characters written (excluding null terminator)
 * 
 * @details Formats the error into a string buffer. The include_details
 *          parameter controls whether to include location, stack trace,
 *          and other debugging information.
 */
int uqm_error_format(const UQM_Error* error, char* buffer, size_t buffer_size,
                     bool include_details);

// ============================================================================
// ERROR RECOVERY AND HANDLING
// ============================================================================

/**
 * @brief Error handler function type
 * @param error The error that occurred
 * @param context User-provided context data
 * @return true to continue execution, false to abort
 * 
 * @details Error handlers are called when errors are reported and can
 *          implement custom recovery logic, logging, or user notification.
 */
typedef bool (*UQM_ErrorHandler)(const UQM_Error* error, void* context);

/**
 * @brief Register a global error handler
 * @param handler Handler function to register
 * @param context Context data to pass to handler
 * @return true on success, false on failure
 * 
 * @details Registers a handler that will be called for all reported errors.
 *          Multiple handlers can be registered and will be called in order.
 */
bool uqm_error_register_handler(UQM_ErrorHandler handler, void* context);

/**
 * @brief Unregister a global error handler
 * @param handler Handler function to unregister
 * @return true if handler was found and removed, false otherwise
 */
bool uqm_error_unregister_handler(UQM_ErrorHandler handler);

/**
 * @brief Set a fatal error handler
 * @param handler Handler for fatal errors (or NULL for default)
 * @param context Context data for the handler
 * 
 * @details Fatal error handlers are called for errors with FATAL severity.
 *          The default handler terminates the program after logging.
 */
void uqm_error_set_fatal_handler(UQM_ErrorHandler handler, void* context);

// ============================================================================
// CONVENIENCE MACROS FOR COMMON PATTERNS
// ============================================================================

/**
 * @brief Create a fatal error and terminate
 * @param code Error code
 * @param format Printf-style format string
 * @param ... Format arguments
 * 
 * @details Creates a fatal error, reports it, and terminates the program.
 *          Should only be used for unrecoverable errors.
 */
#define UQM_FATAL(code, format, ...) \
    do { \
        UQM_Error _fatal_error = uqm_error_create((code), UQM_ERROR_SEVERITY_FATAL, \
                                                  __FILE__, __func__, __LINE__, \
                                                  (format), ##__VA_ARGS__); \
        uqm_error_report(&_fatal_error, "FATAL ERROR"); \
        abort(); \
    } while(0)

/**
 * @brief Assert a condition or create an error result
 * @param condition Condition to check
 * @param code Error code if condition is false
 * @param format Printf-style format string
 * @param ... Format arguments
 * @return Error result if condition is false, otherwise continues
 */
#define UQM_ENSURE(condition, code, format, ...) \
    do { \
        if (!(condition)) { \
            return UQM_ERR((code), (format), ##__VA_ARGS__); \
        } \
    } while(0)

/**
 * @brief Check for NULL pointer and return error if found
 * @param ptr Pointer to check
 * @param name Name of the pointer for error message
 */
#define UQM_CHECK_NULL(ptr, name) \
    UQM_ENSURE((ptr) != NULL, UQM_ERROR_NULL_POINTER, #name " cannot be NULL")

/**
 * @brief Validate an argument and return error if invalid
 * @param condition Validation condition
 * @param message Error message if validation fails
 */
#define UQM_VALIDATE_ARG(condition, message) \
    UQM_ENSURE((condition), UQM_ERROR_INVALID_ARGUMENT, (message))

/**
 * @brief Check array bounds and return error if out of range
 * @param index Index to check
 * @param size Array size
 * @param name Name of the array for error message
 */
#define UQM_CHECK_BOUNDS(index, size, name) \
    UQM_ENSURE((index) < (size), UQM_ERROR_OUT_OF_RANGE, \
               #name " index %zu is out of range (size: %zu)", (size_t)(index), (size_t)(size))

#ifdef __cplusplus
}
#endif

#endif // UQM_ERROR_HANDLING_H