// DEFAULT VALS:

/*

CACHE CONFIGURATION:
    Total cache size
        8192 (8K)
    Block size
        16
    Unified vs. split I- and D-caches
        Unified
    Associativity
        1 (direct mapped)
    Write back vs. write through
        Write back
    Write allocate vs. write no allocate
        Write allocate
*/
const DEFAULT_TOTAL_CACHE_SIZE  :u32 = 8192;
const DEFAULT_BLOCK_SIZE        :u32 = 16;
const DEFAULT_ASSOCIATIVITY     :u32 = 1;
const DEFAULT_WRITE_BACK        :bool = true;
const DEFAULT_WRITE_ALLOCATE    :bool = true;
const DEFAULT_SPLIT_CACHE       :bool = false;
