#include <bits/stdc++.h>
using namespace std;

struct Node {
    int id;
    vector<Node*> child;

    Node(int x){
        id = x;
    }
};

Node* root = nullptr;
unordered_map<int, Node*> mp;

void preOrder(Node* p) {
    if (p == nullptr) return;

    cout << p->id << " ";

    for (Node* child : p->child) preOrder(child);
}

void inOrder(Node* p) {
    if (p == nullptr) return;

    int n = p->child.size();

    if (n > 0) inOrder(p->child[0]);

    cout << p->id << " ";

    for (int i = 1; i<n ; i++) inOrder(p->child[i]);
}

void postOrder(Node* p) {
    if (p == nullptr) return;
    
    for (Node* child : p->child) postOrder(child);
    
    cout << p->id << " ";
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    string cmd;

    while(cin>>cmd && cmd != "*") {
        if (cmd == "MakeRoot"){
            int u; cin >> u;

            if (root == nullptr) { 
                root = new Node(u);
                mp[u] = root;
            }
        }
        else if (cmd == "Insert") {
            int u, v;
            cin >> u >> v;

            if (mp.count(u) || !mp.count(v))
                continue;

            Node* p = new Node(u);
            mp[u] = p;

            mp[v]->child.push_back(p);
        }
        else if (cmd == "PreOrder") {
            preOrder(root);
            cout << '\n';
        }
        else if (cmd == "InOrder") {
            inOrder(root);
            cout << '\n';
        }
        else if (cmd == "PostOrder") {
            postOrder(root);
            cout << '\n';
        }
    }
}
