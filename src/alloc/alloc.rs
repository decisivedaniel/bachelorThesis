extern "C" {
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn brk(__addr: *mut libc::c_void) -> libc::c_int;
    fn sbrk(__delta: intptr_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __intptr_t = libc::c_long;
pub type intptr_t = __intptr_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct MemoryDescriptor {
    #[bitfield(name = "inUse", ty = "size_t", bits = "0..=0")]
    #[bitfield(name = "lengthToNext", ty = "size_t", bits = "1..=63")]
    pub inUse_lengthToNext: [u8; 8],
}
pub type memDesc_t = MemoryDescriptor;
#[no_mangle]
pub static mut currentMemorySize: size_t = 0 as libc::c_int as size_t;
#[no_mangle]
pub static mut currentFreeMemory: size_t = 0 as libc::c_int as size_t;
#[no_mangle]
pub static mut memoryIncrement: size_t = 4096 as libc::c_int as size_t;
#[no_mangle]
pub static mut heap_start: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
#[no_mangle]
pub static mut heap_end: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
#[no_mangle]
pub static mut lastMemPosition: *mut memDesc_t = 0 as *const memDesc_t as *mut memDesc_t;
#[no_mangle]
pub static mut lastFreePosition: *mut memDesc_t = 0 as *const memDesc_t
    as *mut memDesc_t;
#[no_mangle]
pub unsafe extern "C" fn calMemNeeded(mut dataSize: size_t) -> size_t {
    let mut neededSpace: size_t = dataSize
        .wrapping_div(::core::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong);
    neededSpace = (neededSpace as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<memDesc_t>() as libc::c_ulong) as size_t
        as size_t;
    if dataSize.wrapping_rem(::core::mem::size_of::<size_t>() as libc::c_ulong)
        > 0 as libc::c_int as libc::c_ulong
    {
        neededSpace = (neededSpace as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<size_t>() as libc::c_ulong) as size_t
            as size_t;
    }
    return neededSpace;
}
#[no_mangle]
pub unsafe extern "C" fn getNext(mut curr: *mut memDesc_t) -> *mut memDesc_t {
    return (curr as *mut libc::c_void).offset((*curr).lengthToNext() as isize)
        as *mut memDesc_t;
}
#[no_mangle]
pub unsafe extern "C" fn findFreedSpace(
    mut current: *mut memDesc_t,
    mut neededSpace: size_t,
) -> *mut memDesc_t {
    if current >= heap_end as *mut memDesc_t {
        return current;
    }
    if (*current).inUse() as libc::c_int == 0 as libc::c_int
        && (*current).lengthToNext() >= neededSpace
    {
        return current;
    }
    return findFreedSpace(getNext(current), neededSpace);
}
#[no_mangle]
pub unsafe extern "C" fn addFreeSection(mut toFree: *mut memDesc_t) {
    if lastFreePosition.is_null() {
        let ref mut fresh0 = *toFree.offset(1 as libc::c_int as isize);
        (*fresh0).set_lengthToNext(0 as libc::c_int as size_t);
    } else {
        let ref mut fresh1 = *toFree.offset(1 as libc::c_int as isize);
        (*fresh1)
            .set_lengthToNext(
                (lastFreePosition as *mut libc::c_void).offset_from(heap_start)
                    as libc::c_long as size_t,
            );
    }
    lastFreePosition = toFree;
}
#[no_mangle]
pub unsafe extern "C" fn getNextFree(mut curr: *mut memDesc_t) -> *mut memDesc_t {
    return heap_start
        .offset((*curr.offset(1 as libc::c_int as isize)).lengthToNext() as isize)
        as *mut memDesc_t;
}
#[no_mangle]
pub unsafe extern "C" fn removeFree(
    mut curr: *mut memDesc_t,
    mut freeToRm: *mut memDesc_t,
) {
    let mut next: *mut memDesc_t = getNextFree(curr);
    if curr == freeToRm {
        if next.is_null() {
            lastFreePosition = 0 as *mut memDesc_t;
        } else {
            lastFreePosition = next;
        }
        return;
    }
    if next == freeToRm {
        let ref mut fresh2 = *curr.offset(1 as libc::c_int as isize);
        (*fresh2)
            .set_lengthToNext((*next.offset(1 as libc::c_int as isize)).lengthToNext());
        return;
    } else {
        return removeFree(getNextFree(curr), freeToRm)
    };
}
#[no_mangle]
pub unsafe extern "C" fn newFindFreed(
    mut current: *mut memDesc_t,
    mut neededSpace: size_t,
) -> *mut memDesc_t {
    if current.is_null() {
        return heap_end as *mut memDesc_t;
    }
    if (*current).lengthToNext() >= neededSpace {
        return current;
    }
    if (*current.offset(1 as libc::c_int as isize)).lengthToNext()
        == 0 as libc::c_int as libc::c_ulong
    {
        return heap_end as *mut memDesc_t;
    }
    return newFindFreed(getNextFree(current), neededSpace);
}
#[no_mangle]
pub unsafe extern "C" fn mymalloc(mut size: size_t) -> *mut libc::c_void {
    if size == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    if heap_start.is_null() {
        heap_start = sbrk(0 as libc::c_int as intptr_t);
        heap_end = heap_start;
        sbrk(memoryIncrement as intptr_t);
        currentFreeMemory = memoryIncrement;
        currentMemorySize = memoryIncrement;
        lastMemPosition = heap_start as *mut memDesc_t;
    }
    let mut neededSpace: size_t = calMemNeeded(size);
    let mut currMemPlace: *mut memDesc_t = 0 as *mut memDesc_t;
    currMemPlace = newFindFreed(lastFreePosition, neededSpace);
    if currMemPlace < heap_end as *mut memDesc_t {
        removeFree(lastFreePosition, currMemPlace);
        (*currMemPlace).set_inUse(1 as libc::c_int as size_t);
        if neededSpace < (*currMemPlace).lengthToNext() {
            let mut nextLocation: *mut memDesc_t = getNext(currMemPlace);
            (*currMemPlace).set_lengthToNext(neededSpace);
            let mut newLocation: *mut memDesc_t = getNext(currMemPlace);
            (*newLocation).set_inUse(0 as libc::c_int as size_t);
            (*newLocation)
                .set_lengthToNext(
                    (nextLocation as *mut libc::c_void)
                        .offset_from(newLocation as *mut libc::c_void) as libc::c_long
                        as size_t,
                );
            addFreeSection(newLocation);
        }
        return currMemPlace.offset(1 as libc::c_int as isize) as *mut libc::c_void;
    }
    if neededSpace > currentFreeMemory {
        let mut increaseMemory: size_t = neededSpace
            .wrapping_div(memoryIncrement)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(memoryIncrement);
        if sbrk(increaseMemory as intptr_t) == -(1 as libc::c_int) as *mut libc::c_void {
            exit(1 as libc::c_int);
        }
        currentMemorySize = (currentMemorySize as libc::c_ulong)
            .wrapping_add(increaseMemory) as size_t as size_t;
        currentFreeMemory = (currentFreeMemory as libc::c_ulong)
            .wrapping_add(increaseMemory) as size_t as size_t;
    }
    let mut newMemoryMeta: *mut memDesc_t = heap_end as *mut memDesc_t;
    (*newMemoryMeta).set_inUse(1 as libc::c_int as size_t);
    (*newMemoryMeta).set_lengthToNext(neededSpace);
    currentFreeMemory = (currentFreeMemory as libc::c_ulong)
        .wrapping_sub((*newMemoryMeta).lengthToNext()) as size_t as size_t;
    heap_end = heap_end.offset((*newMemoryMeta).lengthToNext() as isize);
    lastMemPosition = newMemoryMeta;
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
    memset(newMemory, 0 as libc::c_int, nmemb.wrapping_mul(size));
    return newMemory;
}
#[no_mangle]
pub unsafe extern "C" fn mergeFreeSections() {
    let mut currMem: *mut memDesc_t = findFreedSpace(
        heap_start as *mut memDesc_t,
        0 as libc::c_int as size_t,
    );
    while currMem != heap_end as *mut memDesc_t {
        let mut nextMem: *mut memDesc_t = getNext(currMem);
        while nextMem as *mut libc::c_void <= heap_end
            && (*nextMem).inUse() as libc::c_int == 0 as libc::c_int
        {
            (*currMem)
                .set_lengthToNext(
                    (*currMem).lengthToNext() + (*nextMem).lengthToNext() as size_t,
                );
            removeFree(lastFreePosition, nextMem);
            if lastMemPosition == nextMem {
                lastMemPosition = currMem;
            }
            nextMem = getNext(nextMem);
        }
        currMem = findFreedSpace(getNext(currMem), 0 as libc::c_int as size_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn unmap() {
    let mut overfilledChunks: size_t = (*lastMemPosition)
        .lengthToNext()
        .wrapping_div(memoryIncrement);
    overfilledChunks = (overfilledChunks as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
    (*lastMemPosition)
        .set_lengthToNext(
            (*lastMemPosition).lengthToNext()
                - overfilledChunks.wrapping_mul(memoryIncrement) as size_t,
        );
    heap_end = getNext(lastMemPosition) as *mut libc::c_void;
    if brk(heap_end) == -(1 as libc::c_int) {
        exit(1 as libc::c_int);
    }
    currentFreeMemory = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn myfree(mut ptr: *mut libc::c_void) {
    if ptr.is_null() {
        return;
    }
    let mut toFree: *mut memDesc_t = ptr
        .offset(-(::core::mem::size_of::<memDesc_t>() as libc::c_ulong as isize))
        as *mut memDesc_t;
    (*toFree).set_inUse(0 as libc::c_int as size_t);
    addFreeSection(toFree);
    if (*lastMemPosition).inUse() as libc::c_int == 0 as libc::c_int
        && (*lastMemPosition).lengthToNext().wrapping_div(memoryIncrement)
            > 4 as libc::c_int as libc::c_ulong
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
    let mut neededSpace: size_t = calMemNeeded(size);
    if neededSpace <= (*currMem).lengthToNext() {
        return ptr
    } else {
        let mut newLocation: *mut libc::c_void = mymalloc(size);
        memcpy(
            newLocation,
            ptr,
            (*currMem)
                .lengthToNext()
                .wrapping_sub(::core::mem::size_of::<memDesc_t>() as libc::c_ulong),
        );
        myfree(ptr);
        return newLocation;
    };
}
