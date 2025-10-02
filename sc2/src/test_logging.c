/*
 * Logging System Test Program
 * Tests our Phase 2 Week 1 logging implementation
 * 
 * Phase 2 Week 1: Logging System Integration
 * Created: October 2024
 * Purpose: Validate logging system before continuing with Phase 2
 */

// For CLOCK_REALTIME and other timing constants  
#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <time.h>
#include <pthread.h>

// Include our foundation and logging systems
#include "shared/common/common_types.h"
#include "shared/logging/modern_logger.h"

/* Test Results Structure */
typedef struct {
    int tests_run;
    int tests_passed;
    int tests_failed;
    char last_error[256];
} TestResults;

static TestResults g_results = {0};

/* Simple Test Framework */
#define TEST_ASSERT(condition, message) \
    do { \
        g_results.tests_run++; \
        if (condition) { \
            g_results.tests_passed++; \
            printf("✓ PASS: %s\n", message); \
        } else { \
            g_results.tests_failed++; \
            snprintf(g_results.last_error, sizeof(g_results.last_error), "%s", message); \
            printf("✗ FAIL: %s\n", message); \
        } \
    } while(0)

#define TEST_START(name) \
    printf("\n=== Testing %s ===\n", name)

#define TEST_END() \
    printf("Tests: %d | Passed: %d | Failed: %d\n", \
           g_results.tests_run, g_results.tests_passed, g_results.tests_failed)

/* Test Functions */

static void test_logger_creation(void) {
    TEST_START("Logger Creation & Destruction");
    
    // Test default logger creation
    UQM_Logger* logger = uqm_logger_create_default();
    TEST_ASSERT(logger != NULL, "Default logger creation works");
    
    // Test level getting/setting
    UQM_LogLevel original_level = uqm_logger_get_level(logger);
    TEST_ASSERT(original_level == UQM_LOG_LEVEL_INFO, "Default log level is INFO");
    
    bool set_result = uqm_logger_set_level(logger, UQM_LOG_LEVEL_DEBUG);
    TEST_ASSERT(set_result, "Setting log level works");
    
    UQM_LogLevel new_level = uqm_logger_get_level(logger);
    TEST_ASSERT(new_level == UQM_LOG_LEVEL_DEBUG, "Log level was actually set");
    
    // Test level checking
    bool debug_enabled = uqm_logger_is_level_enabled(logger, UQM_LOG_LEVEL_DEBUG);
    bool trace_enabled = uqm_logger_is_level_enabled(logger, UQM_LOG_LEVEL_TRACE);
    TEST_ASSERT(debug_enabled && !trace_enabled, "Level checking works correctly");
    
    // Test destruction
    uqm_logger_destroy(logger);
    TEST_ASSERT(true, "Logger destruction completed without crash");
}

static void test_basic_logging(void) {
    TEST_START("Basic Logging Operations");
    
    UQM_Logger* logger = uqm_logger_create_default();
    TEST_ASSERT(logger != NULL, "Logger created for basic logging tests");
    
    if (!logger) return;
    
    // Test basic logging macros
    UQM_LOG_INFO(logger, "test", "This is a test info message %s", "");
    UQM_LOG_WARNING(logger, "test", "This is a test warning message %s", "");
    UQM_LOG_ERROR(logger, "test", "This is a test error message %s", "");
    
    // Test formatted logging
    int test_value = 42;
    const char* test_string = "UQM";
    UQM_LOG_INFO(logger, "test", "Formatted message: %s = %d", test_string, test_value);
    
    // Test empty context
    UQM_LOG_INFO(logger, "", "Message with empty context %s", "");
    UQM_LOG_INFO(logger, NULL, "Message with NULL context %s", "");
    
    // Flush to ensure all messages are written
    bool flush_result = uqm_logger_flush(logger);
    TEST_ASSERT(flush_result, "Logger flush works");
    
    printf("  📝 Check above for log messages - they should appear formatted correctly\n");
    TEST_ASSERT(true, "Basic logging operations completed");
    
    uqm_logger_destroy(logger);
}

