/**
 * @file smart_memory.c
 * @brief Implementation of Smart Memory Management System for UQM
 * 
 * This implementation provides:
 * - Smart pointer simulation with reference counting
 * - Memory pools for efficient allocation
 * - RAII-like patterns in C
 * - Memory leak detection and debugging
 * - Thread-safe memory operations
 * - Performance optimization through pooling
 * 
 * @author The Ur-Quan Masters Refactoring Team
 * @version 2.0.0
 * @date 2025-01-27
 * 
 * @copyright Copyright (c) 2002 Toys for Bob, Inc.
 * @license GNU General Public License v2.0
 */

#include "smart_memory.h"
#include "../logging/modern_logger.h"
#include "../common/error_handling.h"
#include <stdlib.h>
#include <string.h>
#include <stdatomic.h>
#include <pthread.h>
#include <assert.h>
#include <time.h>

/* ========================================================================== */
/*                               CONSTANTS                                    */
/* ========================================================================== */

/** @brief Maximum number of memory pools supported */
#define UQM_MAX_MEMORY_POOLS 64

/** @brief Default block size for memory pools */
#define UQM_DEFAULT_BLOCK_SIZE 1024

/** @brief Alignment for memory allocations */
#define UQM_MEMORY_ALIGNMENT 16

/** @brief Magic number for memory block validation */
#define UQM_MEMORY_MAGIC 0xDEADBEEF

/** @brief Maximum allocation size to prevent abuse */
#define UQM_MAX_ALLOCATION_SIZE (1024 * 1024 * 1024) // 1GB

/* ========================================================================== */
/*                            INTERNAL STRUCTURES                            */
/* ========================================================================== */

/**
 * @brief Memory block header for debugging and validation
 */
typedef struct UQM_MemoryBlockHeader {
    uint32_t magic;              /**< Magic number for validation */
    size_t size;                 /**< Size of the allocated block */
    const char* file;            /**< Source file where allocated */
    int line;                    /**< Line number where allocated */
    struct timespec timestamp;   /**< Allocation timestamp */
    struct UQM_MemoryBlockHeader* next; /**< Next block in leak detection list */
} UQM_MemoryBlockHeader;

/**
 * @brief Memory pool implementation
 */
struct UQM_MemoryPool {
    char name[64];                /**< Pool name for debugging */
    size_t block_size;            /**< Size of each block */
    size_t total_blocks;          /**< Total number of blocks */
    size_t used_blocks;           /**< Number of blocks in use */
    atomic_size_t peak_usage;     /**< Peak usage statistics */
    
    void* memory_start;           /**< Start of allocated memory */
    void* free_list;              /**< Free blocks linked list */
    
    pthread_mutex_t mutex;        /**< Thread safety mutex */
    bool is_initialized;          /**< Initialization flag */
    
    // Statistics
    atomic_size_t total_allocations;  /**< Total allocations made */
    atomic_size_t total_deallocations; /**< Total deallocations made */
    atomic_size_t current_usage;       /**< Current memory usage */
};

/**
 * @brief Smart pointer implementation with reference counting
 */
struct UQM_SmartPointer {
    void* data;                   /**< Pointer to actual data */
    atomic_int ref_count;         /**< Reference counter */
    UQM_SmartPtr_Deleter deleter; /**< Custom deleter function */
    size_t size;                  /**< Size of allocated data */
    pthread_mutex_t mutex;        /**< Thread safety mutex */
    const char* type_name;        /**< Type name for debugging */
};

/* ========================================================================== */
/*                              GLOBAL STATE                                 */
/* ========================================================================== */

/** @brief Global memory pools array */
static UQM_MemoryPool g_memory_pools[UQM_MAX_MEMORY_POOLS];

/** @brief Number of initialized memory pools */
static atomic_int g_pool_count = 0;

