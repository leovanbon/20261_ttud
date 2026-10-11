
#include <bits/stdc++.h>
using namespace std;

unordered_map<string, vector<string>> tree;
unordered_map<string, int> desc, gen;

void dfs(const string& u) {
    if (gen.count(u)) return;

    desc[u] = 0;
    gen[u] = 0;

    for (const string& v : tree[u]) {
        dfs(v);
        desc[u] += 1 + desc[v];
        gen[u] = max(gen[u], 1 + gen[v]);
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    string child, parent;

    while (cin >> child && child != "***") {
        cin >> parent;
        tree[parent].push_back(child);
    }

    string cmd, name;

    while (cin >> cmd && cmd != "***") {
        cin >> name;
        dfs(name);

        if (cmd == "descendants")
            cout << desc[name] << '\n';
        else if (cmd == "generation")
            cout << gen[name] << '\n';
    }

    return 0;
}
