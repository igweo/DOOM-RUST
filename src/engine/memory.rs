use std::cell::RefCell;
use std::rc::Rc;

enum PurgeTag {
    // Tags < 100 are not overwritten until freed
    PuStatic = 1,
    PuSound = 2,
    PuMusic = 3,
    PuLevel = 50,
    PuLevlSpec = 51,
    // Tags >= 100 are purged when memory is needed
    PuPurgeLevel = 100,
    PuCache = 101,
}

impl TryFrom<u8> for PurgeTag {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(PurgeTag::PuStatic),
            2 => Ok(PurgeTag::PuSound),
            3 => Ok(PurgeTag::PuMusic),
            50 => Ok(PurgeTag::PuLevel),
            51 => Ok(PurgeTag::PuLevlSpec),
            100 => Ok(PurgeTag::PuPurgeLevel),
            101 => Ok(PurgeTag::PuCache),
            _ => Err(()),
        }
    }
}

/// Represents a block of memory in the custom memory allocator.
///
/// This struct is used as a node in a doubly linked list (DLL) to manage memory blocks.
/// Each block contains a fixed-size array of data (`data`), and links to the previous
/// and next blocks in the list (`prev` and `next`). The `BlockMetaData` stores important
/// information about the block's size and its classification (`tag`), which helps the allocator
/// manage the block's state (whether it's in use or free, and what it's used for).
///
/// As blocks are allocated and freed, the DLL is updated accordingly, with blocks being split
/// into smaller blocks or merged back together to optimize memory usage. The memory allocator
/// is responsible for updating the `size` and `tag` fields in the `BlockMetaData` to reflect
/// the current state of each block. The `data` field holds the actual memory content, which will
/// be used for allocation requests.
struct Block {
    next: Rc<RefCell<Option<Block>>>, // Pointer to the next block in the doubly linked list.
    prev: Rc<RefCell<Option<Block>>>, // Pointer to the previous block in the doubly linked list.
    metadata: BlockMetaData,          // Metadata holding the block's tag and size.
    pub data: [u8; 64],               // The actual memory content of the block (fixed size).
}

/// TODO
impl Block {
    fn new() -> Self {
        const SIZE: u8 = 64;
        Self {
            next: Rc::new(RefCell::new(None)),
            prev: Rc::new(RefCell::new(None)),
            metadata: BlockMetaData {
                tag: None,
                size: SIZE,
            },
            data: [0; SIZE as usize],
        }
    }
    fn size(&self) -> u8 {
        self.metadata.size
    }
}

struct BlockMetaData {
    tag: Option<PurgeTag>, // The tag is used for classification (e.g. PU_STATIC, PU_PURGELEVEL)
    size: u8,              // The size of the current block (in bytes)
}

impl BlockMetaData {
    fn new(tag: Option<PurgeTag>, size: u8) -> Self {
        Self { tag, size }
    }
}

// TODO: Memory Allocation Strategy
//
// Implement a memory allocation strategy for the `Block` struct, ensuring the proper management
// of the doubly linked list of blocks. Each block will be allocated or freed as needed
// based on memory requests. When a block is allocated, we may need to split the block into smaller
// blocks to accommodate the requested memory size. When a block is freed, it will be returned to
// the free list and merged with adjacent free blocks if possible to avoid fragmentation.
//
// Considered adding the following features:
// - Implement block splitting and merging to efficiently manage memory and reduce fragmentation.
// - Introduce a free list for faster lookup of available blocks, if needed.
// - Implement a strategy for purging blocks (e.g., based on the `PU_PURGELEVEL` tag).
// - Implement unit tests for allocation, freeing, and block merging/splitting to ensure correctness.
struct MemoryAllocator;

impl MemoryAllocator {
    fn init() -> Block {
        Block::new()
    }

    //TODO: Implement a strategy to minimize fragmenetation
    pub fn allocate(block: &Block, size: u8) -> Block {
        // Firstly traverse the DLL to find a FREE Block with adequate room
        // While traversing, keep track of any Blocks of adequate size with a `tag` over 100
        // If no free blocks, then call the `purge function` on the Block(s) with a `tag` over 100
        // Merge Blocks if needed
        // Return the newly allocated block
        Block::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_allocator() {
        let block = MemoryAllocator::init();
        assert_eq!(block.size(), 64);
    }

    #[test]
    fn test_allocator_with_varying_allocation_sizes() {
        // asserts that the block sizes are what they should be
        let block = MemoryAllocator::init();
        assert_eq!(
            block.size(),
            64,
            "We are testing that the initial block size {} is equal to {} bytes",
            block.size(),
            64
        );
        let block_2 = MemoryAllocator::allocate(&block, 16);
        assert_eq!(block.size(), 48);
        assert_eq!(block_2.size(), 16);
        let block_3 = MemoryAllocator::allocate(&block, 8);
        assert_eq!(block.size(), 40);
        assert_eq!(block_3.size(), 8);
        // null <- block -> block_2 -> block_3
    }

    #[test]
    fn test_block_no_overlap() {
        let block = MemoryAllocator::init();
        let block_2 = MemoryAllocator::allocate(&block, 16);
        // assert that the address of the last element of the first block is less than the first
        // element of the second block
        let last_ele_first_block = &block.data[block.data.len() - 1];
        let first_ele_last_block = &block.data[0];

        let first_ptr = last_ele_first_block as *const u8;
        let last_ptr = first_ele_last_block as *const u8;
        assert!(first_ptr < last_ptr);
    }
}
