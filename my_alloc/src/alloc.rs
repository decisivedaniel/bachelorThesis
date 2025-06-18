use bitfield_struct::bitfield;

extern "C" {
    fn exit(n: libc::c_int) -> !;
    fn memcpy(
        dest: *mut libc::c_void,
        src: *const libc::c_void,
        n: libc::size_t
    ) -> *mut libc::c_void;
    fn memset(
        s: *mut libc::c_void,
        c: libc::c_int,
        n: libc::size_t
    ) -> *mut libc::c_void;
    fn brk(__addr: *mut libc::c_void) -> libc::c_int;
    fn sbrk(__delta: intptr_t) -> *mut libc::c_void;
}

type size_t = libc::c_ulong;
type __intptr_t = libc::c_long;
type intptr_t = __intptr_t;

#[bitfield(u64)]
struct MemoryDescriptor {
    #[bits(1)]
    in_use : bool,
    #[bits(63)]
    length_to_next: usize
} type memDesc_t = MemoryDescriptor;

static mut CURRENT_MEMORY_SIZE: usize = 0;
static mut CURRENT_FREE_MEMORY: usize = 0;
static MEMORY_INCREMENT: usize = 4096;
static mut HEAP_START: *mut libc::c_void = 0 as *mut libc::c_void;
static mut HEAP_END: *mut libc::c_void = 0 as *mut libc::c_void;
static mut LAST_MEM_POS: *mut memDesc_t = 0 as *mut memDesc_t;
static mut LAST_FREE_POS: *mut memDesc_t = 0 as *mut memDesc_t;


unsafe fn cal_mem_needed(data_size: usize) -> usize {
    let mut needed_space: usize = data_size
        .wrapping_div(size_of::<usize>())
        .wrapping_mul(size_of::<usize>());
    needed_space = (needed_space)
        .wrapping_add(size_of::<memDesc_t>());
    if data_size.wrapping_rem(size_of::<size_t>()) > 0
    {
        needed_space = (needed_space)
            .wrapping_add(size_of::<size_t>());
    }
    return needed_space;
}

unsafe fn get_next(curr: *mut memDesc_t) -> *mut memDesc_t {
    return (curr as *mut libc::c_void)
        .offset((*curr).length_to_next() as isize)
        as *mut memDesc_t;
}

unsafe fn find_freed_space(current: *mut memDesc_t, needed_space: usize) -> *mut memDesc_t {
    if current >= HEAP_END as *mut memDesc_t {
        return current;
    }
    if !(*current).in_use() && (*current).length_to_next() >= needed_space
    {
        return current;
    }
    return find_freed_space(get_next(current), needed_space);
}

unsafe fn add_free_section(to_free: *mut memDesc_t) {
    if LAST_FREE_POS.is_null() {
        let ref mut fresh0 = *to_free.offset(1 as libc::c_int as isize);
        (*fresh0).set_length_to_next(0 as usize);
    } else {
        let ref mut fresh1 = *to_free.offset(1 as libc::c_int as isize);
        (*fresh1)
            .set_length_to_next(
                (LAST_FREE_POS as *mut libc::c_void).offset_from(HEAP_START)
                    as libc::c_long as usize,
            );
    }
    LAST_FREE_POS = to_free;
}

unsafe fn get_next_free(curr: *mut memDesc_t) -> *mut memDesc_t {
    return HEAP_START
        .offset((*curr.offset(1 as libc::c_int as isize)).length_to_next() as isize)
        as *mut memDesc_t;
}

unsafe fn remove_free(curr: *mut memDesc_t, free_to_rm: *mut memDesc_t) {
    let next: *mut memDesc_t = get_next_free(curr);
    if curr == free_to_rm {
        if next.is_null() {
            LAST_FREE_POS = 0 as *mut memDesc_t;
        } else {
            LAST_FREE_POS = next;
        }
        return;
    }
    if next == free_to_rm {
        let ref mut fresh2 = *curr.offset(1 as libc::c_int as isize);
        (*fresh2)
            .set_length_to_next((*next.offset(1 as libc::c_int as isize)).length_to_next());
        return;
    } else {
        return remove_free(get_next_free(curr), free_to_rm)
    };
}

unsafe fn new_find_freed(current: *mut memDesc_t, needed_space: usize) -> *mut memDesc_t {
    if current.is_null() {
        return HEAP_END as *mut memDesc_t;
    }
    if (*current).length_to_next() >= needed_space {
        return current;
    }
    if (*current.offset(1 as libc::c_int as isize)).length_to_next() == 0 {
        return HEAP_END as *mut memDesc_t;
    }
    return new_find_freed(get_next_free(current), needed_space);
}