/** @brief Global mutex for pool management */
static pthread_mutex_t g_pool_mutex = PTHREAD_MUTEX_INITIALIZER;

/** @brief Memory leak detection list */
static UQM_MemoryBlockHeader* g_leak_detection_list = NULL;

/** @brief Mutex for leak detection */
static pthread_mutex_t g_leak_mutex = PTHREAD_MUTEX_INITIALIZER;

/** @brief Memory management initialization flag */
static atomic_bool g_memory_initialized = false;

/** @brief Global memory statistics */
static struct {
    atomic_size_t total_allocated;
    atomic_size_t total_freed;
    atomic_size_t current_usage;
    atomic_size_t peak_usage;
    atomic_size_t allocation_count;
    atomic_size_t deallocation_count;
} g_memory_stats = {0};

/* ========================================================================== */
/*                           INTERNAL FUNCTIONS                              */
/* ========================================================================== */

/**
 * @brief Align memory address to specified boundary
 * @param size Size to align
 * @param alignment Alignment boundary
 * @return Aligned size
 */
static size_t align_size(size_t size, size_t alignment) {
    return ((size + alignment - 1) / alignment) * alignment;
}

/**
 * @brief Add memory block to leak detection list
 * @param header Memory block header
 */
static void add_to_leak_detection(UQM_MemoryBlockHeader* header) {
    if (!header) return;
    
    pthread_mutex_lock(&g_leak_mutex);
    header->next = g_leak_detection_list;
    g_leak_detection_list = header;
    pthread_mutex_unlock(&g_leak_mutex);
}

/**
 * @brief Remove memory block from leak detection list
 * @param header Memory block header
 */
static void remove_from_leak_detection(UQM_MemoryBlockHeader* header) {
    if (!header) return;
    
    pthread_mutex_lock(&g_leak_mutex);
    
    UQM_MemoryBlockHeader** current = &g_leak_detection_list;
    while (*current) {
        if (*current == header) {
            *current = header->next;
            break;
        }
        current = &(*current)->next;
    }
    
    pthread_mutex_unlock(&g_leak_mutex);
}

/**
 * @brief Initialize memory management system
 * @return Result indicating success or failure
 */
static UQM_Result(void*, UQM_MemoryError) initialize_memory_system(void) {
    if (atomic_load(&g_memory_initialized)) {
        return UQM_OK_VOID();
    }
    
    // Initialize statistics
    atomic_store(&g_memory_stats.total_allocated, 0);
    atomic_store(&g_memory_stats.total_freed, 0);
    atomic_store(&g_memory_stats.current_usage, 0);
    atomic_store(&g_memory_stats.peak_usage, 0);
    atomic_store(&g_memory_stats.allocation_count, 0);
    atomic_store(&g_memory_stats.deallocation_count, 0);
    
    // Initialize pools
    memset(g_memory_pools, 0, sizeof(g_memory_pools));
    atomic_store(&g_pool_count, 0);
    
    atomic_store(&g_memory_initialized, true);
    
    UQM_LOG_INFO("Memory management system initialized successfully");
    return UQM_OK_VOID();
}

/* ========================================================================== */
/*                          MEMORY POOL FUNCTIONS                            */
/* ========================================================================== */

