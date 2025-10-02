/**
 * @file modern_logger.h
 * @brief Modern, thread-safe logging system for The Ur-Quan Masters
 * 
 * This header provides a comprehensive logging framework following modern C11/C17
 * best practices. It features structured logging, thread safety, performance 
 * optimization, and configurable output formats.
 * 
 * @author The Ur-Quan Masters Development Team
 * @date 2025
 * @version 1.0.0
 * 
 * ## Features
 * - Thread-safe logging with minimal contention
 * - Structured logging with JSON output option
 * - Performance-optimized with lock-free queues
 * - Memory-safe with automatic resource management
 * - Configurable log levels and filtering
 * - Support for custom formatters and outputs
 * 
 * ## Usage Example
 * ```c
 * // Initialize the logger
 * UQM_Logger* logger = uqm_logger_create_default();
 * 
 * // Log structured messages
 * UQM_LOG_INFO(logger, "ship_movement", "Ship %s moved to sector %d,%d", 
 *              ship_name, x, y);
 * UQM_LOG_ERROR(logger, "resource_load", "Failed to load resource: %s", 
 *               resource_path);
 * 
 * // Performance-critical logging (zero-copy)
 * UQM_LOG_DEBUG_FAST(logger, "render_stats", "FPS: %f, Frame time: %fms", 
 *                     fps, frame_time_ms);
 * 
 * // Cleanup
 * uqm_logger_destroy(logger);
 * ```
 * 
 * @copyright Copyright (c) 2025 The Ur-Quan Masters Team
 * @license GNU General Public License v2.0
 */

#ifndef UQM_MODERN_LOGGER_H
#define UQM_MODERN_LOGGER_H

#include <stdint.h>
#include <stdbool.h>
#include <stdarg.h>
#include <stdio.h>
#include <time.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief Maximum length of a single log message
 * @details Defines the buffer size for individual log entries.
 *          Larger messages will be truncated with an indicator.
 */
#define UQM_LOG_MAX_MESSAGE_LENGTH 4096

/**
 * @brief Maximum length of a log context/category name
 * @details Categories help organize and filter log messages by system.
 */
#define UQM_LOG_MAX_CONTEXT_LENGTH 64

/**
 * @brief Default log queue size
 * @details Number of log entries that can be buffered before blocking.
 *          Should be sized based on expected logging volume.
 */
#define UQM_LOG_DEFAULT_QUEUE_SIZE 8192

/**
 * @brief Log severity levels
 * @details Hierarchical levels from most critical to least critical.
 *          Each level includes all more critical levels when filtering.
 */
typedef enum {
    UQM_LOG_LEVEL_FATAL    = 0,  ///< System cannot continue - immediate shutdown
    UQM_LOG_LEVEL_ERROR    = 1,  ///< Error conditions - functionality impaired
    UQM_LOG_LEVEL_WARNING  = 2,  ///< Warning conditions - potential issues
    UQM_LOG_LEVEL_INFO     = 3,  ///< Informational messages - general flow
    UQM_LOG_LEVEL_DEBUG    = 4,  ///< Debug messages - detailed tracing
    UQM_LOG_LEVEL_TRACE    = 5,  ///< Trace messages - very detailed execution flow
    UQM_LOG_LEVEL_COUNT           ///< Number of log levels (not a valid level)
} UQM_LogLevel;

/**
 * @brief Log output format types
 * @details Controls how log messages are formatted when written.
 */
typedef enum {
    UQM_LOG_FORMAT_PLAIN,      ///< Simple human-readable format
    UQM_LOG_FORMAT_TIMESTAMPED,///< Plain format with timestamps
    UQM_LOG_FORMAT_STRUCTURED, ///< Key-value pairs for machine parsing
    UQM_LOG_FORMAT_JSON,       ///< JSON format for log aggregation systems
    UQM_LOG_FORMAT_CUSTOM      ///< User-defined custom formatter
} UQM_LogFormat;

/**
 * @brief Log entry structure
 * @details Contains all information for a single log message.
 *          Designed for efficient memory layout and cache performance.
 */
typedef struct {
    uint64_t        timestamp_ns;      ///< Nanosecond timestamp (monotonic)
    uint32_t        thread_id;         ///< Thread identifier
    UQM_LogLevel    level;             ///< Message severity level
    char            context[UQM_LOG_MAX_CONTEXT_LENGTH]; ///< System/category name
    char            message[UQM_LOG_MAX_MESSAGE_LENGTH]; ///< Formatted log message
    const char*     file;              ///< Source file (compile-time string)
    const char*     function;          ///< Source function (compile-time string)
    uint32_t        line;              ///< Source line number
} UQM_LogEntry;

/**
 * @brief Forward declaration of logger instance
 * @details Opaque structure to hide implementation details and ensure
 *          proper encapsulation of internal state.
 */
