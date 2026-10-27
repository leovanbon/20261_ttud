#include <bits/stdc++.h>
using namespace std;

const int INF = 1e9;

int tree[400005];

void update(int id, int l, int r, int pos, int val) {
    if (l == r) {
        tree[id] = val;
        return;
    }

    int mid = (l + r) / 2;

    if (pos <= mid)
        update(id * 2, l, mid, pos, val);
    else
        update(id * 2 + 1, mid + 1, r, pos, val);

    tree[id] = min(tree[id * 2], tree[id * 2 + 1]);
}

int query(int id, int l, int r, int ql, int qr) {
    // no overlap
    if (r < ql || qr < l)
        return INF;

    // completely inside
    if (ql <= l && r <= qr)
        return tree[id];

    int mid = (l + r) / 2;

    return min(
        query(id * 2, l, mid, ql, qr),
        query(id * 2 + 1, mid + 1, r, ql, qr)
    );
}

int main() {
    int n;
    cin >> n;

    fill(tree, tree + 400005, INF);

    for (int i = 1; i <= n; i++) {
        int x;
        cin >> x;

        update(1, 1, n, i, x);
    }

    cout << query(1, 1, n, 3, 8) << '\n';
}