UQM_Result(UQM_MemoryPool*, UQM_MemoryError) 
UQM_CreateMemoryPool(const char* name, size_t block_size, size_t block_count) {
    UQM_LOG_TRACE("Creating memory pool: %s (block_size=%zu, count=%zu)", 
                  name ? name : "unnamed", block_size, block_count);
    
    // Initialize memory system if needed
    UQM_TRY_VOID(initialize_memory_system());
    
    // Validate parameters
    if (block_size == 0 || block_count == 0) {
        UQM_LOG_ERROR("Invalid pool parameters: block_size=%zu, count=%zu", 
                      block_size, block_count);
        return UQM_ERROR(UQM_MEMORY_INVALID_ARGUMENT);
    }
    
    if (block_size > UQM_MAX_ALLOCATION_SIZE / block_count) {
        UQM_LOG_ERROR("Pool would exceed maximum allocation size");
        return UQM_ERROR(UQM_MEMORY_INVALID_ARGUMENT);
    }
    
    pthread_mutex_lock(&g_pool_mutex);
    
    // Check if we have space for another pool
    int current_count = atomic_load(&g_pool_count);
    if (current_count >= UQM_MAX_MEMORY_POOLS) {
        pthread_mutex_unlock(&g_pool_mutex);
        UQM_LOG_ERROR("Maximum number of memory pools exceeded");
        return UQM_ERROR(UQM_MEMORY_POOL_FULL);
    }
    
    // Find an empty slot
    UQM_MemoryPool* pool = NULL;
    for (int i = 0; i < UQM_MAX_MEMORY_POOLS; i++) {
        if (!g_memory_pools[i].is_initialized) {
            pool = &g_memory_pools[i];
            break;
        }
    }
    
    if (!pool) {
        pthread_mutex_unlock(&g_pool_mutex);
        UQM_LOG_ERROR("No available pool slots");
        return UQM_ERROR(UQM_MEMORY_POOL_FULL);
    }
    
    // Align block size
    size_t aligned_block_size = align_size(block_size, UQM_MEMORY_ALIGNMENT);
    size_t total_size = aligned_block_size * block_count;
    
    // Allocate memory for the pool
    void* memory = aligned_alloc(UQM_MEMORY_ALIGNMENT, total_size);
    if (!memory) {
        pthread_mutex_unlock(&g_pool_mutex);
        UQM_LOG_ERROR("Failed to allocate memory for pool");
        return UQM_ERROR(UQM_MEMORY_OUT_OF_MEMORY);
    }
    
    // Initialize pool structure
    memset(pool, 0, sizeof(UQM_MemoryPool));
    if (name) {
        strncpy(pool->name, name, sizeof(pool->name) - 1);
        pool->name[sizeof(pool->name) - 1] = '\0';
    } else {
        snprintf(pool->name, sizeof(pool->name), "Pool_%d", current_count);
    }
    
    pool->block_size = aligned_block_size;
    pool->total_blocks = block_count;
    pool->used_blocks = 0;
    atomic_store(&pool->peak_usage, 0);
    pool->memory_start = memory;
    pool->free_list = memory;
    
    if (pthread_mutex_init(&pool->mutex, NULL) != 0) {
        free(memory);
        pthread_mutex_unlock(&g_pool_mutex);
        UQM_LOG_ERROR("Failed to initialize pool mutex");
        return UQM_ERROR(UQM_MEMORY_SYSTEM_ERROR);
    }
    
    // Initialize statistics
    atomic_store(&pool->total_allocations, 0);
    atomic_store(&pool->total_deallocations, 0);
    atomic_store(&pool->current_usage, 0);
    
    // Set up free list
    char* current = (char*)memory;
    for (size_t i = 0; i < block_count - 1; i++) {
        *(void**)current = current + aligned_block_size;
        current += aligned_block_size;
    }
    *(void**)current = NULL; // Last block points to NULL
    
    pool->is_initialized = true;
    atomic_fetch_add(&g_pool_count, 1);
    
    pthread_mutex_unlock(&g_pool_mutex);
    
    UQM_LOG_INFO("Memory pool '%s' created successfully (%zu blocks of %zu bytes)", 
                 pool->name, block_count, aligned_block_size);
    
    return UQM_OK(pool);
}

