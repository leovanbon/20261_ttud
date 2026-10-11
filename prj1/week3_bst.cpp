#include<bits/stdc++.h>
using namespace std;

struct Node {
    int key;
    Node *left = nullptr, *right = nullptr;

    Node(int k) : key(k) {}
};

Node *insert(Node* root, int k) {
    if(!root) return new Node(k);

    if(k < root->key) root->left = insert(root->left, k);
    else if (k > root->key) root->right = insert(root->right, k);

    return root;
}

void preorder(Node *root) {
    if (!root) return;

    cout << root->key << " ";
    preorder(root->left);
    preorder(root->right);
}

int main(){
    Node* root = nullptr;
    string cmd;
    int k;

    while (cin >> cmd && cmd != "#") {
        cin >> k;
        root = insert(root, k);
    }

    preorder(root);
    return 0;
}