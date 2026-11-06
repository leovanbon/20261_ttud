#include <bits/stdc++.h>
#include <queue>

using namespace std;

const int INF = 1e9;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n, m; cin >> n >> m;

    vector<vector<pair<int, int>>> adj(n+1);

    for (int i = 0; i < m; i++){
        int u, v, w;
        cin >> u >> v >> w;
        adj[u].push_back({v, w});
    }

    for (int s = 1; s <= n ; s++){
        vector<int> dist(n+1, INF);
        dist[s] = 0;

        priority_queue<
            pair<int, int>,
            vector<pair<int, int>>,
            greater<pair<int, int>>
        > pq;

        pq.push({0, s});

        while (!pq.empty()) {
            auto [d, u] = pq.top();
            pq.pop();

            if (d > dist[u]) continue;

            for (auto [v, w] : adj[u]) {
                if (dist[v] > dist[u] + w) {
                    dist[v] = dist[u] + w;
                    pq.push({dist[v], v});
                }
            }
        }

        for (int j = 1; j <= n; j++) {
            if (j > 1) cout << " ";
            cout << (dist[j] == INF ? -1 : dist[j]);
        }

        cout << '\n';
    }

    return 0;
}