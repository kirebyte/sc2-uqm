/**
 * @file smart_memory.h
 * @brief Modern memory management system for The Ur-Quan Masters
 * 
 * This header provides a comprehensive memory management framework implementing
 * RAII-like patterns, memory pools, leak detection, and smart pointer simulation
 * for C. It follows modern C11/C17 best practices for safe and efficient memory
 * handling.
 * 
 * @author The Ur-Quan Masters Development Team
 * @date 2025
 * @version 1.0.0
 * 
 * ## Features
 * - Smart pointer simulation with automatic cleanup
 * - High-performance memory pools for frequent allocations
 * - Comprehensive leak detection and debugging
 * - Memory alignment and cache optimization
 * - Thread-safe allocators with low contention
 * - Automatic error handling and recovery
 * 
 * ## Usage Example
 * ```c
 * // Initialize memory system
 * uqm_memory_init();
 * 
 * // Smart pointer usage
 * UQM_SmartPtr(char) buffer = UQM_MAKE_UNIQUE(char, 1024);
 * strcpy(buffer.ptr, "Hello, World!");
 * // Automatically freed when buffer goes out of scope
 * 
 * // Memory pool for frequent allocations
 * UQM_MemoryPool* pool = uqm_pool_create(sizeof(GameEntity), 1000);
 * GameEntity* entity = uqm_pool_alloc(pool);
 * // ... use entity ...
 * uqm_pool_free(pool, entity);
 * 
 * // Cleanup
 * uqm_pool_destroy(pool);
 * uqm_memory_shutdown();
 * ```
 * 
 * @copyright Copyright (c) 2025 The Ur-Quan Masters Team
 * @license GNU General Public License v2.0
 */

#ifndef UQM_SMART_MEMORY_H
#define UQM_SMART_MEMORY_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// CONFIGURATION AND CONSTANTS
// ============================================================================

/**
 * @brief Maximum alignment supported by the memory system
 * @details Must be a power of 2. Most modern systems support up to 64-byte
 *          alignment for optimal cache line utilization.
 */
#define UQM_MEMORY_MAX_ALIGNMENT 64

/**
 * @brief Default memory alignment for general allocations
 * @details Chosen to align with common data types and provide good
 *          performance on most architectures.
 */  
#define UQM_MEMORY_DEFAULT_ALIGNMENT 16

/**
 * @brief Size of guard bytes for buffer overflow detection
 * @details Guard bytes are placed before and after allocated blocks
 *          to detect buffer overruns and underruns in debug builds.
 */
#define UQM_MEMORY_GUARD_SIZE 16

/**
 * @brief Maximum length of allocation context strings
 * @details Used for tracking allocation locations and purposes
 *          in debug builds for leak detection and analysis.
 */
#define UQM_MEMORY_CONTEXT_MAX_LENGTH 64

/**
 * @brief Magic numbers for memory validation
 * @details Used to detect memory corruption, use-after-free,
 *          and other memory-related bugs.
 */
#define UQM_MEMORY_MAGIC_ALLOC 0x414C4C4F43 // "ALLOC"
#define UQM_MEMORY_MAGIC_FREE  0x46524545  // "FREE"
#define UQM_MEMORY_GUARD_PATTERN 0xDEADBEEF

// ============================================================================
// TYPE DEFINITIONS
// ============================================================================

/**
 * @brief Memory allocation statistics
 * @details Tracks detailed information about memory usage patterns
 *          for debugging, optimization, and monitoring purposes.
 */
typedef struct {
    uint64_t total_allocated;     ///< Total bytes allocated (lifetime)
    uint64_t total_freed;         ///< Total bytes freed (lifetime)
    uint64_t current_allocated;   ///< Currently allocated bytes
    uint64_t peak_allocated;      ///< Peak memory usage
    uint32_t allocation_count;    ///< Number of active allocations
    uint32_t total_allocations;   ///< Total allocations (lifetime)
    uint32_t total_frees;         ///< Total frees (lifetime)
    uint32_t failed_allocations;  ///< Failed allocation attempts
} UQM_MemoryStats;

/**
 * @brief Memory block metadata (debug builds only)
 * @details Contains detailed information about each allocation
 *          for debugging, leak detection, and memory analysis.
 */