typedef struct UQM_Logger UQM_Logger;

/**
 * @brief Custom log formatter function type
 * @param entry Pointer to the log entry to format
 * @param buffer Output buffer for formatted message
 * @param buffer_size Size of the output buffer
 * @return Number of characters written (excluding null terminator)
 * 
 * @details Custom formatters allow complete control over log output format.
 *          They must be thread-safe and should not perform blocking operations.
 */
typedef int (*UQM_LogFormatter)(const UQM_LogEntry* entry, char* buffer, size_t buffer_size);

/**
 * @brief Log output function type
 * @param formatted_message The formatted log message to output
 * @param length Length of the message in bytes
 * @param user_data User-provided context data
 * 
 * @details Output functions handle the actual writing of log messages.
 *          They must be thread-safe and should handle errors gracefully.
 */
typedef void (*UQM_LogOutput)(const char* formatted_message, size_t length, void* user_data);

/**
 * @brief Logger configuration structure
 * @details Contains all configurable options for logger behavior.
 *          Default values provide sensible settings for most use cases.
 */
typedef struct {
    UQM_LogLevel    min_level;          ///< Minimum level to log (filters lower levels)
    UQM_LogFormat   format;             ///< Output format type
    bool            include_location;   ///< Include file:line:function in output
    bool            include_thread_id;  ///< Include thread ID in output
    bool            async_mode;         ///< Use asynchronous logging (better performance)
    size_t          queue_size;         ///< Size of async logging queue
    UQM_LogFormatter custom_formatter;  ///< Custom formatter (if format == CUSTOM)
    UQM_LogOutput   output_func;        ///< Custom output function
    void*           output_user_data;   ///< User data for output function
    FILE*           output_file;        ///< File for output (if output_func is NULL)
} UQM_LoggerConfig;

// ============================================================================
// CORE LOGGER API
// ============================================================================

/**
 * @brief Create a new logger with default configuration
 * @return Pointer to new logger instance, or NULL on failure
 * 
 * @details Creates a logger with sensible defaults:
 *          - Level: INFO and above
 *          - Format: TIMESTAMPED
 *          - Output: stderr
 *          - Async: enabled
 * 
 * @note The caller is responsible for calling uqm_logger_destroy()
 *       to free resources when the logger is no longer needed.
 */
UQM_Logger* uqm_logger_create_default(void);

/**
 * @brief Create a new logger with custom configuration
 * @param config Pointer to configuration structure
 * @return Pointer to new logger instance, or NULL on failure
 * 
 * @details Creates a logger with the specified configuration.
 *          The config structure is copied, so it can be freed after this call.
 * 
 * @note The caller is responsible for calling uqm_logger_destroy()
 *       to free resources when the logger is no longer needed.
 */
UQM_Logger* uqm_logger_create(const UQM_LoggerConfig* config);

/**
 * @brief Destroy a logger and free all associated resources
 * @param logger Pointer to logger instance to destroy
 * 
 * @details Properly shuts down the logger, flushes any pending messages,
 *          and frees all allocated memory. Safe to call with NULL pointer.
 * 
 * @warning Do not use the logger pointer after calling this function.
 */
void uqm_logger_destroy(UQM_Logger* logger);

/**
 * @brief Set the minimum log level for a logger
 * @param logger Pointer to logger instance
 * @param level New minimum level (messages below this level are filtered)
 * @return true on success, false on failure
 * 
 * @details Changes the logging threshold. This setting takes effect immediately
 *          and affects all subsequent log messages.
 */
bool uqm_logger_set_level(UQM_Logger* logger, UQM_LogLevel level);

/**
 * @brief Get the current minimum log level
 * @param logger Pointer to logger instance
 * @return Current minimum log level, or UQM_LOG_LEVEL_COUNT on error
 */
UQM_LogLevel uqm_logger_get_level(const UQM_Logger* logger);

/**
 * @brief Check if a log level is enabled for the logger
 * @param logger Pointer to logger instance
 * @param level Log level to check
 * @return true if the level would be logged, false otherwise
 * 
 * @details Useful for avoiding expensive log message construction
 *          when the message would be filtered anyway.
 */
bool uqm_logger_is_level_enabled(const UQM_Logger* logger, UQM_LogLevel level);

/**
 * @brief Flush any pending log messages
 * @param logger Pointer to logger instance
 * @return true on success, false on failure
 * 
 * @details Forces immediate output of all queued messages.
 *          Useful before shutdown or at critical points.
 */
bool uqm_logger_flush(UQM_Logger* logger);

// ============================================================================
// CORE LOGGING FUNCTIONS
// ============================================================================