UQM_Result(void*, UQM_MemoryError) 
UQM_MemoryPool_Alloc(UQM_MemoryPool* pool) {
    if (!pool || !pool->is_initialized) {
        UQM_LOG_ERROR("Invalid or uninitialized memory pool");
        return UQM_ERROR(UQM_MEMORY_INVALID_ARGUMENT);
    }
    
    pthread_mutex_lock(&pool->mutex);
    
    if (!pool->free_list) {
        pthread_mutex_unlock(&pool->mutex);
        UQM_LOG_WARNING("Memory pool '%s' is exhausted", pool->name);
        return UQM_ERROR(UQM_MEMORY_POOL_EXHAUSTED);
    }
    
    // Get block from free list
    void* block = pool->free_list;
    pool->free_list = *(void**)pool->free_list;
    
    pool->used_blocks++;
    atomic_fetch_add(&pool->total_allocations, 1);
    atomic_fetch_add(&pool->current_usage, pool->block_size);
    
    // Update peak usage
    size_t current_usage = atomic_load(&pool->current_usage);
    size_t peak = atomic_load(&pool->peak_usage);
    while (current_usage > peak && 
           !atomic_compare_exchange_weak(&pool->peak_usage, &peak, current_usage)) {
        // Keep trying until we successfully update or peak becomes larger
    }
    
    pthread_mutex_unlock(&pool->mutex);
    
    // Clear the block
    memset(block, 0, pool->block_size);
    
    UQM_LOG_TRACE("Allocated block from pool '%s' (%p, used: %zu/%zu)", 
                  pool->name, block, pool->used_blocks, pool->total_blocks);
    
    return UQM_OK(block);
}

UQM_Result(void*, UQM_MemoryError) 
UQM_MemoryPool_Free(UQM_MemoryPool* pool, void* ptr) {
    if (!pool || !pool->is_initialized) {
        UQM_LOG_ERROR("Invalid or uninitialized memory pool");
        return UQM_ERROR(UQM_MEMORY_INVALID_ARGUMENT);
    }
    
    if (!ptr) {
        return UQM_OK_VOID(); // Freeing NULL is a no-op
    }
    
    // Validate that pointer belongs to this pool
    char* pool_start = (char*)pool->memory_start;
    char* pool_end = pool_start + (pool->total_blocks * pool->block_size);
    char* block_ptr = (char*)ptr;
    
    if (block_ptr < pool_start || block_ptr >= pool_end) {
        UQM_LOG_ERROR("Pointer %p does not belong to pool '%s'", ptr, pool->name);
        return UQM_ERROR(UQM_MEMORY_INVALID_POINTER);
    }
    
    // Check alignment
    if ((block_ptr - pool_start) % pool->block_size != 0) {
        UQM_LOG_ERROR("Pointer %p is not properly aligned for pool '%s'", ptr, pool->name);
        return UQM_ERROR(UQM_MEMORY_INVALID_POINTER);
    }
    
    pthread_mutex_lock(&pool->mutex);
    
    // Add block back to free list
    *(void**)ptr = pool->free_list;
    pool->free_list = ptr;
    
    pool->used_blocks--;
    atomic_fetch_add(&pool->total_deallocations, 1);
    atomic_fetch_sub(&pool->current_usage, pool->block_size);
    
    pthread_mutex_unlock(&pool->mutex);
    
    UQM_LOG_TRACE("Freed block from pool '%s' (%p, used: %zu/%zu)", 
                  pool->name, ptr, pool->used_blocks, pool->total_blocks);
    
    return UQM_OK_VOID();
}