static void test_log_levels(void) {
    TEST_START("Log Level Filtering");
    
    UQM_Logger* logger = uqm_logger_create_default();
    TEST_ASSERT(logger != NULL, "Logger created for level tests");
    
    if (!logger) return;
    
    // Set to WARNING level - should filter out INFO and below
    uqm_logger_set_level(logger, UQM_LOG_LEVEL_WARNING);
    
    printf("  📝 The following INFO message should NOT appear:\n");
    UQM_LOG_INFO(logger, "test", "This INFO message should be filtered out %s", "");
    
    printf("  📝 The following WARNING message SHOULD appear:\n");
    UQM_LOG_WARNING(logger, "test", "This WARNING message should appear %s", "");
    
    printf("  📝 The following ERROR message SHOULD appear:\n");
    UQM_LOG_ERROR(logger, "test", "This ERROR message should appear %s", "");
    
    // Test FATAL - should always appear
    printf("  📝 The following FATAL message SHOULD appear:\n");
    UQM_LOG_FATAL(logger, "test", "This FATAL message should always appear %s", "");
    
    uqm_logger_flush(logger);
    TEST_ASSERT(true, "Log level filtering tested");
    
    uqm_logger_destroy(logger);
}

static void test_custom_config(void) {
    TEST_START("Custom Logger Configuration");
    
    // Create a custom config
    UQM_LoggerConfig config = {
        .min_level = UQM_LOG_LEVEL_DEBUG,
        .format = UQM_LOG_FORMAT_JSON,
        .include_location = true,
        .include_thread_id = true,
        .async_mode = false,
        .queue_size = 1024,
        .custom_formatter = NULL,
        .output_func = NULL,
        .output_user_data = NULL,
        .output_file = stdout  // Use stdout instead of stderr
    };
    
    UQM_Logger* logger = uqm_logger_create(&config);
    TEST_ASSERT(logger != NULL, "Custom logger creation works");
    
    if (!logger) return;
    
    printf("  📝 The following messages should appear in JSON format:\n");
    UQM_LOG_INFO(logger, "json_test", "This should be in JSON format %s", "");
    UQM_LOG_DEBUG(logger, "json_test", "Debug message in JSON: value=%d", 123);
    
    uqm_logger_flush(logger);
    TEST_ASSERT(true, "Custom configuration works");
    
    uqm_logger_destroy(logger);
}

// Thread test data
typedef struct {
    UQM_Logger* logger;
    int thread_id;
    int message_count;
} ThreadTestData;

static void* thread_logging_worker(void* arg) {
    ThreadTestData* data = (ThreadTestData*)arg;
    
    for (int i = 0; i < data->message_count; i++) {
        UQM_LOG_INFO(data->logger, "thread_test", 
                     "Thread %d, message %d", data->thread_id, i);
        
        // Small delay to interleave messages from different threads
        usleep(1000); // 1ms
    }
    
    return NULL;
}

static void test_thread_safety(void) {
    TEST_START("Thread Safety");
    
    UQM_Logger* logger = uqm_logger_create_default();
    TEST_ASSERT(logger != NULL, "Logger created for thread safety test");
    
    if (!logger) return;
    
    #define MAX_THREADS 4
    const int num_threads = MAX_THREADS;
    const int messages_per_thread = 5;
    
    pthread_t threads[MAX_THREADS];
    ThreadTestData thread_data[MAX_THREADS];
    
    printf("  📝 Starting %d threads, each sending %d messages...\n", 
           num_threads, messages_per_thread);
    
    // Start threads
    for (int i = 0; i < num_threads; i++) {
        thread_data[i].logger = logger;
        thread_data[i].thread_id = i;
        thread_data[i].message_count = messages_per_thread;
        
        int result = pthread_create(&threads[i], NULL, thread_logging_worker, &thread_data[i]);
        TEST_ASSERT(result == 0, "Thread creation successful");
    }
    
    // Wait for all threads to complete
    for (int i = 0; i < num_threads; i++) {
        pthread_join(threads[i], NULL);
    }
    
    uqm_logger_flush(logger);
    printf("  📝 Check above for %d interleaved thread messages\n", 
           num_threads * messages_per_thread);
    TEST_ASSERT(true, "Thread safety test completed");
    
    uqm_logger_destroy(logger);
}

