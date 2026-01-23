/*
 * Test Foundation Systems
 * Tests our newly refactored foundation infrastructure
 * 
 * Phase 1 Foundation Test Program
 * Created: December 2024
 * Purpose: Validate foundation systems before Phase 2
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <time.h>

// Include our foundation systems
#include "shared/common/common_types.h"
#include "shared/common/system_config.h"
#include "shared/logging/modern_logger.h"
#include "shared/memory/smart_memory.h"

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

static void test_common_types(void) {
    TEST_START("Common Types System");
    
    // Test platform detection
    const char* platform = get_platform_name();
    TEST_ASSERT(platform != NULL, "Platform detection works");
    printf("  Platform: %s\n", platform);
    
    // Test compiler detection  
    const char* compiler = get_compiler_name();
    TEST_ASSERT(compiler != NULL, "Compiler detection works");
    printf("  Compiler: %s\n", compiler);
    
    // Test point operations
    Point2D p1 = {10, 20};
    Point2D p2 = {30, 40};
    Point2D result = point_add(p1, p2);
    TEST_ASSERT(result.x == 40 && result.y == 60, "Point addition works");
    
    // Test rectangle operations
    Rect rect = {0, 0, 100, 50};
    Point2D test_point = {50, 25};
    TEST_ASSERT(point_in_rect(test_point, rect), "Point in rectangle detection works");
    
    // Test string utilities
    char test_str[] = "  Hello World  ";
    char* trimmed = str_trim(test_str);
    TEST_ASSERT(strcmp(trimmed, "Hello World") == 0, "String trimming works");
    free(trimmed);
    
    // Test safe string copy
    char dest[10];
    bool copy_result = str_safe_copy(dest, sizeof(dest), "Test123");
    TEST_ASSERT(copy_result && strcmp(dest, "Test123") == 0, "Safe string copy works");
    
    // Test time utilities
    uint64_t timestamp = get_timestamp_ms();
    TEST_ASSERT(timestamp > 0, "Timestamp generation works");
    printf("  Current timestamp: %lu ms\n", timestamp);
}

static void test_system_config(void) {
    TEST_START("System Configuration");
    
    // Initialize config system
    ConfigResult init_result = config_system_init();
    TEST_ASSERT(init_result == CONFIG_SUCCESS, "Config system initialization");
    
    // Test setting and getting values
    ConfigResult set_result = config_set_string("test.name", "UQM Refactored");
    TEST_ASSERT(set_result == CONFIG_SUCCESS, "Setting string value");
    
    const char* name_value = config_get_string("test.name", "default");
    TEST_ASSERT(strcmp(name_value, "UQM Refactored") == 0, "Getting string value");
    
    // Test integer values
    config_set_int("test.width", 1024);
    int width = config_get_int("test.width", 800);
    TEST_ASSERT(width == 1024, "Integer config values");
    
    // Test boolean values
    config_set_bool("test.fullscreen", true);
    bool fullscreen = config_get_bool("test.fullscreen", false);
    TEST_ASSERT(fullscreen == true, "Boolean config values");
    
    // Test float values
    config_set_float("test.volume", 0.75f);
    float volume = config_get_float("test.volume", 1.0f);
    TEST_ASSERT(volume == 0.75f, "Float config values");
    
    // Cleanup
    config_system_shutdown();
}

static void test_logging_system(void) {
    TEST_START("Modern Logging System");
    
    // Initialize logging
    LoggerConfig log_config = {
        .log_level = LOG_LEVEL_DEBUG,
        .output_format = LOG_FORMAT_JSON,
        .enable_colors = true,
        .enable_timestamps = true,
        .enable_thread_id = false,
        .enable_file_line = true,
        .max_message_size = 1024,
        .buffer_size = 10000,
        .flush_interval_ms = 1000
    };
    
    LoggerResult init_result = logger_init(&log_config);
    TEST_ASSERT(init_result == LOGGER_SUCCESS, "Logger initialization");
    
    // Test different log levels
    logger_debug("Testing debug message");
    logger_info("Testing info message");
    logger_warn("Testing warning message");
    logger_error("Testing error message");
    
    // Test formatted logging
    logger_info("Testing %s with %d parameters", "formatted logging", 2);
    
    // Test structured logging
    logger_structured(LOG_LEVEL_INFO, "system.startup", 
                     "component", "foundation_test",
                     "status", "running",
                     "timestamp", get_timestamp_ms(),
                     NULL);
    
    // Get some stats
    LoggerStats stats = logger_get_stats();
    TEST_ASSERT(stats.messages_logged > 0, "Logger recorded messages");
    printf("  Messages logged: %lu\n", stats.messages_logged);
    printf("  Messages dropped: %lu\n", stats.messages_dropped);
    
    // Cleanup
    logger_shutdown();
}

static void test_memory_system(void) {
    TEST_START("Smart Memory Management");
    
    // Initialize memory system
    MemoryResult init_result = memory_system_init(1024 * 1024); // 1MB heap
    TEST_ASSERT(init_result == MEMORY_SUCCESS, "Memory system initialization");
    
    // Test smart pointer creation
    SmartPtr* ptr = smart_ptr_create(100);
    TEST_ASSERT(ptr != NULL, "Smart pointer creation");
    TEST_ASSERT(smart_ptr_get_ref_count(ptr) == 1, "Initial reference count");
    
    // Test reference counting
    SmartPtr* ptr2 = smart_ptr_retain(ptr);
    TEST_ASSERT(smart_ptr_get_ref_count(ptr) == 2, "Reference increment");
    
    smart_ptr_release(ptr2);
    TEST_ASSERT(smart_ptr_get_ref_count(ptr) == 1, "Reference decrement");
    
    // Test memory pool
    MemoryPool* pool = memory_pool_create("test_pool", 64, 10);
    TEST_ASSERT(pool != NULL, "Memory pool creation");
    
    void* mem1 = memory_pool_alloc(pool);
    void* mem2 = memory_pool_alloc(pool);
    TEST_ASSERT(mem1 != NULL && mem2 != NULL, "Pool allocation");
    
    memory_pool_free(pool, mem1);
    memory_pool_free(pool, mem2);
    
    // Get memory stats
    MemoryStats stats = memory_get_stats();
    printf("  Total allocated: %zu bytes\n", stats.total_allocated);
    printf("  Peak usage: %zu bytes\n", stats.peak_usage);
    printf("  Active allocations: %zu\n", stats.active_allocations);
    
    // Cleanup
    smart_ptr_release(ptr);
    memory_pool_destroy(pool);
    memory_system_shutdown();
}

static void run_performance_tests(void) {
    TEST_START("Performance Tests");
    
    // Test logging performance
    LoggerConfig perf_config = {
        .log_level = LOG_LEVEL_INFO,
        .output_format = LOG_FORMAT_STRUCTURED,
        .enable_colors = false,
        .enable_timestamps = true,
        .enable_thread_id = false,
        .enable_file_line = false,
        .max_message_size = 512,
        .buffer_size = 50000,
        .flush_interval_ms = 100
    };
    
    logger_init(&perf_config);
    
    struct timespec start, end;
    clock_gettime(CLOCK_MONOTONIC, &start);
    
    const int test_messages = 1000;
    for (int i = 0; i < test_messages; i++) {
        logger_info("Performance test message %d", i);
    }
    
    clock_gettime(CLOCK_MONOTONIC, &end);
    
    long duration_ns = (end.tv_sec - start.tv_sec) * 1000000000L + 
                       (end.tv_nsec - start.tv_nsec);
    double duration_ms = duration_ns / 1000000.0;
    double msgs_per_sec = test_messages / (duration_ms / 1000.0);
    
    printf("  Logged %d messages in %.2f ms\n", test_messages, duration_ms);
    printf("  Performance: %.0f messages/second\n", msgs_per_sec);
    
    TEST_ASSERT(msgs_per_sec > 1000, "Logging performance > 1000 msg/sec");
    
    logger_shutdown();
}

static void print_system_info(void) {
    printf("\n=== System Information ===\n");
    printf("Platform: %s\n", get_platform_name());
    printf("Compiler: %s\n", get_compiler_name());
    printf("Build Date: %s %s\n", __DATE__, __TIME__);
    printf("Process ID: %d\n", getpid());
    
    // Test endian detection
    #ifdef UQM_BIG_ENDIAN
    printf("Byte Order: Big Endian\n");
    #else
    printf("Byte Order: Little Endian\n");
    #endif
    
    printf("Pointer Size: %zu bytes\n", sizeof(void*));
    printf("Time: %s", ctime(&(time_t){time(NULL)}));
}

int main(int argc, char* argv[]) {
    printf("========================================\n");
    printf("The Ur-Quan Masters - Foundation Test\n");
    printf("Phase 1: Infrastructure Validation\n");
    printf("========================================\n");
    
    // Print system info
    print_system_info();
    
    // Run all tests
    test_common_types();
    test_system_config();
    test_logging_system();
    test_memory_system();
    run_performance_tests();
    
    // Final results
    printf("\n========================================\n");
    printf("FOUNDATION TEST RESULTS\n");
    printf("========================================\n");
    TEST_END();
    
    if (g_results.tests_failed == 0) {
        printf("\n🎉 ALL TESTS PASSED! Foundation is ready.\n");
        printf("✅ Ready for Phase 2: Core Systems Integration\n");
        return EXIT_SUCCESS;
    } else {
        printf("\n❌ %d test(s) failed. Last error: %s\n", 
               g_results.tests_failed, g_results.last_error);
        printf("🔧 Foundation needs fixes before Phase 2\n");
        return EXIT_FAILURE;
    }
}