/**
 * @brief Log a message with full context information
 * @param logger Pointer to logger instance
 * @param level Severity level of the message
 * @param context System/category context (e.g., "graphics", "audio")
 * @param file Source file name (__FILE__)
 * @param function Source function name (__func__)
 * @param line Source line number (__LINE__)
 * @param format Printf-style format string
 * @param ... Arguments for format string
 * 
 * @details Core logging function that provides complete control over
 *          all aspects of the log message. Other logging functions
 *          are typically convenience wrappers around this one.
 */
void uqm_logger_log(UQM_Logger* logger, 
                    UQM_LogLevel level,
                    const char* context,
                    const char* file,
                    const char* function,
                    uint32_t line,
                    const char* format,
                    ...) __attribute__((format(printf, 7, 8)));

/**
 * @brief Log a message with va_list arguments
 * @param logger Pointer to logger instance
 * @param level Severity level of the message
 * @param context System/category context
 * @param file Source file name
 * @param function Source function name
 * @param line Source line number
 * @param format Printf-style format string
 * @param args Variable argument list
 * 
 * @details Variant of uqm_logger_log that accepts a va_list instead
 *          of variadic arguments. Useful for wrapper functions.
 */
void uqm_logger_log_va(UQM_Logger* logger,
                       UQM_LogLevel level,
                       const char* context,
                       const char* file,
                       const char* function,
                       uint32_t line,
                       const char* format,
                       va_list args);

// ============================================================================
// CONVENIENCE MACROS
// ============================================================================

/**
 * @brief Log a fatal error message
 * @param logger Logger instance
 * @param context Context/category string
 * @param format Printf-style format string
 * @param ... Format arguments
 * 
 * @details Fatal errors indicate the system cannot continue operating.
 *          These messages are always logged regardless of level settings.
 */
