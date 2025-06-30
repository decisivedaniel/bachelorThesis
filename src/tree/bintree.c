
#include <stdlib.h>;

extern int treeLessThan(void* l, void* r);
extern int treeEqual(void* l, void* r);

struct Node {
    void* value;
    t_node* left;
    t_node* right;
    t_node* parent;
} typedef t_node;

struct Tree {
    t_node* root;
};

struct Tree* treeNew() {
    return (struct Tree*)malloc(sizeof(struct Tree));
}

void inorderWalk(t_node* node);
int contains(t_node* node, const void* lookValue);
t_node* find(const t_node* node, const void* lookValue);
t_node* min(struct Tree* tree);
t_node* max(t_node* node);
t_node* successor(t_node* node);
void insert(struct Tree* tree, void* add);
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

t_node* find(const t_node* node, const void* lookValue) {
    if (node == NULL || treeEqual(node->value, lookValue)) {
        return node;
    }
    if (treeLessThan(lookValue, node->value)) {
        return find(node->left, lookValue);
    }
    return find(node->right, lookValue);
}

int contains(t_node* node, const void* lookValue) {
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

t_node* successor(t_node* node) {
    if (node == NULL) { return node; }
    if (node->right != NULL) {
        return min(node->right);
    }
    t_node* parent = node->parent;
    while (parent != NULL && treeEqual(node, parent->right)) {
        node = parent;
        parent = parent->parent;
    }
    return parent;
}

void insert(struct Tree* tree, void* add) {
    t_node* previous = NULL;
    t_node* current = tree->root;
    while (current != NULL) {
        previous = current;
        if (treeEqual(add, current->value)) { return; }
        else if (treeLessThan(add, current->value)) {
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
    } else if (treeLessThan(newNode->value, previous->value)) {
        previous->left = newNode;
    } else {
        previous->right = newNode;
    }
}

void transplant(struct Tree* tree, t_node* remove, t_node* replacement) {
    if (remove->parent == NULL) {
        tree->root = replacement;
    } else if (remove == remove->parent->left) {
        remove->parent->left = replacement;
    } else {
        remove->parent->right = replacement;
    }
    if (replacement != NULL) {
        replacement->parent = remove->parent;
    } 
}

void delete(struct Tree* tree, void* value) {
    t_node* toDelete = find(tree->root, value);
    if (toDelete == NULL) {return;}
    if (toDelete->left == NULL) {
        transplant(tree, toDelete, toDelete->right);
    } else if (toDelete->right == NULL) {
        transplant(tree, toDelete, toDelete->left);
    } else {
        t_node* nextMin = min(toDelete->right);
        if (treeEqual(nextMin->parent, toDelete) != 0) {
            transplant(tree, nextMin, nextMin->right);
            nextMin->right = toDelete->right;
            nextMin->right->parent = nextMin;
        }
        transplant(tree, toDelete, nextMin);
        nextMin->left = toDelete->left;
        nextMin->left->parent = nextMin;
    }
}

