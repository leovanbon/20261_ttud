
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int key;
    Node* prev;
    Node* next;

    Node(int k) {
        key = k;
        prev = nullptr;
        next = nullptr;
    }
};

Node* head = nullptr;
Node* tail = nullptr;

unordered_map<int, Node*> mp;

void addlast(int k) {
    if (mp.count(k)) return;

    Node* p = new Node(k);

    if (head == nullptr) {
        head = tail = p;
    } else {
        tail->next = p;
        p->prev = tail;
        tail = p;
    }

    mp[k] = p;
}

void addfirst(int k) {
    if (mp.count(k)) return;

    Node* p = new Node(k);

    if (head == nullptr) {
        head = tail = p;
    } else {
        p->next = head;
        head->prev = p;
        head = p;
    }

    mp[k] = p;
}

void addafter(int u, int v) {
    if (mp.count(u) || !mp.count(v)) return;

    Node* q = mp[v];

    if (q == tail) {
        addlast(u);
        return;
    }

    Node* p = new Node(u);

    p->next = q->next;
    p->prev = q;
    q->next->prev = p;
    q->next = p;

    mp[u] = p;
}


void addbefore(int u, int v) {
    if (mp.count(u) || !mp.count(v)) return;

    Node* q = mp[v];

    if (q == head) {
        addfirst(u);
        return;
    }

    Node* p = new Node(u);

    p->prev = q->prev;
    p->next = q;
    q->prev->next = p;
    q->prev = p;

    mp[u] = p;
}

void removeNode(int k) {
    if (!mp.count(k)) return;

    Node* p = mp[k];

    if (p->prev != nullptr)
        p->prev->next = p->next;
    else
        head = p->next;

    if (p->next != nullptr)
        p->next->prev = p->prev;
    else
        tail = p->prev;

    mp.erase(k);
    delete p;
}

// Dao nguoc danh sach, khong tao Node moi
void reverseList() {
    Node* p = head;

    while (p != nullptr) {
        swap(p->prev, p->next);
        p = p->prev;
    }

    swap(head, tail);
}

// In danh sach
void printList() {
    Node* p = head;

    while (p != nullptr) {
        cout << p->key;
        if (p->next != nullptr) cout << " ";
        p = p->next;
    }

    cout << '\n';
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n, k, u, v;
    cin >> n;

    for (int i = 0; i < n; i++) {
        cin >> k;
        addlast(k);
    }

    string cmd;

    while (cin >> cmd && cmd != "#") {
        if (cmd == "addlast") {
            cin >> k;
            addlast(k);
        }
        else if (cmd == "addfirst") {
            cin >> k;
            addfirst(k);
        }
        else if (cmd == "addafter") {
            cin >> u >> v;
            addafter(u, v);
        }
        else if (cmd == "addbefore") {
            cin >> u >> v;
            addbefore(u, v);
        }
        else if (cmd == "remove") {
            cin >> k;
            removeNode(k);
        }
        else if (cmd == "reverse") {
            reverseList();
        }
    }

    printList();

    return 0;
}