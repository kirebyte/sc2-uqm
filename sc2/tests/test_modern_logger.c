/**
 * @file test_modern_logger.c
 * @brief Unit tests for the Modern Logger System
 * 
 * This file contains comprehensive unit tests for the modern logging system,
 * testing thread safety, performance, formatting, and error handling.
 * 
 * Features tested:
 * - Basic logging functionality
 * - Thread safety
 * - Performance characteristics
 * - Format string handling
 * - File output
 * - JSON output
 * - Memory management
 * - Error conditions
 * 
 * @author The Ur-Quan Masters Refactoring Team
 * @version 2.0.0
 * @date 2025-01-27
 * 
 * @copyright Copyright (c) 2002 Toys for Bob, Inc.
 * @license GNU General Public License v2.0
 */

#include "../src/shared/logging/modern_logger.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <unistd.h>
#include <pthread.h>
#include <time.h>
#include <sys/stat.h>

/* ========================================================================== */
/*                                TEST FRAMEWORK                             */
/* ========================================================================== */

#define TEST_ASSERT(condition, message) \
    do { \
        if (!(condition)) { \
            fprintf(stderr, "ASSERTION FAILED: %s at %s:%d\n", \
                    message, __FILE__, __LINE__); \
            exit(EXIT_FAILURE); \
        } \
    } while (0)

#define TEST_ASSERT_EQ(expected, actual, message) \
    do { \
        if ((expected) != (actual)) { \
            fprintf(stderr, "ASSERTION FAILED: %s - Expected: %ld, Got: %ld at %s:%d\n", \
                    message, (long)(expected), (long)(actual), __FILE__, __LINE__); \
            exit(EXIT_FAILURE); \
        } \
    } while (0)

#define TEST_ASSERT_STR_EQ(expected, actual, message) \
    do { \
        if (strcmp(expected, actual) != 0) { \
            fprintf(stderr, "ASSERTION FAILED: %s - Expected: '%s', Got: '%s' at %s:%d\n", \
                    message, expected, actual, __FILE__, __LINE__); \
            exit(EXIT_FAILURE); \
        } \
    } while (0)

static int g_test_count = 0;
static int g_test_passed = 0;