UQM_Result(void*, UQM_MemoryError) 
UQM_DestroyMemoryPool(UQM_MemoryPool* pool) {
    if (!pool || !pool->is_initialized) {
        return UQM_OK_VOID(); // Already destroyed or invalid
    }
    
    UQM_LOG_INFO("Destroying memory pool '%s'", pool->name);
    
    pthread_mutex_lock(&g_pool_mutex);
    pthread_mutex_lock(&pool->mutex);
    
    // Check for memory leaks
    if (pool->used_blocks > 0) {
        UQM_LOG_WARNING("Memory pool '%s' destroyed with %zu blocks still in use", 
                        pool->name, pool->used_blocks);
    }
    
    // Free the pool memory
    if (pool->memory_start) {
        free(pool->memory_start);
    }
    
    // Print statistics
    UQM_LOG_INFO("Pool '%s' statistics: "
                 "Total allocs: %zu, Total deallocs: %zu, Peak usage: %zu bytes",
                 pool->name,
                 atomic_load(&pool->total_allocations),
                 atomic_load(&pool->total_deallocations),
                 atomic_load(&pool->peak_usage));
    
    pthread_mutex_unlock(&pool->mutex);
    pthread_mutex_destroy(&pool->mutex);
    
    // Clear the pool structure
    memset(pool, 0, sizeof(UQM_MemoryPool));
    atomic_fetch_sub(&g_pool_count, 1);
    
    pthread_mutex_unlock(&g_pool_mutex);
    
    return UQM_OK_VOID();
}

/* ========================================================================== */
/*                        SMART POINTER FUNCTIONS                            */
/* ========================================================================== */

UQM_Result(UQM_SmartPointer*, UQM_MemoryError) 
UQM_SmartPtr_Create(void* data, size_t size, UQM_SmartPtr_Deleter deleter, const char* type_name) {
    if (!data) {
        UQM_LOG_ERROR("Cannot create smart pointer for NULL data");
        return UQM_ERROR(UQM_MEMORY_INVALID_ARGUMENT);
    }
    
    UQM_SmartPointer* smart_ptr = malloc(sizeof(UQM_SmartPointer));
    if (!smart_ptr) {
        UQM_LOG_ERROR("Failed to allocate memory for smart pointer");
        return UQM_ERROR(UQM_MEMORY_OUT_OF_MEMORY);
    }
    
    smart_ptr->data = data;
    atomic_store(&smart_ptr->ref_count, 1);
    smart_ptr->deleter = deleter ? deleter : free;
    smart_ptr->size = size;
    smart_ptr->type_name = type_name ? type_name : "unknown";
    
    if (pthread_mutex_init(&smart_ptr->mutex, NULL) != 0) {
        free(smart_ptr);
        UQM_LOG_ERROR("Failed to initialize smart pointer mutex");
        return UQM_ERROR(UQM_MEMORY_SYSTEM_ERROR);
    }
    
    UQM_LOG_TRACE("Created smart pointer for %s (%p, size: %zu)", 
                  smart_ptr->type_name, data, size);
    
    return UQM_OK(smart_ptr);
}

UQM_SmartPointer* UQM_SmartPtr_AddRef(UQM_SmartPointer* smart_ptr) {
    if (!smart_ptr) {
        return NULL;
    }
    
    int old_count = atomic_fetch_add(&smart_ptr->ref_count, 1);
    
    UQM_LOG_TRACE("Smart pointer %s ref count increased: %d -> %d", 
                  smart_ptr->type_name, old_count, old_count + 1);
    
    return smart_ptr;
}

UQM_Result(void*, UQM_MemoryError) 
UQM_SmartPtr_Release(UQM_SmartPointer* smart_ptr) {
    if (!smart_ptr) {
        return UQM_OK_VOID();
    }
    
    int old_count = atomic_fetch_sub(&smart_ptr->ref_count, 1);
    
    UQM_LOG_TRACE("Smart pointer %s ref count decreased: %d -> %d", 
                  smart_ptr->type_name, old_count, old_count - 1);
    
    if (old_count == 1) {
        // Last reference, cleanup
        UQM_LOG_TRACE("Cleaning up smart pointer %s (%p)", 
                      smart_ptr->type_name, smart_ptr->data);
        
        if (smart_ptr->data && smart_ptr->deleter) {
            smart_ptr->deleter(smart_ptr->data);
        }
        
        pthread_mutex_destroy(&smart_ptr->mutex);
        free(smart_ptr);
    }
    
    return UQM_OK_VOID();
}