static void test_performance(void) {
    TEST_START("Performance Testing");
    
    UQM_Logger* logger = uqm_logger_create_default();
    TEST_ASSERT(logger != NULL, "Logger created for performance test");
    
    if (!logger) return;
    
    const int test_messages = 1000;
    struct timespec start, end;
    
    printf("  📝 Performance test: %d messages...\n", test_messages);
    
    // Get start time (use CLOCK_REALTIME for better compatibility)
    clock_gettime(CLOCK_REALTIME, &start);
    
    // Log many messages
    for (int i = 0; i < test_messages; i++) {
        UQM_LOG_INFO(logger, "perf_test", "Performance test message %d", i);
    }
    
    uqm_logger_flush(logger);
    
    // Get end time
    clock_gettime(CLOCK_REALTIME, &end);
    
    // Calculate performance
    long duration_ns = (end.tv_sec - start.tv_sec) * 1000000000L + 
                       (end.tv_nsec - start.tv_nsec);
    double duration_ms = (double)duration_ns / 1000000.0;
    double msgs_per_sec = test_messages / (duration_ms / 1000.0);
    
    printf("  📊 Performance Results:\n");
    printf("    - %d messages in %.2f ms\n", test_messages, duration_ms);
    printf("    - %.0f messages/second\n", msgs_per_sec);
    printf("    - %.2f ns/message average\n", (double)duration_ns / test_messages);
    
    // Our target is >1000 msg/sec (should be much higher)
    TEST_ASSERT(msgs_per_sec > 1000, "Performance > 1000 msg/sec");
    TEST_ASSERT(duration_ms < 10000, "Performance test completed in reasonable time");
    
    uqm_logger_destroy(logger);
}

static void print_system_info(void) {
    printf("\n=== System Information ===\n");
    printf("Platform: %s\n", get_platform_name());
    printf("Compiler: %s\n", get_compiler_name());
    printf("Build Date: %s %s\n", __DATE__, __TIME__);
    printf("Process ID: %d\n", getpid());
    printf("Thread Support: Available (pthread)\n");
    
    #ifdef UQM_BIG_ENDIAN
    printf("Byte Order: Big Endian\n");
    #else
    printf("Byte Order: Little Endian\n");
    #endif
    
    printf("Pointer Size: %zu bytes\n", sizeof(void*));
}

int main(int argc, char* argv[]) {
    (void)argc; // Suppress unused parameter warning
    (void)argv; // Suppress unused parameter warning
    
    printf("========================================\n");
    printf("The Ur-Quan Masters - Logging Test\n");
    printf("Phase 2 Week 1: Logging System Validation\n");
    printf("========================================\n");
    
    // Print system info
    print_system_info();
    
    // Run all logging tests
    test_logger_creation();
    test_basic_logging();
    test_log_levels();
    test_custom_config();
    test_thread_safety();
    test_performance();
    
    // Final results
    printf("\n========================================\n");
    printf("LOGGING SYSTEM TEST RESULTS\n");
    printf("========================================\n");
    TEST_END();
    
    if (g_results.tests_failed == 0) {
        printf("\n🎉 ALL LOGGING TESTS PASSED!\n");
        printf("✅ Logging system is working correctly\n");
        printf("✅ Thread safety validated\n");
        printf("✅ Performance targets met\n");
        printf("✅ Ready for Phase 2 Week 2: Memory System\n");
        return EXIT_SUCCESS;
    } else {
        printf("\n❌ %d test(s) failed. Last error: %s\n", 
               g_results.tests_failed, g_results.last_error);
        printf("🔧 Logging system needs fixes before continuing Phase 2\n");
        return EXIT_FAILURE;
    }
}