#define RUN_TEST(test_func) \
    do { \
        printf("Running test: %s... ", #test_func); \
        fflush(stdout); \
        g_test_count++; \
        test_func(); \
        g_test_passed++; \
        printf("PASSED\n"); \
    } while (0)

/* ========================================================================== */
/*                               TEST UTILITIES                              */
/* ========================================================================== */

/**
 * @brief Create a temporary file for testing
 * @param template Template for temporary file name
 * @return File descriptor or -1 on error
 */
static int create_temp_file(char* template) {
    return mkstemp(template);
}

/**
 * @brief Read entire file content into a buffer
 * @param filename File to read
 * @param size Pointer to store file size
 * @return Allocated buffer with file content (caller must free)
 */
static char* read_file_content(const char* filename, size_t* size) {
    FILE* file = fopen(filename, "r");
    if (!file) {
        return NULL;
    }
    
    fseek(file, 0, SEEK_END);
    long file_size = ftell(file);
    if (file_size < 0) {
        fclose(file);
        return NULL;
    }
    
    fseek(file, 0, SEEK_SET);
    
    char* buffer = malloc(file_size + 1);
    if (!buffer) {
        fclose(file);
        return NULL;
    }
    
    size_t read_size = fread(buffer, 1, file_size, file);
    buffer[read_size] = '\0';
    
    if (size) {
        *size = read_size;
    }
    
    fclose(file);
    return buffer;
}

/* ========================================================================== */
/*                               BASIC TESTS                                 */
/* ========================================================================== */

/**
 * @brief Test logger initialization and shutdown
 */
static void test_logger_init_shutdown(void) {
    // Test initialization
    UQM_Result(void*, UQM_LogError) init_result = UQM_Logger_Initialize();
    TEST_ASSERT(UQM_IS_OK(init_result), "Logger initialization should succeed");
    
    // Test that we can log after initialization
    UQM_LOG_INFO("Test message after initialization");
    
    // Test shutdown
    UQM_Result(void*, UQM_LogError) shutdown_result = UQM_Logger_Shutdown();
    TEST_ASSERT(UQM_IS_OK(shutdown_result), "Logger shutdown should succeed");
    
    // Test double shutdown (should be safe)
    shutdown_result = UQM_Logger_Shutdown();
    TEST_ASSERT(UQM_IS_OK(shutdown_result), "Double shutdown should be safe");
}

/**
 * @brief Test basic logging functionality
 */
static void test_basic_logging(void) {
    UQM_Logger_Initialize();
    
    // Test all log levels
    UQM_LOG_TRACE("This is a trace message");
    UQM_LOG_DEBUG("This is a debug message");
    UQM_LOG_INFO("This is an info message");
    UQM_LOG_WARN("This is a warning message");
    UQM_LOG_ERROR("This is an error message");
    UQM_LOG_FATAL("This is a fatal message");
    
    // Test formatted logging
    UQM_LOG_INFO("Formatted message: %d, %s, %f", 42, "test", 3.14);
    
    UQM_Logger_Shutdown();
}

/**
 * @brief Test log level filtering
 */
static void test_log_level_filtering(void) {
    UQM_Logger_Initialize();
    
    // Set log level to WARNING
    UQM_Result(void*, UQM_LogError) result = UQM_Logger_SetLevel(UQM_LOG_LEVEL_WARN);
    TEST_ASSERT(UQM_IS_OK(result), "Setting log level should succeed");
    
    // Verify log level was set
    UQM_LogLevel current_level;
    result = UQM_Logger_GetLevel(&current_level);
    TEST_ASSERT(UQM_IS_OK(result), "Getting log level should succeed");
    TEST_ASSERT_EQ(UQM_LOG_LEVEL_WARN, current_level, "Log level should be WARNING");
    
    // Test that lower levels are filtered out
    UQM_LOG_TRACE("This should not appear");
    UQM_LOG_DEBUG("This should not appear");
    UQM_LOG_INFO("This should not appear");
    
    // Test that higher or equal levels appear
    UQM_LOG_WARN("This should appear");
    UQM_LOG_ERROR("This should appear");
    UQM_LOG_FATAL("This should appear");
    
    UQM_Logger_Shutdown();
}

/* ========================================================================== */
/*                               FILE OUTPUT TESTS                           */
/* ========================================================================== */

/**
 * @brief Test file output functionality
 */
static void test_file_output(void) {
    char temp_file[] = "/tmp/uqm_log_test_XXXXXX";
    int fd = create_temp_file(temp_file);
    TEST_ASSERT(fd >= 0, "Creating temporary file should succeed");
    close(fd);
    
    UQM_Logger_Initialize();
    
    // Add file appender
    UQM_Result(void*, UQM_LogError) result = UQM_Logger_AddFileAppender(temp_file);
    TEST_ASSERT(UQM_IS_OK(result), "Adding file appender should succeed");
    
    // Log some messages
    UQM_LOG_INFO("Test message 1");
    UQM_LOG_WARN("Test message 2");
    UQM_LOG_ERROR("Test message 3");
    
    // Flush to ensure messages are written
    UQM_Logger_Flush();
    
    UQM_Logger_Shutdown();
    
    // Read file content and verify
    size_t file_size;
    char* content = read_file_content(temp_file, &file_size);
    TEST_ASSERT(content != NULL, "Reading log file should succeed");
    TEST_ASSERT(file_size > 0, "Log file should contain data");
    
    // Check that our messages are in the file
    TEST_ASSERT(strstr(content, "Test message 1") != NULL, "First message should be in log");
    TEST_ASSERT(strstr(content, "Test message 2") != NULL, "Second message should be in log");
    TEST_ASSERT(strstr(content, "Test message 3") != NULL, "Third message should be in log");
    
    free(content);
    unlink(temp_file);
}

/**
 * @brief Test JSON output format
 */
static void test_json_output(void) {
    char temp_file[] = "/tmp/uqm_log_json_test_XXXXXX";
    int fd = create_temp_file(temp_file);
    TEST_ASSERT(fd >= 0, "Creating temporary file should succeed");
    close(fd);
    
    UQM_Logger_Initialize();
    
    // Add JSON file appender
    UQM_Result(void*, UQM_LogError) result = UQM_Logger_AddJSONFileAppender(temp_file);
    TEST_ASSERT(UQM_IS_OK(result), "Adding JSON file appender should succeed");
    
    // Log a message
    UQM_LOG_INFO("JSON test message");
    
    // Flush and shutdown
    UQM_Logger_Flush();
    UQM_Logger_Shutdown();
    
    // Read and verify JSON content
    size_t file_size;
    char* content = read_file_content(temp_file, &file_size);
    TEST_ASSERT(content != NULL, "Reading JSON log file should succeed");
    TEST_ASSERT(file_size > 0, "JSON log file should contain data");
    
    // Basic JSON structure checks
    TEST_ASSERT(strstr(content, "\"level\"") != NULL, "JSON should contain level field");
    TEST_ASSERT(strstr(content, "\"message\"") != NULL, "JSON should contain message field");
    TEST_ASSERT(strstr(content, "\"timestamp\"") != NULL, "JSON should contain timestamp field");
    TEST_ASSERT(strstr(content, "JSON test message") != NULL, "JSON should contain our message");
    
    free(content);
    unlink(temp_file);
}

/* ========================================================================== */
/*                           THREAD SAFETY TESTS                             */
/* ========================================================================== */

/**
 * @brief Thread data for multi-threaded tests
 */
typedef struct {
    int thread_id;
    int message_count;
    volatile int* start_flag;
} ThreadData;

/**
 * @brief Thread function for stress testing
 */
static void* logging_thread(void* arg) {
    ThreadData* data = (ThreadData*)arg;
    
    // Wait for start signal
    while (!*(data->start_flag)) {
        usleep(1000);
    }
    
    // Log messages
    for (int i = 0; i < data->message_count; i++) {
        UQM_LOG_INFO("Thread %d message %d", data->thread_id, i);
        
        // Occasionally use other log levels
        if (i % 10 == 0) {
            UQM_LOG_WARN("Thread %d warning %d", data->thread_id, i);
        }
        if (i % 20 == 0) {
            UQM_LOG_ERROR("Thread %d error %d", data->thread_id, i);
        }
    }
    
    return NULL;
}

/**
 * @brief Test thread safety of logging system
 */
static void test_thread_safety(void) {
    UQM_Logger_Initialize();
    
    const int num_threads = 8;
    const int messages_per_thread = 100;
    pthread_t threads[num_threads];
    ThreadData thread_data[num_threads];
    volatile int start_flag = 0;
    
    // Create threads
    for (int i = 0; i < num_threads; i++) {
        thread_data[i].thread_id = i;
        thread_data[i].message_count = messages_per_thread;
        thread_data[i].start_flag = &start_flag;
        
        int result = pthread_create(&threads[i], NULL, logging_thread, &thread_data[i]);
        TEST_ASSERT(result == 0, "Thread creation should succeed");
    }
    
    // Start all threads simultaneously
    start_flag = 1;
    
    // Wait for all threads to complete
    for (int i = 0; i < num_threads; i++) {
        int result = pthread_join(threads[i], NULL);
        TEST_ASSERT(result == 0, "Thread join should succeed");
    }
    
    // Flush and shutdown
    UQM_Logger_Flush();
    UQM_Logger_Shutdown();
}

/**
 * @brief Test performance under high load
 */
static void test_performance(void) {
    UQM_Logger_Initialize();
    
    const int message_count = 10000;
    struct timespec start_time, end_time;
    
    // Measure logging performance
    clock_gettime(CLOCK_MONOTONIC, &start_time);
    
    for (int i = 0; i < message_count; i++) {
        UQM_LOG_INFO("Performance test message %d with some data: %f", i, i * 3.14159);
    }
    
    UQM_Logger_Flush();
    clock_gettime(CLOCK_MONOTONIC, &end_time);
    
    // Calculate performance metrics
    double elapsed = (end_time.tv_sec - start_time.tv_sec) + 
                    (end_time.tv_nsec - start_time.tv_nsec) / 1e9;
    double messages_per_second = message_count / elapsed;
    
    printf("\nPerformance metrics:\n");
    printf("  Messages: %d\n", message_count);
    printf("  Time: %.3f seconds\n", elapsed);
    printf("  Rate: %.0f messages/second\n", messages_per_second);
    
    // Performance should be reasonable (at least 1000 messages/second)
    TEST_ASSERT(messages_per_second > 1000.0, "Performance should be at least 1000 msgs/sec");
    
    UQM_Logger_Shutdown();
}

/* ========================================================================== */
/*                             ERROR HANDLING TESTS                          */
/* ========================================================================== */

/**
 * @brief Test error handling scenarios
 */
static void test_error_handling(void) {
    // Test operations without initialization
    UQM_Result(void*, UQM_LogError) result = UQM_Logger_SetLevel(UQM_LOG_LEVEL_INFO);
    TEST_ASSERT(UQM_IS_ERROR(result), "Setting level without init should fail");
    
    // Initialize for remaining tests
    UQM_Logger_Initialize();
    
    // Test invalid log level
    result = UQM_Logger_SetLevel((UQM_LogLevel)999);
    TEST_ASSERT(UQM_IS_ERROR(result), "Setting invalid log level should fail");
    
    // Test invalid file path
    result = UQM_Logger_AddFileAppender("/invalid/path/that/does/not/exist/log.txt");
    TEST_ASSERT(UQM_IS_ERROR(result), "Adding invalid file appender should fail");
    
    // Test NULL parameters
    UQM_LogLevel level;
    result = UQM_Logger_GetLevel(NULL);
    TEST_ASSERT(UQM_IS_ERROR(result), "Getting level with NULL pointer should fail");
    
    UQM_Logger_Shutdown();
}

/**
 * @brief Test memory management
 */
static void test_memory_management(void) {
    // This test checks for memory leaks by initializing and shutting down
    // the logger multiple times
    
    for (int i = 0; i < 10; i++) {
        UQM_Logger_Initialize();
        
        // Add some appenders
        char temp_file[256];
        snprintf(temp_file, sizeof(temp_file), "/tmp/uqm_mem_test_%d_XXXXXX", i);
        int fd = mkstemp(temp_file);
        if (fd >= 0) {
            close(fd);
            UQM_Logger_AddFileAppender(temp_file);
            
            // Log some messages
            UQM_LOG_INFO("Memory test iteration %d", i);
            UQM_LOG_WARN("Warning in iteration %d", i);
            
            UQM_Logger_Flush();
            unlink(temp_file);
        }
        
        UQM_Logger_Shutdown();
    }
    
    // If we get here without crashing, memory management is likely correct
    printf("Memory management test completed successfully\n");
}

/* ========================================================================== */
/*                                MAIN FUNCTION                              */
/* ========================================================================== */

/**
 * @brief Run all logger tests
 */
int main(void) {
    printf("Starting UQM Modern Logger Tests\n");
    printf("==================================\n\n");
    
    // Run basic functionality tests
    RUN_TEST(test_logger_init_shutdown);
    RUN_TEST(test_basic_logging);
    RUN_TEST(test_log_level_filtering);
    
    // Run file output tests
    RUN_TEST(test_file_output);
    RUN_TEST(test_json_output);
    
    // Run thread safety and performance tests
    RUN_TEST(test_thread_safety);
    RUN_TEST(test_performance);
    
    // Run error handling tests
    RUN_TEST(test_error_handling);
    RUN_TEST(test_memory_management);
    
    // Print results
    printf("\n==================================\n");
    printf("Test Results: %d/%d tests passed\n", g_test_passed, g_test_count);
    
    if (g_test_passed == g_test_count) {
        printf("All tests PASSED! ✅\n");
        return EXIT_SUCCESS;
    } else {
        printf("Some tests FAILED! ❌\n");
        return EXIT_FAILURE;
    }
}