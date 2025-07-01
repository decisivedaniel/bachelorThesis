
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
    return ((int64_t) now.tv_sec) * 1000 + ((int64_t) now.tv_nsec) / 1000000;
}

int main() {
    int64_t start = what_time_is_it();
    struct Tree* tree = treeNew();
    for(int i = 0; i < 10000; i++) {
        insert(tree, i);
    }
    for(int i = 0; i < 10000; i++) {
        contains(tree->root, i);
    }
    printf("time took %f", what_time_is_it() - start);

    freeTree(tree);
    
}
