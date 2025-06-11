use libc::{sbrk, size_t, ssize_t};
use std::slice;

struct MemoryDescriptor {
    is_free : bool,
    to_next : u32
}

struct Memory {
    size : u32,
    free_size : u32,
    last_mem_position : u32,
    last_free_position : u32,
    storage : Vec<MemoryDescriptor>
}

impl Memory {
    fn new() -> Memory {
        Memory {
            size: MEMORY_INCREMENT,
            free_size: 0,
            last_mem_position: 0,
            last_free_position: 0,
            storage: Vec::<MemoryDescriptor>::new().reserve(MEMORY_INCREMENT)
        }
    }
}

fn get_memory() -> Memory {
    if
}
static mut CURRENT_MEMORY_SIZE : u32 = 0;
static mut CURRENT_FREE_MEMORY : u32 = 0;
static MEMORY_INCREMENT : u32 = 4096;
static mut STORAGE : Vec<MemoryDescriptor> = Vec::new();
static mut LAST_MEM_POSITION : u32 = 0;
static mut LAST_FREE_POSITION : u32 = 0;
 
fn main() {
    println!("Hello, world!");
    let mut block : Vec<MemoryDescriptor> = Vec::new();
}

#[unsafe(no_mangle)]
pub extern "C" 
fn mymalloc(size : size_t) -> *mut libc::c_void{
    if size == 0 {return 0 as *mut libc::c_void;}
    if unsafe{CURRENT_MEMORY_SIZE == 0} {
        unsafe {
            STORAGE.reserve(MEMORY_INCREMENT.try_into().unwrap());
        }
    }

    unsafe {return *mut STORAGE[0].;}
}