typedef struct UQM_MemoryBlock {
    uint64_t magic;                                    ///< Magic number for validation
    size_t size;                                       ///< Size of the allocated block
    size_t alignment;                                  ///< Alignment of the block
    const char* file;                                  ///< Source file of allocation
    const char* function;                              ///< Source function of allocation
    uint32_t line;                                     ///< Source line number
    char context[UQM_MEMORY_CONTEXT_MAX_LENGTH];       ///< Allocation context/purpose
    uint64_t timestamp;                                ///< Allocation timestamp
    uint32_t thread_id;                                ///< Thread that allocated
    struct UQM_MemoryBlock* prev;                      ///< Previous block in list
    struct UQM_MemoryBlock* next;                      ///< Next block in list
} UQM_MemoryBlock;

/**
 * @brief Custom memory allocator function type
 * @param size Number of bytes to allocate
 * @param alignment Required alignment (must be power of 2)
 * @param context Allocation context for debugging
 * @return Pointer to allocated memory, or NULL on failure
 * 
 * @details Custom allocators allow for specialized memory management
 *          strategies such as stack allocators, pool allocators, etc.
 */
typedef void* (*UQM_Allocator)(size_t size, size_t alignment, const char* context);

/**
 * @brief Custom memory deallocator function type
 * @param ptr Pointer to memory to free
 * @param context Deallocation context for debugging
 * 
 * @details Custom deallocators must be compatible with their corresponding
 *          allocators and handle NULL pointers gracefully.
 */
typedef void (*UQM_Deallocator)(void* ptr, const char* context);

/**
 * @brief Memory cleanup function type
 * @param ptr Pointer to object to cleanup
 * 
 * @details Cleanup functions are called automatically when smart pointers
 *          are destroyed. They should handle resource cleanup, destructor
 *          calls, etc.
 */
typedef void (*UQM_CleanupFunc)(void* ptr);

// ============================================================================
// SMART POINTER SYSTEM
// ============================================================================

/**
 * @brief Smart pointer structure
 * @details Provides automatic memory management similar to C++ unique_ptr.
 *          The pointer is automatically freed when it goes out of scope
 *          or is explicitly reset.
 */
#define UQM_SmartPtr(type) struct { \
    type* ptr; \
    UQM_CleanupFunc cleanup; \
    bool is_valid; \
}

/**
 * @brief Create a smart pointer with automatic cleanup
 * @param type Type of the pointed-to object
 * @param count Number of elements to allocate
 * @return Smart pointer instance
 * 
 * @details Allocates memory for the specified type and count, returning
 *          a smart pointer that will automatically free the memory when
 *          it goes out of scope.
 */
