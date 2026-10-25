#include <bits/stdc++.h>
using namespace std;

int n, m;
vector<vector<int>> adj;
vector<bool> visited;

bool dfs(int u, int cnt) {
    if (cnt == n)
        return adj[u][0] == 1;

    for (int v = 0; v < n; v++) {
        if (adj[u][v] && !visited[v]) {
            visited[v] = true;

            if (dfs(v, cnt + 1))
                return true;

            visited[v] = false;
        }
    }

    return false;
}

int solve() {
    cin >> n >> m;

    adj.assign(n, vector<int>(n, 0));
    visited.assign(n, false);

    for (int i = 0; i < m; i++) {
        int u, v; cin >> u >> v;
        --u; --v;
        adj[u][v] = adj[v][u] = 1;
    }

    if (n < 3) return 0;

    visited[0] = true;

    return dfs(0, 1);
}

int main() {
    int t;
    cin >> t;

    while (t--) {
        cout << solve() << "\n";
    }
}