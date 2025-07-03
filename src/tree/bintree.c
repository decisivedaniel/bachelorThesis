
#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <inttypes.h>

typedef struct Node t_node;

struct Node {
    int value;
    t_node* left;
    t_node* right;
    t_node* parent;
};

void freeNode(t_node* node) {
    if (node == NULL) return;
    freeNode(node->left);
    freeNode(node->right);
    free(node);
}

struct Tree {
    t_node* root;
};

struct Tree* treeNew() {
    struct Tree* new = (struct Tree*)malloc(sizeof(struct Tree));
    new->root = NULL;
}

void freeTree(struct Tree* tree) {
    if(tree->root) {
        freeNode(tree->root);
    }
    free(tree);
}


void inorderWalk(t_node* node);
int contains(t_node* node, const int lookValue);
t_node* find(t_node* node, const int lookValue);
t_node* min(struct Tree* tree);
t_node* max(t_node* node);
t_node* successor(t_node* node);
void insert(struct Tree* tree, int add);
void transplant(struct Tree* tree, t_node* u, t_node* v);
void delete(struct Tree* tree, void* value);

void inorderWalk(t_node* node) {
    struct Tree* tree = treeNew();
    if (node != NULL) {
        inorderWalk(node->left);
        printf("%d",node->value);
        inorderWalk(node->right);
    }
}

t_node* find(t_node* node, const int lookValue) {
    if (node == NULL || node->value == lookValue) {
        return node;
    }
    if (lookValue < node->value) {
        return find(node->left, lookValue);
    }
    return find(node->right, lookValue);
}

int contains(t_node* node, const int lookValue) {
    t_node* found = find(node, lookValue);
    if (found == NULL) {
        return 0;
    }
    return 1;
}

t_node* min(struct Tree* tree) {
    t_node* node = tree->root;
    if (node == NULL) { return node; }
    while (node->left != NULL) {
        node = node->left;
    }
    return node;
}

t_node* max(t_node* node) {
    if (node == NULL) { return node; }
    while (node->right != NULL) {
        node = node->right;
    }
    return node;
}

// t_node* successor(t_node* node) {
//     if (node == NULL) { return node; }
//     if (node->right != NULL) {
//         return min(node->right);
//     }
//     t_node* parent = node->parent;
//     while (parent != NULL && treeEqual(node, parent->right)) {
//         node = parent;
//         parent = parent->parent;
//     }
//     return parent;
// }

void insert(struct Tree* tree, int add) {
    t_node* previous = NULL;
    t_node* current = tree->root;
    while (current != NULL) {
        previous = current;
        if (add == current->value) { return; }
        else if (add < current->value) {
            current = current->left;
        } else {
            current = current->right;
        }
    }
    t_node* newNode = malloc(sizeof(t_node));
    newNode->left = NULL;
    newNode->right = NULL;
    newNode->value = add;
    newNode->parent = previous;
    if (previous == NULL) {
        tree->root = newNode;
    } else if (newNode->value < previous->value) {
        previous->left = newNode;
    } else {
        previous->right = newNode;
    }
}

// void transplant(struct Tree* tree, t_node* remove, t_node* replacement) {
//     if (remove->parent == NULL) {
//         tree->root = replacement;
//     } else if (remove == remove->parent->left) {
//         remove->parent->left = replacement;
//     } else {
//         remove->parent->right = replacement;
//     }
//     if (replacement != NULL) {
//         replacement->parent = remove->parent;
//     } 
// }

// void delete(struct Tree* tree, void* value) {
//     t_node* toDelete = find(tree->root, value);
//     if (toDelete == NULL) {return;}
//     if (toDelete->left == NULL) {
//         transplant(tree, toDelete, toDelete->right);
//     } else if (toDelete->right == NULL) {
//         transplant(tree, toDelete, toDelete->left);
//     } else {
//         t_node* nextMin = min(toDelete->right);
//         if (treeEqual(nextMin->parent, toDelete) != 0) {
//             transplant(tree, nextMin, nextMin->right);
//             nextMin->right = toDelete->right;
//             nextMin->right->parent = nextMin;
//         }
//         transplant(tree, toDelete, nextMin);
//         nextMin->left = toDelete->left;
//         nextMin->left->parent = nextMin;
//     }
// }


double what_time_is_it()
{
    struct timespec now;
    timespec_get(&now, TIME_UTC);
    //clock_gettime(CLOCK_REALTIME, &now);
    return ((int64_t) now.tv_sec) + ((int64_t) now.tv_nsec) / 1000000000.0;
}

struct Tree* create_test(int* list, int max) {
    struct Tree* tree = treeNew();
    for(int i = 0; i < max; i++) {
        insert(tree, list[i]);
    }
    return tree;
}

void readTree(struct Tree* tree, int max) {
    for(int i = 0; i < max; i++) {
        contains(tree->root, i);
    }
}

void optimizedList(int* list, int* currentPosition, int lower, int upper) {
    if(lower == upper) {return;}
    int mid = ((upper - lower) / 2) + lower;
    list[*currentPosition] = mid;
    *currentPosition = *currentPosition + 1;
    optimizedList(list,currentPosition,lower,mid);
    optimizedList(list,currentPosition,mid+1,upper);
}

int* createOptimizedList(int max) {
    int* list = malloc(sizeof(int) * max);
    int currentPos = 0;
    optimizedList(list, &currentPos, 0, max);
    return list;
}

int* createUnoptimizedList(int max) {
    int* list = malloc(sizeof(int) * max);
    for(int i = 0; i<max; i++) {
        list[i] = i;
    }
    return list;
}

int main() {
    int counts[5] = {1024,2048,4096,8192,16384};
    int numberOfRuns = 10;
    for (int count = 0; count < 5; count++) {
        printf("%d - ", counts[count]);
        double* results = calloc(sizeof(double), numberOfRuns*4);
        for (int run = 0; run < numberOfRuns; run++) {
            int* optimized = createOptimizedList(counts[count]);
            double start = what_time_is_it();
            struct Tree* tree = create_test(optimized, counts[count]);
            results[run] = what_time_is_it() - start;
            double read = what_time_is_it();
            readTree(tree,counts[count]);
            results[run+numberOfRuns] = what_time_is_it() - read;
            freeTree(tree);
            free(optimized);

            int* worst = createUnoptimizedList(counts[count]);
            start = what_time_is_it();
            struct Tree* worstTree = create_test(worst, counts[count]);
            results[run+(numberOfRuns*2)] = what_time_is_it() - start;
            read = what_time_is_it();
            readTree(worstTree,counts[count]);
            results[run+(numberOfRuns*3)] = what_time_is_it() - read;
            freeTree(worstTree);
            free(worst);
        }  
        for(int result = 0; result < numberOfRuns*4; result++) {
            printf("%f\\\\", results[result]);
            if((result+1)%numberOfRuns == 0) {
                printf("\n");
            }
        }
        free(results);
        printf("\n\n");
    }  
}
