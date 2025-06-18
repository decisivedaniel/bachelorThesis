
#include <stdlib.h>;

struct Node {
    int value;
    t_node* left;
    t_node* right;
    t_node* parent;
} typedef t_node;

struct Tree {
    t_node* root;
};

void inorderWalk(t_node* node);
int contains(t_node* node, int lookValue);
t_node* find(t_node* node, int lookValue);
t_node* min(t_node* node);
t_node* max(t_node* node);
t_node* successor(t_node* node);
void insert(struct Tree* tree, int add);
void transplant(struct Tree* tree, t_node* u, t_node* v);
void delete(struct Tree* tree, int value);

void inorderWalk(t_node* node) {
    if (node != NULL) {
        inorderWalk(node->left);
        printf("%d",node->value);
        inorderWalk(node->right);
    }
}

t_node* find(t_node* node, int lookValue) {
    if (node == NULL || node->value == lookValue) {
        return node;
    }
    if (lookValue < node->value) {
        return find(node->left, lookValue);
    }
    return find(node->right, lookValue);
}

int contains(t_node* node, int lookValue) {
    t_node* found = find(node, lookValue);
    if (found == NULL) {
        return 0;
    }
    return 1;
}

t_node* min(t_node* node) {
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
    while (parent != NULL && node == parent->right) {
        node = parent;
        parent = parent->parent;
    }
    return parent;
}

void insert(struct Tree* tree, int add) {
    t_node* previous = NULL;
    t_node* current = tree->root;
    while (current != NULL) {
        previous = current;
        if (add < current->value) {
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

void transplant(struct Tree* tree, t_node* u, t_node* v) {
    if (u->parent == NULL) {
        tree->root = v;
    } else if (u == u->parent->left) {
        u->parent->left = v;
    } else {
        u->parent->right = v;
    }
    if (v != NULL) {
        v->parent = u->parent;
    } 
}

void delete(struct Tree* tree, int value) {
    t_node* toDelete = find(tree->root, value);
    if (toDelete == NULL) {return;}
    if (toDelete->left == NULL) {
        transplant(tree, toDelete, toDelete->right);
    } else if (toDelete->right == NULL) {
        transplant(tree, toDelete, toDelete->left);
    } else {
        t_node* nextMin = min(toDelete->right);
        if (nextMin->parent != toDelete) {
            transplant(tree, nextMin, nextMin->right);
            nextMin->right = toDelete->right;
            nextMin->right->parent = nextMin;
        }
        transplant(tree, toDelete, nextMin);
        nextMin->left = toDelete->left;
        nextMin->left->parent = nextMin;
    }
}

