#include <bits/stdc++.h>
using namespace std;

const int MAX = 1e5 + 7;
int t[4 * MAX];

void upd(int id, int l, int r, int p, int v) {
    if (p < l || p > r) return;
    if (l == r) { t[id] = v; return; }
    int m = (l + r) / 2;
    if (p <= m) upd(id * 2, l, m, p, v);
    else upd(id * 2 + 1, m + 1, r, p, v);
    t[id] = t[id * 2] + t[id * 2 + 1];
}

int get_min_greater(int id, int l, int r, int q) {
    if (r < q || t[id] == 0) return 0;
    if (l == r) return l;
    int m = (l + r) / 2;
    int x = get_min_greater(id * 2, l, m, q);
    return (x != 0) ? x : get_min_greater(id * 2 + 1, m + 1, r, q);
}

int main() {

    int n;
    if (!(cin >> n)) return 0;

    int c;
    for (int i = 0; i < n; ++i) {
        cin >> c;
        upd(1, 1, MAX, c, 1);
    }

    string s;
    while (cin >> s && s != "#") {
        cin >> c;

        if (s == "insert") {
            upd(1, 1, MAX, c, 1);
        } else if (s == "remove") {
            upd(1, 1, MAX, c, 0);
        } else if (s == "min_greater_equal") {
            int ans = (c <= MAX) ? get_min_greater(1, 1, MAX, c) : 0;
            if (ans == 0) cout << "NULL\n";
            else cout << ans << "\n";
        } else if (s == "min_greater") {
            int ans = (c < MAX) ? get_min_greater(1, 1, MAX, c + 1) : 0;
            if (ans == 0) cout << "NULL\n";
            else cout << ans << "\n";
        }
    }

    return 0;
}