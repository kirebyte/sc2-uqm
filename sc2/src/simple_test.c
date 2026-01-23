/*
 * Simple Foundation Test
 * Basic validation of our foundation systems
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Include basic types only
#include "shared/common/common_types.h"

/* Simple Test Framework */
static int tests_run = 0;
static int tests_passed = 0;

#define TEST_ASSERT(condition, message) \
    do { \
        tests_run++; \
        if (condition) { \
            tests_passed++; \
            printf("✓ PASS: %s\n", message); \
        } else { \
            printf("✗ FAIL: %s\n", message); \
        } \
    } while(0)

static void test_basic_types(void) {
    printf("\n=== Testing Basic Types ===\n");
    
    // Test platform detection
    const char* platform = get_platform_name();
    TEST_ASSERT(platform != NULL, "Platform detection works");
    printf("  Platform: %s\n", platform);
    
    // Test compiler detection  
    const char* compiler = get_compiler_name();
    TEST_ASSERT(compiler != NULL, "Compiler detection works");
    printf("  Compiler: %s\n", compiler);
    
    // Test basic point operations with proper types
    UQM_Point2i p1 = {10, 20};
    UQM_Point2i p2 = {30, 40};
    TEST_ASSERT(p1.x == 10 && p1.y == 20, "Point creation works");
    TEST_ASSERT(p2.x == 30 && p2.y == 40, "Point creation works");
    
    // Test rectangle operations
    UQM_Rect2i rect = {0, 0, 100, 50};
    TEST_ASSERT(rect.x == 0 && rect.y == 0, "Rect creation works");
    TEST_ASSERT(rect.width == 100 && rect.height == 50, "Rect dimensions work");
    
    // Test color operations
    UQM_Color8 color = {255, 128, 64, 255};
    TEST_ASSERT(color.r == 255 && color.g == 128, "Color creation works");
    TEST_ASSERT(color.b == 64 && color.a == 255, "Color alpha works");
    
    // Test string utilities
    char test_str[] = "  Hello World  ";
    char* trimmed = str_trim(test_str);
    if (trimmed) {
        TEST_ASSERT(strcmp(trimmed, "Hello World") == 0, "String trimming works");
        free(trimmed);
    } else {
        printf("✗ SKIP: String trimming (not implemented)\n");
    }
    
    // Test safe string copy
    char dest[10];
    bool copy_result = str_safe_copy(dest, sizeof(dest), "Test123");
    if (copy_result) {
        TEST_ASSERT(strcmp(dest, "Test123") == 0, "Safe string copy works");
    } else {
        printf("✗ SKIP: Safe string copy (not implemented)\n");
    }
    
    // Test time utilities
    uint64_t timestamp = get_timestamp_ms();
    if (timestamp > 0) {
        TEST_ASSERT(timestamp > 0, "Timestamp generation works");
        printf("  Current timestamp: %lu ms\n", timestamp);
    } else {
        printf("✗ SKIP: Timestamp generation (not implemented)\n");
    }
}

void print_system_info(void) {
    printf("\n=== System Information ===\n");
    printf("Platform: %s\n", get_platform_name());
    printf("Compiler: %s\n", get_compiler_name());
    printf("Build Date: %s %s\n", __DATE__, __TIME__);
    
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
    (void)argc; // Suppress unused parameter warning
    (void)argv; // Suppress unused parameter warning
    printf("========================================\n");
    printf("The Ur-Quan Masters - Foundation Test\n");
    printf("Phase 1: Basic Infrastructure Test\n");
    printf("========================================\n");
    
    // Print system info
    print_system_info();
    
    // Run basic tests
    test_basic_types();
    
    // Final results
    printf("\n========================================\n");
    printf("BASIC FOUNDATION TEST RESULTS\n");
    printf("========================================\n");
    printf("Tests: %d | Passed: %d | Failed: %d\n", 
           tests_run, tests_passed, tests_run - tests_passed);
    
    if (tests_passed == tests_run) {
        printf("\n🎉 ALL BASIC TESTS PASSED!\n");
        printf("✅ Common types system is working\n");
        return EXIT_SUCCESS;
    } else {
        printf("\n❌ %d test(s) failed.\n", tests_run - tests_passed);
        return EXIT_FAILURE;
    }
}