void* UQM_SmartPtr_Get(UQM_SmartPointer* smart_ptr) {
    if (!smart_ptr) {
        return NULL;
    }
    
    // Check if the pointer is still valid
    if (atomic_load(&smart_ptr->ref_count) <= 0) {
        UQM_LOG_ERROR("Attempted to access destroyed smart pointer %s", 
                      smart_ptr->type_name);
        return NULL;
    }
    
    return smart_ptr->data;
}

int UQM_SmartPtr_GetRefCount(UQM_SmartPointer* smart_ptr) {
    if (!smart_ptr) {
        return 0;
    }
    
    return atomic_load(&smart_ptr->ref_count);
}

/* ========================================================================== */
/*                           GENERAL ALLOCATION                              */
/* ========================================================================== */

UQM_Result(void*, UQM_MemoryError) 
UQM_Memory_Alloc_Debug(size_t size, const char* file, int line) {
    UQM_TRY_VOID(initialize_memory_system());
    
    if (size == 0) {
        UQM_LOG_WARNING("Zero-sized allocation requested at %s:%d", file, line);
        return UQM_OK(NULL);
    }
    
    if (size > UQM_MAX_ALLOCATION_SIZE) {
        UQM_LOG_ERROR("Allocation too large: %zu bytes at %s:%d", size, file, line);
        return UQM_ERROR(UQM_MEMORY_INVALID_ARGUMENT);
    }
    
    // Allocate space for header + data
    size_t total_size = sizeof(UQM_MemoryBlockHeader) + size;
    UQM_MemoryBlockHeader* header = aligned_alloc(UQM_MEMORY_ALIGNMENT, total_size);
    
    if (!header) {
        UQM_LOG_ERROR("Failed to allocate %zu bytes at %s:%d", size, file, line);
        return UQM_ERROR(UQM_MEMORY_OUT_OF_MEMORY);
    }
    
    // Initialize header
    header->magic = UQM_MEMORY_MAGIC;
    header->size = size;
    header->file = file;
    header->line = line;
    clock_gettime(CLOCK_MONOTONIC, &header->timestamp);
    header->next = NULL;
    
    // Add to leak detection
    add_to_leak_detection(header);
    
    // Update statistics
    atomic_fetch_add(&g_memory_stats.total_allocated, size);
    atomic_fetch_add(&g_memory_stats.allocation_count, 1);
    size_t current = atomic_fetch_add(&g_memory_stats.current_usage, size) + size;
    
    // Update peak usage
    size_t peak = atomic_load(&g_memory_stats.peak_usage);
    while (current > peak && 
           !atomic_compare_exchange_weak(&g_memory_stats.peak_usage, &peak, current)) {
        // Keep trying
    }
    
    void* user_ptr = (char*)header + sizeof(UQM_MemoryBlockHeader);
    memset(user_ptr, 0, size);
    
    UQM_LOG_TRACE("Allocated %zu bytes at %p (%s:%d)", size, user_ptr, file, line);
    
    return UQM_OK(user_ptr);
}

UQM_Result(void*, UQM_MemoryError) 
UQM_Memory_Free_Debug(void* ptr, const char* file, int line) {
    if (!ptr) {
        return UQM_OK_VOID(); // Freeing NULL is a no-op
    }
    
    // Get header
    UQM_MemoryBlockHeader* header = (UQM_MemoryBlockHeader*)((char*)ptr - sizeof(UQM_MemoryBlockHeader));
    
    // Validate magic number
    if (header->magic != UQM_MEMORY_MAGIC) {
        UQM_LOG_ERROR("Invalid memory block or double-free detected at %s:%d (ptr: %p)", 
                      file, line, ptr);
        return UQM_ERROR(UQM_MEMORY_CORRUPTION);
    }
    
    // Remove from leak detection
    remove_from_leak_detection(header);
    
    // Update statistics
    atomic_fetch_add(&g_memory_stats.total_freed, header->size);
    atomic_fetch_add(&g_memory_stats.deallocation_count, 1);
    atomic_fetch_sub(&g_memory_stats.current_usage, header->size);
    
    UQM_LOG_TRACE("Freed %zu bytes at %p (%s:%d, originally allocated at %s:%d)", 
                  header->size, ptr, file, line, header->file, header->line);
    
    // Clear magic to detect double-free
    header->magic = 0;
    
    // Free the block
    free(header);
    
    return UQM_OK_VOID();
}

