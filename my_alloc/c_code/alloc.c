#include <limits.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

size_t currentMemorySize = 0;
size_t currentFreeMemory = 0;
size_t memoryIncrement = 4096;
void *heap_start;
void *heap_end;

typedef struct MemoryDescriptor {
    size_t inUse : 1;
    size_t lengthToNext : 63;
} memDesc_t;

memDesc_t *lastMemPosition;
memDesc_t *lastFreePosition;

size_t calMemNeeded(size_t dataSize) {
    size_t neededSpace = (dataSize / sizeof(size_t)) * sizeof(size_t);
    neededSpace += sizeof(memDesc_t);
    if (dataSize % sizeof(size_t) > 0) {
        neededSpace += sizeof(size_t);
    }
    return neededSpace;
}

memDesc_t *getNext(memDesc_t *curr) {
    return (memDesc_t *)(((void *)curr) + curr->lengthToNext);
}

memDesc_t *findFreedSpace(memDesc_t *current, size_t neededSpace) {
    if (current >= (memDesc_t *)heap_end) {
        return current;
    }
    if (current->inUse == 0 && current->lengthToNext >= neededSpace) {
        return current;
    }
    return findFreedSpace(getNext(current), neededSpace);
}

void addFreeSection(memDesc_t *toFree) {
    if (lastFreePosition == NULL) {
        (toFree + 1)->lengthToNext = 0;
    } else {
        (toFree + 1)->lengthToNext = (void *)lastFreePosition - heap_start;
    }
    lastFreePosition = toFree;
}

memDesc_t *getNextFree(memDesc_t *curr) {
    return (memDesc_t *)(heap_start + (curr + 1)->lengthToNext);
}

void removeFree(memDesc_t *curr, memDesc_t *freeToRm) {
    memDesc_t *next = getNextFree(curr);
    if (curr == freeToRm) {
        if (next == 0) {
            lastFreePosition = NULL;
        } else {
            lastFreePosition = next;
        }
        return;
    }
    if (next == freeToRm) {
        (curr + 1)->lengthToNext = (next + 1)->lengthToNext;
        return;
    } else {
        return removeFree(getNextFree(curr), freeToRm);
    }
}

memDesc_t *newFindFreed(memDesc_t *current, size_t neededSpace) {
    if (current == NULL) {
        return heap_end;
    }
    if (current ->lengthToNext >= neededSpace) {
        return current;
    }
    if ((current+1)->lengthToNext == 0) {
        return heap_end;
    }
    return newFindFreed(getNextFree(current), neededSpace);
}

void *mymalloc(size_t size)
{
    if (size == 0) {
        return NULL;
    }
    if (heap_start == NULL) {
        heap_start = sbrk(0);
        heap_end = heap_start;
        sbrk(memoryIncrement);
        currentFreeMemory = memoryIncrement;
        currentMemorySize = memoryIncrement;
        lastMemPosition = heap_start;
    }
    size_t neededSpace = calMemNeeded(size);
    //reuse current memory
    memDesc_t *currMemPlace;
    currMemPlace = newFindFreed(lastFreePosition, neededSpace);
    if (currMemPlace < (memDesc_t *)heap_end) {
        removeFree(lastFreePosition,currMemPlace);
        currMemPlace->inUse = 1;
        //Split new section to only what is needed
        if(neededSpace < currMemPlace->lengthToNext) {
            memDesc_t *nextLocation = getNext(currMemPlace);
            currMemPlace->lengthToNext = neededSpace;
            memDesc_t *newLocation = getNext(currMemPlace);
            newLocation->inUse = 0;
            newLocation->lengthToNext = (void *)nextLocation - (void *)newLocation;
            addFreeSection(newLocation);
        }
        return (void *)(currMemPlace + 1);
    }
    if (neededSpace > currentFreeMemory) {
        //create new memory sbrk
        size_t increaseMemory = ((neededSpace / memoryIncrement) + 1) * memoryIncrement;
        if (sbrk(increaseMemory) == (void *)-1) {
            exit(1);
        }
        currentMemorySize += increaseMemory;
        currentFreeMemory += increaseMemory;
    }
    memDesc_t *newMemoryMeta = (memDesc_t *)heap_end;
    newMemoryMeta->inUse = 1;
    newMemoryMeta->lengthToNext = neededSpace;
    currentFreeMemory -= newMemoryMeta->lengthToNext;
    heap_end = heap_end + newMemoryMeta->lengthToNext;
    lastMemPosition = newMemoryMeta;
    return (void *)(newMemoryMeta + 1);
}

void *mycalloc(size_t nmemb, size_t size)
{
    // error checking
    if (nmemb == 0 || size == 0) {
        return NULL;
    }
    if (nmemb * size > INT_MAX) {
        return NULL;
    }

    void *newMemory = mymalloc(nmemb * size);
    memset(newMemory, 0, nmemb * size);
    return newMemory;
}

void mergeFreeSections() {
    memDesc_t *currMem = findFreedSpace(heap_start, 0);
    while (currMem != heap_end) {
        memDesc_t *nextMem = getNext(currMem);
        while ((void *)nextMem <= heap_end && nextMem->inUse == 0) {
            currMem->lengthToNext += nextMem->lengthToNext;
            removeFree(lastFreePosition, nextMem);
            if (lastMemPosition == nextMem) {
                lastMemPosition = currMem;
            }
            nextMem = getNext(nextMem);
        }
        currMem = findFreedSpace(getNext(currMem), 0);
    }
}

void unmap() {
    size_t overfilledChunks = lastMemPosition->lengthToNext / memoryIncrement;
    overfilledChunks -= 1;
    lastMemPosition->lengthToNext -= overfilledChunks * memoryIncrement;
    heap_end = getNext(lastMemPosition);
    if (brk(heap_end) == -1) {
        exit(1);
    }
    currentFreeMemory = 0;
}

void myfree(void *ptr)
{
    if (ptr == NULL) {
        return;
    }
    memDesc_t *toFree = (memDesc_t *)(ptr - sizeof(memDesc_t));
    toFree->inUse = 0;
    //add to linked list
    addFreeSection(toFree);
    // merge free sections
    //mergeFreeSections();
    if (lastMemPosition->inUse == 0 && lastMemPosition->lengthToNext / memoryIncrement > 4) {
        unmap();
    }
}

void *myrealloc(void *ptr, size_t size)
{
    if (ptr == NULL) {
        return mymalloc(size);
    }
    if (size == 0) {
        myfree(ptr);
        return NULL;
    }
    memDesc_t *currMem = ptr - sizeof(memDesc_t);
    size_t neededSpace = calMemNeeded(size);
    if (neededSpace <= currMem->lengthToNext) {
        return ptr;
    } else {
        void *newLocation = mymalloc(size);
        memcpy(newLocation, ptr, currMem->lengthToNext - sizeof(memDesc_t));
        myfree(ptr);
        return newLocation;
    }
}


/*
 * Enable the code below to enable system allocator support for your allocator.
 * Doing so will make debugging much harder (e.g., using printf may result in
 * infinite loops).
 */
#if 0
void *malloc(size_t size) { return mymalloc(size); }
void *calloc(size_t nmemb, size_t size) { return mycalloc(nmemb, size); }
void *realloc(void *ptr, size_t size) { return myrealloc(ptr, size); }
void free(void *ptr) { myfree(ptr); }
#endif