#define UQM_LOG_FATAL(logger, context, format, ...) \
    uqm_logger_log((logger), UQM_LOG_LEVEL_FATAL, (context), __FILE__, __func__, __LINE__, (format), ##__VA_ARGS__)

/**
 * @brief Log an error message
 * @param logger Logger instance
 * @param context Context/category string
 * @param format Printf-style format string
 * @param ... Format arguments
 * 
 * @details Error messages indicate functionality is impaired but
 *          the system can continue operating in a degraded state.
 */
#define UQM_LOG_ERROR(logger, context, format, ...) \
    uqm_logger_log((logger), UQM_LOG_LEVEL_ERROR, (context), __FILE__, __func__, __LINE__, (format), ##__VA_ARGS__)

/**
 * @brief Log a warning message
 * @param logger Logger instance
 * @param context Context/category string
 * @param format Printf-style format string
 * @param ... Format arguments
 * 
 * @details Warning messages indicate potential issues that may
 *          cause problems but don't immediately affect functionality.
 */
#define UQM_LOG_WARNING(logger, context, format, ...) \
    uqm_logger_log((logger), UQM_LOG_LEVEL_WARNING, (context), __FILE__, __func__, __LINE__, (format), ##__VA_ARGS__)

/**
 * @brief Log an informational message
 * @param logger Logger instance
 * @param context Context/category string
 * @param format Printf-style format string
 * @param ... Format arguments
 * 
 * @details Informational messages provide general program flow information
 *          useful for understanding system behavior.
 */
#define UQM_LOG_INFO(logger, context, format, ...) \
    uqm_logger_log((logger), UQM_LOG_LEVEL_INFO, (context), __FILE__, __func__, __LINE__, (format), ##__VA_ARGS__)

/**
 * @brief Log a debug message
 * @param logger Logger instance
 * @param context Context/category string
 * @param format Printf-style format string
 * @param ... Format arguments
 * 
 * @details Debug messages provide detailed information useful
 *          for troubleshooting and development.
 */
#define UQM_LOG_DEBUG(logger, context, format, ...) \
    uqm_logger_log((logger), UQM_LOG_LEVEL_DEBUG, (context), __FILE__, __func__, __LINE__, (format), ##__VA_ARGS__)

/**
 * @brief Log a trace message
 * @param logger Logger instance
 * @param context Context/category string
 * @param format Printf-style format string
 * @param ... Format arguments
 * 
 * @details Trace messages provide very detailed execution flow information.
 *          Typically used for performance analysis and deep debugging.
 */
#define UQM_LOG_TRACE(logger, context, format, ...) \
    uqm_logger_log((logger), UQM_LOG_LEVEL_TRACE, (context), __FILE__, __func__, __LINE__, (format), ##__VA_ARGS__)

// ============================================================================
// PERFORMANCE-OPTIMIZED MACROS
// ============================================================================

/**
 * @brief Performance-optimized logging with level check
 * @param logger Logger instance
 * @param level Log level to check
 * @param context Context/category string
 * @param format Printf-style format string
 * @param ... Format arguments
 * 
 * @details Only evaluates format arguments if the log level is enabled.
 *          Use this for expensive logging operations to avoid performance impact.
 */
#define UQM_LOG_IF_ENABLED(logger, level, context, format, ...) \
    do { \
        if (uqm_logger_is_level_enabled((logger), (level))) { \
            uqm_logger_log((logger), (level), (context), __FILE__, __func__, __LINE__, (format), ##__VA_ARGS__); \
        } \
    } while(0)

/**
 * @brief Fast debug logging with automatic level check
 * @param logger Logger instance
 * @param context Context/category string
 * @param format Printf-style format string
 * @param ... Format arguments
 * 
 * @details Optimized version of UQM_LOG_DEBUG that checks level first.
 *          Use in performance-critical code paths.
 */
#define UQM_LOG_DEBUG_FAST(logger, context, format, ...) \
    UQM_LOG_IF_ENABLED((logger), UQM_LOG_LEVEL_DEBUG, (context), (format), ##__VA_ARGS__)

/**
 * @brief Fast trace logging with automatic level check
 * @param logger Logger instance
 * @param context Context/category string
 * @param format Printf-style format string
 * @param ... Format arguments
 * 
 * @details Optimized version of UQM_LOG_TRACE that checks level first.
 *          Use in very performance-critical code paths.
 */
#define UQM_LOG_TRACE_FAST(logger, context, format, ...) \
    UQM_LOG_IF_ENABLED((logger), UQM_LOG_LEVEL_TRACE, (context), (format), ##__VA_ARGS__)

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/**
 * @brief Convert log level enum to human-readable string
 * @param level Log level to convert
 * @return String representation of the level
 * 
 * @details Returns standardized string names for log levels:
 *          FATAL, ERROR, WARNING, INFO, DEBUG, TRACE
 */
const char* uqm_log_level_to_string(UQM_LogLevel level);

/**
 * @brief Parse log level from string
 * @param level_str String representation of log level
 * @return Corresponding log level enum, or UQM_LOG_LEVEL_COUNT on error
 * 
 * @details Accepts case-insensitive string names:
 *          "fatal", "error", "warning", "info", "debug", "trace"
 */
UQM_LogLevel uqm_log_level_from_string(const char* level_str);

/**
 * @brief Get default logger configuration
 * @return Default configuration structure
 * 
 * @details Returns a configuration structure with sensible defaults
 *          that can be modified before passing to uqm_logger_create().
 */
UQM_LoggerConfig uqm_logger_get_default_config(void);

// ============================================================================
// GLOBAL LOGGER CONVENIENCE FUNCTIONS
// ============================================================================

/**
 * @brief Initialize a global logger instance
 * @param config Configuration for the global logger (NULL for defaults)
 * @return true on success, false on failure
 * 
 * @details Sets up a global logger that can be accessed via UQM_LOG_GLOBAL_*
 *          macros. This is convenient for libraries and modules that don't
 *          want to pass logger instances around.
 * 
 * @warning Only call this once during program initialization.
 */
bool uqm_logger_init_global(const UQM_LoggerConfig* config);

/**
 * @brief Shutdown the global logger
 * 
 * @details Properly shuts down and frees the global logger instance.
 *          Call this during program cleanup.
 */
void uqm_logger_shutdown_global(void);

/**
 * @brief Get the global logger instance
 * @return Pointer to global logger, or NULL if not initialized
 * 
 * @details Returns the global logger instance for direct use.
 *          Prefer the UQM_LOG_GLOBAL_* macros for most use cases.
 */
UQM_Logger* uqm_logger_get_global(void);

// Global logger convenience macros
#define UQM_LOG_GLOBAL_FATAL(context, format, ...) \
    UQM_LOG_FATAL(uqm_logger_get_global(), (context), (format), ##__VA_ARGS__)

#define UQM_LOG_GLOBAL_ERROR(context, format, ...) \
    UQM_LOG_ERROR(uqm_logger_get_global(), (context), (format), ##__VA_ARGS__)

#define UQM_LOG_GLOBAL_WARNING(context, format, ...) \
    UQM_LOG_WARNING(uqm_logger_get_global(), (context), (format), ##__VA_ARGS__)

#define UQM_LOG_GLOBAL_INFO(context, format, ...) \
    UQM_LOG_INFO(uqm_logger_get_global(), (context), (format), ##__VA_ARGS__)

#define UQM_LOG_GLOBAL_DEBUG(context, format, ...) \
    UQM_LOG_DEBUG(uqm_logger_get_global(), (context), (format), ##__VA_ARGS__)

#define UQM_LOG_GLOBAL_TRACE(context, format, ...) \
    UQM_LOG_TRACE(uqm_logger_get_global(), (context), (format), ##__VA_ARGS__)

#ifdef __cplusplus
}
#endif

#endif // UQM_MODERN_LOGGER_H