use std::cell::RefCell;
use std::rc::Rc;

//Purge tags
//Tags < 100 are not overwritten until freed
const PU_STATIC: u8 = 1;
const PU_SOUND: u8 = 2;
const PU_MUSIC: u8 = 3;
const PU_LEVEL: u8 = 50;
const PU_LEVSPEC: u8 = 51;
// Tags >= 100 are purged when memory is needed
const PU_PURGELEVEL: u8 = 100;
const PU_CACHE: u8 = 101;

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
///
/// Fields:
/// - `next`: A reference to the next block in the doubly linked list (DLL). This allows traversal
///   of the list in the forward direction. Uses `Rc<RefCell<Option<Block>>>` to allow shared mutability.
///
/// - `prev`: A reference to the previous block in the doubly linked list (DLL). This allows traversal
///   of the list in the reverse direction. Uses `Rc<RefCell<Option<Block>>>` for shared mutability.
///
/// - `metadata`: Metadata associated with the block, including the tag(classification) and size.
///   The `metadata` helps the allocator manage and classify the block.

struct Block {
    next: Rc<RefCell<Option<Block>>>, // Pointer to the next block in the doubly linked list.
    prev: Rc<RefCell<Option<Block>>>, // Pointer to the previous block in the doubly linked list.
    metadata: BlockMetaData,          // Metadata holding the block's tag and size.
    data: [u8; 64],                   // The actual memory content of the block (fixed size).
}

impl Block {
    fn new(metadata: BlockMetaData) -> Self {
        Self {
            next: Rc::new(RefCell::new(None)),
            prev: Rc::new(RefCell::new(None)),
            metadata,
            data: [0; 64],
        }
    }
}
/// Metadata associated with a memory block.
///
/// This struct holds information about the state of a memory block, including its size,
/// allocation status, and any relevant tags. It is used by the custom memory allocator to
/// manage blocks of memory in the doubly linked list. Each block's metadata is updated
/// as the block is allocated, split, or freed.
///
/// Fields:
/// - `tag`: A classification tag for the block. This can be used to categorize memory blocks
///   for different purposes, such as persistent data (`PU_STATIC`), level-specific data (`PU_LEVSPEC`),
///   or memory that can be purged when necessary (`PU_PURGELEVEL`).
///
/// - `size`: The size of the memory block in bytes. This is important for allocation and splitting.
///   The size will decrease when a block is split into smaller blocks and will be updated as
///   the block is reused or freed.
struct BlockMetaData {
    tag: Option<u8>, // The tag is used for classification (e.g. PU_STATIC, PU_PURGELEVEL)
    size: u8,        // The size of the current block (in bytes)
}

impl BlockMetaData {
    fn new(tag: Option<u8>, size: u8) -> Self {
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