/* ========================================================================== */
/*                          DIAGNOSTIC FUNCTIONS                             */
/* ========================================================================== */

void UQM_Memory_PrintStatistics(void) {
    UQM_LOG_INFO("=== Memory Management Statistics ===");
    UQM_LOG_INFO("Total allocated: %zu bytes (%zu allocations)", 
                 atomic_load(&g_memory_stats.total_allocated),
                 atomic_load(&g_memory_stats.allocation_count));
    UQM_LOG_INFO("Total freed: %zu bytes (%zu deallocations)", 
                 atomic_load(&g_memory_stats.total_freed),
                 atomic_load(&g_memory_stats.deallocation_count));
    UQM_LOG_INFO("Current usage: %zu bytes", 
                 atomic_load(&g_memory_stats.current_usage));
    UQM_LOG_INFO("Peak usage: %zu bytes", 
                 atomic_load(&g_memory_stats.peak_usage));
    
    // Pool statistics
    int pool_count = atomic_load(&g_pool_count);
    UQM_LOG_INFO("Active memory pools: %d", pool_count);
    
    for (int i = 0; i < UQM_MAX_MEMORY_POOLS; i++) {
        UQM_MemoryPool* pool = &g_memory_pools[i];
        if (pool->is_initialized) {
            UQM_LOG_INFO("Pool '%s': %zu/%zu blocks used, %zu bytes peak", 
                         pool->name, pool->used_blocks, pool->total_blocks,
                         atomic_load(&pool->peak_usage));
        }
    }
}

void UQM_Memory_CheckLeaks(void) {
    pthread_mutex_lock(&g_leak_mutex);
    
    if (!g_leak_detection_list) {
        UQM_LOG_INFO("No memory leaks detected");
        pthread_mutex_unlock(&g_leak_mutex);
        return;
    }
    
    UQM_LOG_WARNING("=== Memory Leaks Detected ===");
    
    size_t leak_count = 0;
    size_t total_leaked = 0;
    
    UQM_MemoryBlockHeader* current = g_leak_detection_list;
    while (current) {
        leak_count++;
        total_leaked += current->size;
        
        UQM_LOG_ERROR("Leak %zu: %zu bytes allocated at %s:%d", 
                      leak_count, current->size, current->file, current->line);
        
        current = current->next;
    }
    
    UQM_LOG_ERROR("Total leaks: %zu blocks, %zu bytes", leak_count, total_leaked);
    
    pthread_mutex_unlock(&g_leak_mutex);
}

UQM_Result(void*, UQM_MemoryError) UQM_Memory_Cleanup(void) {
    if (!atomic_load(&g_memory_initialized)) {
        return UQM_OK_VOID();
    }
    
    UQM_LOG_INFO("Cleaning up memory management system");
    
    // Check for leaks
    UQM_Memory_CheckLeaks();
    
    // Print final statistics
    UQM_Memory_PrintStatistics();
    
    // Destroy all pools
    for (int i = 0; i < UQM_MAX_MEMORY_POOLS; i++) {
        if (g_memory_pools[i].is_initialized) {
            UQM_DestroyMemoryPool(&g_memory_pools[i]);
        }
    }
    
    atomic_store(&g_memory_initialized, false);
    
    UQM_LOG_INFO("Memory management system cleanup completed");
    
    return UQM_OK_VOID();
}