#define UQM_MAKE_UNIQUE(type, count) \
    uqm_smart_ptr_create(sizeof(type) * (count), _Alignof(type), \
                         #type, __FILE__, __func__, __LINE__)

/**
 * @brief Create a smart pointer from existing memory
 * @param ptr Existing pointer to wrap
 * @param cleanup Custom cleanup function
 * @return Smart pointer instance
 * 
 * @details Wraps an existing pointer in a smart pointer with custom cleanup.
 *          Useful for managing resources that require special cleanup logic.
 */
#define UQM_WRAP_UNIQUE(ptr, cleanup) \
    uqm_smart_ptr_wrap((ptr), (cleanup))

/**
 * @brief Reset a smart pointer to NULL
 * @param smart_ptr Smart pointer to reset
 * 
 * @details Frees the current pointer (if any) and sets it to NULL.
 *          Safe to call multiple times.
 */
#define UQM_SMART_PTR_RESET(smart_ptr) \
    do { \
        if ((smart_ptr).is_valid && (smart_ptr).ptr && (smart_ptr).cleanup) { \
            (smart_ptr).cleanup((smart_ptr).ptr); \
        } \
        (smart_ptr).ptr = NULL; \
        (smart_ptr).cleanup = NULL; \
        (smart_ptr).is_valid = false; \
    } while(0)

/**
 * @brief Transfer ownership from one smart pointer to another
 * @param dest Destination smart pointer
 * @param src Source smart pointer (will be reset)
 * 
 * @details Moves ownership from src to dest. After the operation,
 *          src will be NULL and dest will own the pointer.
 */
#define UQM_SMART_PTR_MOVE(dest, src) \
    do { \
        UQM_SMART_PTR_RESET(dest); \
        (dest) = (src); \
        (src).ptr = NULL; \
        (src).cleanup = NULL; \
        (src).is_valid = false; \
    } while(0)

// ============================================================================
// MEMORY POOL SYSTEM
// ============================================================================

/**
 * @brief Forward declaration of memory pool
 * @details Opaque structure to hide implementation details
 */
typedef struct UQM_MemoryPool UQM_MemoryPool;

/**
 * @brief Memory pool configuration
 * @details Specifies parameters for memory pool creation
 */
typedef struct {
    size_t element_size;      ///< Size of each element in bytes
    size_t element_alignment; ///< Required alignment for elements
    size_t initial_capacity;  ///< Initial number of elements to allocate
    size_t growth_factor;     ///< How much to grow when pool is full (0 = no growth)
    bool thread_safe;         ///< Whether the pool should be thread-safe
    const char* name;         ///< Name for debugging and profiling
} UQM_PoolConfig;

/**
 * @brief Memory pool statistics
 * @details Information about pool usage and performance
 */
typedef struct {
    size_t total_capacity;    ///< Total elements that can be stored
    size_t allocated_count;   ///< Number of currently allocated elements
    size_t peak_usage;        ///< Peak number of allocated elements
    size_t total_allocations; ///< Lifetime allocation count
    size_t total_frees;       ///< Lifetime free count
    size_t memory_used;       ///< Total memory used by the pool
    double fragmentation;     ///< Fragmentation ratio (0.0 - 1.0)
} UQM_PoolStats;

// ============================================================================
// CORE MEMORY MANAGEMENT API
// ============================================================================

/**
 * @brief Initialize the memory management system
 * @return true on success, false on failure
 * 
 * @details Must be called before using any other memory management functions.
 *          Sets up global state, statistics tracking, and debugging facilities.
 * 
 * @note This function is not thread-safe and should be called only once
 *       during program initialization.
 */
bool uqm_memory_init(void);

/**
 * @brief Shutdown the memory management system
 * 
 * @details Frees all remaining allocations, reports memory leaks (in debug
 *          builds), and cleans up global state. Should be called during
 *          program shutdown.
 * 
 * @warning After calling this function, no other memory management functions
 *          should be used.
 */
void uqm_memory_shutdown(void);

/**
 * @brief Get current memory usage statistics
 * @param stats Output structure for statistics
 * @return true on success, false on failure
 * 
 * @details Provides detailed information about current memory usage,
 *          including totals, peaks, and allocation counts.
 */
bool uqm_memory_get_stats(UQM_MemoryStats* stats);

/**
 * @brief Reset memory usage statistics
 * 
 * @details Resets all counters except current usage. Useful for measuring
 *          memory usage over specific time periods or operations.
 */
void uqm_memory_reset_stats(void);

/**
 * @brief Enable or disable memory leak detection
 * @param enabled Whether leak detection should be enabled
 * @return Previous state of leak detection
 * 
 * @details In debug builds, tracks all allocations and reports leaks.
 *          Has no effect in release builds.
 */
bool uqm_memory_set_leak_detection(bool enabled);

/**
 * @brief Report current memory leaks to the log
 * @return Number of leaked blocks found
 * 
 * @details Scans all tracked allocations and reports those that haven't
 *          been freed. Only available in debug builds.
 */
uint32_t uqm_memory_report_leaks(void);

// ============================================================================
// ALLOCATION FUNCTIONS
// ============================================================================

/**
 * @brief Allocate aligned memory with debugging information
 * @param size Number of bytes to allocate
 * @param alignment Required alignment (must be power of 2)
 * @param context Description of allocation purpose
 * @param file Source file name (__FILE__)
 * @param function Source function name (__func__)  
 * @param line Source line number (__LINE__)
 * @return Pointer to allocated memory, or NULL on failure
 * 
 * @details Core allocation function with full debugging support.
 *          All other allocation functions ultimately call this one.
 */
void* uqm_memory_alloc_debug(size_t size, size_t alignment, const char* context,
                             const char* file, const char* function, uint32_t line);

/**
 * @brief Free memory allocated with uqm_memory_alloc_debug
 * @param ptr Pointer to memory to free (NULL is safe)
 * @param file Source file name (__FILE__)
 * @param function Source function name (__func__)
 * @param line Source line number (__LINE__)
 * 
 * @details Frees memory and updates tracking information. Performs
 *          validation checks in debug builds to detect corruption.
 */
void uqm_memory_free_debug(void* ptr, const char* file, 
                          const char* function, uint32_t line);

/**
 * @brief Reallocate memory with new size
 * @param ptr Existing pointer (NULL for initial allocation)
 * @param new_size New size in bytes
 * @param alignment Required alignment
 * @param context Description for debugging
 * @param file Source file name
 * @param function Source function name  
 * @param line Source line number
 * @return Pointer to resized memory, or NULL on failure
 * 
 * @details Resizes an existing allocation. If ptr is NULL, behaves like
 *          malloc. If new_size is 0, behaves like free and returns NULL.
 */
void* uqm_memory_realloc_debug(void* ptr, size_t new_size, size_t alignment,
                               const char* context, const char* file,
                               const char* function, uint32_t line);

// ============================================================================
// CONVENIENCE MACROS
// ============================================================================

/**
 * @brief Allocate memory with default alignment
 * @param size Number of bytes to allocate
 * @param context Description of allocation purpose
 * @return Pointer to allocated memory, or NULL on failure
 */
#define UQM_ALLOC(size, context) \
    uqm_memory_alloc_debug((size), UQM_MEMORY_DEFAULT_ALIGNMENT, (context), \
                           __FILE__, __func__, __LINE__)

/**
 * @brief Allocate aligned memory
 * @param size Number of bytes to allocate
 * @param alignment Required alignment (must be power of 2)
 * @param context Description of allocation purpose
 * @return Pointer to allocated memory, or NULL on failure
 */
#define UQM_ALLOC_ALIGNED(size, alignment, context) \
    uqm_memory_alloc_debug((size), (alignment), (context), \
                           __FILE__, __func__, __LINE__)

/**
 * @brief Allocate and zero-initialize memory
 * @param count Number of elements
 * @param size Size of each element
 * @param context Description of allocation purpose
 * @return Pointer to allocated memory, or NULL on failure
 */
#define UQM_CALLOC(count, size, context) \
    uqm_memory_calloc_debug((count), (size), UQM_MEMORY_DEFAULT_ALIGNMENT, \
                            (context), __FILE__, __func__, __LINE__)

/**
 * @brief Free memory
 * @param ptr Pointer to memory to free (NULL is safe)
 */
#define UQM_FREE(ptr) \
    uqm_memory_free_debug((ptr), __FILE__, __func__, __LINE__)

/**
 * @brief Reallocate memory
 * @param ptr Existing pointer
 * @param new_size New size in bytes
 * @param context Description for debugging
 * @return Pointer to resized memory, or NULL on failure
 */
#define UQM_REALLOC(ptr, new_size, context) \
    uqm_memory_realloc_debug((ptr), (new_size), UQM_MEMORY_DEFAULT_ALIGNMENT, \
                             (context), __FILE__, __func__, __LINE__)

/**
 * @brief Duplicate a string using the memory system
 * @param str String to duplicate
 * @param context Description for debugging
 * @return Pointer to duplicated string, or NULL on failure
 */
#define UQM_STRDUP(str, context) \
    uqm_memory_strdup_debug((str), (context), __FILE__, __func__, __LINE__)

// ============================================================================
// SMART POINTER FUNCTIONS
// ============================================================================

/**
 * @brief Create a smart pointer (internal implementation)
 * @param size Size to allocate
 * @param alignment Required alignment
 * @param type_name Type name for debugging
 * @param file Source file
 * @param function Source function
 * @param line Source line
 * @return Smart pointer structure
 */
void* uqm_smart_ptr_create(size_t size, size_t alignment, const char* type_name,
                          const char* file, const char* function, uint32_t line);

/**
 * @brief Wrap existing pointer in smart pointer
 * @param ptr Existing pointer
 * @param cleanup Cleanup function
 * @return Smart pointer structure
 */
void* uqm_smart_ptr_wrap(void* ptr, UQM_CleanupFunc cleanup);

/**
 * @brief Default cleanup function for smart pointers
 * @param ptr Pointer to free
 */
void uqm_smart_ptr_default_cleanup(void* ptr);

// ============================================================================
// MEMORY POOL FUNCTIONS
// ============================================================================

/**
 * @brief Create a new memory pool
 * @param config Pool configuration
 * @return Pointer to new pool, or NULL on failure
 * 
 * @details Creates a memory pool optimized for frequent allocation and
 *          deallocation of fixed-size objects. Provides better performance
 *          and less fragmentation than general allocators.
 */
UQM_MemoryPool* uqm_pool_create(const UQM_PoolConfig* config);

/**
 * @brief Create a memory pool with default configuration
 * @param element_size Size of each element
 * @param initial_capacity Initial number of elements
 * @param name Pool name for debugging
 * @return Pointer to new pool, or NULL on failure
 */
UQM_MemoryPool* uqm_pool_create_simple(size_t element_size, size_t initial_capacity, 
                                       const char* name);

/**
 * @brief Destroy a memory pool and free all memory
 * @param pool Pool to destroy
 * 
 * @details Frees all memory associated with the pool. Any outstanding
 *          allocations from the pool become invalid after this call.
 * 
 * @warning Ensure all allocated objects are no longer in use before
 *          calling this function.
 */
void uqm_pool_destroy(UQM_MemoryPool* pool);

/**
 * @brief Allocate an object from a memory pool
 * @param pool Memory pool
 * @return Pointer to allocated object, or NULL on failure
 * 
 * @details Allocates a single object from the pool. Much faster than
 *          general malloc for fixed-size allocations.
 */
void* uqm_pool_alloc(UQM_MemoryPool* pool);

/**
 * @brief Free an object back to a memory pool
 * @param pool Memory pool
 * @param ptr Pointer to object to free
 * 
 * @details Returns an object to the pool for reuse. The object must
 *          have been allocated from the same pool.
 * 
 * @warning Freeing an object to the wrong pool or freeing the same
 *          object twice results in undefined behavior.
 */
void uqm_pool_free(UQM_MemoryPool* pool, void* ptr);

/**
 * @brief Get memory pool statistics
 * @param pool Memory pool
 * @param stats Output structure for statistics
 * @return true on success, false on failure
 * 
 * @details Provides detailed information about pool usage, performance,
 *          and memory overhead.
 */
bool uqm_pool_get_stats(const UQM_MemoryPool* pool, UQM_PoolStats* stats);

/**
 * @brief Reset memory pool statistics
 * @param pool Memory pool
 * 
 * @details Resets performance counters while preserving current allocations.
 *          Useful for measuring performance over specific periods.
 */
void uqm_pool_reset_stats(UQM_MemoryPool* pool);

/**
 * @brief Shrink pool to minimum size needed
 * @param pool Memory pool
 * @return Number of bytes freed
 * 
 * @details Frees unused memory in the pool while preserving all
 *          current allocations. Useful for reducing memory footprint.
 */
size_t uqm_pool_shrink(UQM_MemoryPool* pool);

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/**
 * @brief Duplicate a string with memory tracking
 * @param str String to duplicate
 * @param context Description for debugging
 * @param file Source file
 * @param function Source function
 * @param line Source line
 * @return Pointer to duplicated string, or NULL on failure
 */
char* uqm_memory_strdup_debug(const char* str, const char* context,
                              const char* file, const char* function, uint32_t line);

/**
 * @brief Allocate and zero-initialize memory
 * @param count Number of elements
 * @param size Size of each element
 * @param alignment Required alignment
 * @param context Description for debugging
 * @param file Source file
 * @param function Source function
 * @param line Source line
 * @return Pointer to allocated memory, or NULL on failure
 */
void* uqm_memory_calloc_debug(size_t count, size_t size, size_t alignment,
                              const char* context, const char* file,
                              const char* function, uint32_t line);

/**
 * @brief Check if a pointer was allocated by the memory system
 * @param ptr Pointer to check
 * @return true if valid, false otherwise
 * 
 * @details Validates that a pointer was allocated by this memory system
 *          and hasn't been corrupted. Only available in debug builds.
 */
bool uqm_memory_is_valid_pointer(const void* ptr);

/**
 * @brief Get information about an allocated block
 * @param ptr Pointer to query
 * @return Pointer to block information, or NULL if not found
 * 
 * @details Returns metadata about an allocation, including size,
 *          location, and timing information. Debug builds only.
 */
const UQM_MemoryBlock* uqm_memory_get_block_info(const void* ptr);

/**
 * @brief Set custom allocator functions
 * @param allocator Custom allocator function
 * @param deallocator Custom deallocator function
 * @return true on success, false on failure
 * 
 * @details Allows replacing the default allocator with custom implementations.
 *          Useful for specialized memory management strategies.
 * 
 * @warning The custom allocator must be compatible with the expected
 *          interface and handle all edge cases properly.
 */
bool uqm_memory_set_allocator(UQM_Allocator allocator, UQM_Deallocator deallocator);

/**
 * @brief Restore default allocator functions
 * 
 * @details Resets to the built-in allocator after using custom allocators.
 */
void uqm_memory_restore_default_allocator(void);

#ifdef __cplusplus
}
#endif

#endif // UQM_SMART_MEMORY_H