#[no_mangle]
pub unsafe extern "C" fn mymalloc(mut size: size_t) -> *mut libc::c_void {
    if size == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    if HEAP_START.is_null() {
        HEAP_START = sbrk(0 as libc::c_int as intptr_t);
        HEAP_END = HEAP_START;
        sbrk(MEMORY_INCREMENT as intptr_t);
        CURRENT_FREE_MEMORY = MEMORY_INCREMENT;
        CURRENT_MEMORY_SIZE = MEMORY_INCREMENT;
        LAST_MEM_POS = HEAP_START as *mut memDesc_t;
    }
    let mut neededSpace: usize = cal_mem_needed(size as usize);
    let mut currMemPlace: *mut memDesc_t = 0 as *mut memDesc_t;
    currMemPlace = new_find_freed(LAST_FREE_POS, neededSpace);
    if currMemPlace < HEAP_END as *mut memDesc_t {
        remove_free(LAST_FREE_POS, currMemPlace);
        (*currMemPlace).set_in_use(true);
        if neededSpace < (*currMemPlace).length_to_next() {
            let mut nextLocation: *mut memDesc_t = get_next(currMemPlace);
            (*currMemPlace).set_length_to_next(neededSpace);
            let mut newLocation: *mut memDesc_t = get_next(currMemPlace);
            (*newLocation).set_in_use(false);
            (*newLocation)
                .set_length_to_next(
                    (nextLocation as *mut libc::c_void)
                        .offset_from(newLocation as *mut libc::c_void) as libc::c_long
                        as usize,
                );
            add_free_section(newLocation);
        }
        return currMemPlace.offset(1 as libc::c_int as isize) as *mut libc::c_void;
    }
    if neededSpace > CURRENT_FREE_MEMORY {
        let mut increaseMemory: usize = neededSpace
            .wrapping_div(MEMORY_INCREMENT)
            .wrapping_add(1 as libc::c_int as usize)
            .wrapping_mul(MEMORY_INCREMENT);
        if sbrk(increaseMemory as intptr_t) == -(1 as libc::c_int) as *mut libc::c_void {
            exit(1 as libc::c_int);
        }
        CURRENT_MEMORY_SIZE = (CURRENT_MEMORY_SIZE as libc::c_ulong)
            .wrapping_add(increaseMemory.try_into().unwrap()) as usize;
        CURRENT_FREE_MEMORY = (CURRENT_FREE_MEMORY as libc::c_ulong)
            .wrapping_add(increaseMemory.try_into().unwrap()) as usize;
    }
    let mut newMemoryMeta: *mut memDesc_t = HEAP_END as *mut memDesc_t;
    (*newMemoryMeta).set_in_use(true);
    (*newMemoryMeta).set_length_to_next(neededSpace);
    CURRENT_FREE_MEMORY = (CURRENT_FREE_MEMORY as libc::c_ulong)
        .wrapping_sub((*newMemoryMeta).length_to_next().try_into().unwrap()) as usize;
    HEAP_END = HEAP_END.offset((*newMemoryMeta).length_to_next() as isize);
    LAST_MEM_POS = newMemoryMeta;
    return newMemoryMeta.offset(1 as libc::c_int as isize) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn mycalloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    if nmemb == 0 as libc::c_int as libc::c_ulong
        || size == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as *mut libc::c_void;
    }
    if nmemb.wrapping_mul(size) > 2147483647 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    let mut newMemory: *mut libc::c_void = mymalloc(nmemb.wrapping_mul(size));
    memset(newMemory, 0 as libc::c_int, nmemb.wrapping_mul(size) as libc::size_t);
    return newMemory;
}
#[no_mangle]
pub unsafe extern "C" fn mergeFreeSections() {
    let mut currMem: *mut memDesc_t = find_freed_space(HEAP_START as *mut memDesc_t, 0);
    while currMem != HEAP_END as *mut memDesc_t {
        let mut nextMem: *mut memDesc_t = get_next(currMem);
        while nextMem as *mut libc::c_void <= HEAP_END
            && !(*nextMem).in_use() 
        {
            (*currMem)
                .set_length_to_next(
                    (*currMem).length_to_next() + (*nextMem).length_to_next(),
                );
            remove_free(LAST_FREE_POS, nextMem);
            if LAST_MEM_POS == nextMem {
                LAST_MEM_POS = currMem;
            }
            nextMem = get_next(nextMem);
        }
        currMem = find_freed_space(get_next(currMem), 0);
    }
}

unsafe fn unmap() {
    let mut overfilled_chunks: usize = (*LAST_MEM_POS)
        .length_to_next()
        .wrapping_div(MEMORY_INCREMENT);
    overfilled_chunks = overfilled_chunks.wrapping_sub(1);
    (*LAST_MEM_POS).set_length_to_next(
            (*LAST_MEM_POS).length_to_next() - overfilled_chunks.wrapping_mul(MEMORY_INCREMENT));
    HEAP_END = get_next(LAST_MEM_POS) as *mut libc::c_void;
    if brk(HEAP_END) == -(1 as libc::c_int) {
        exit(1 as libc::c_int);
    }
    CURRENT_FREE_MEMORY = 0;
}

#[no_mangle]
pub unsafe extern "C" fn myfree(mut ptr: *mut libc::c_void) {
    if ptr.is_null() {
        return;
    }
    let mut toFree: *mut memDesc_t = ptr
        .offset(-(::core::mem::size_of::<memDesc_t>() as libc::c_ulong as isize))
        as *mut memDesc_t;
    (*toFree).set_in_use(false);
    add_free_section(toFree);
    if !(*LAST_MEM_POS).in_use() && 
    (*LAST_MEM_POS).length_to_next().wrapping_div(MEMORY_INCREMENT) > 4 
    {
        unmap();
    }
}

#[no_mangle]
pub unsafe extern "C" fn myrealloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    if ptr.is_null() {
        return mymalloc(size);
    }
    if size == 0 as libc::c_int as libc::c_ulong {
        myfree(ptr);
        return 0 as *mut libc::c_void;
    }
    let mut currMem: *mut memDesc_t = ptr
        .offset(-(::core::mem::size_of::<memDesc_t>() as libc::c_ulong as isize))
        as *mut memDesc_t;
    let mut neededSpace: usize = cal_mem_needed(size as usize);
    if neededSpace <= (*currMem).length_to_next() {
        return ptr
    } else {
        let mut newLocation: *mut libc::c_void = mymalloc(size);
        memcpy(
            newLocation,
            ptr,
            (*currMem)
                .length_to_next()
                .wrapping_sub(::core::mem::size_of::<memDesc_t>()) as libc::size_t,
        );
        myfree(ptr);
        return newLocation;
    };
}
