#include<bits/stdc++.h>
using namespace std;

struct Edge {
    int u, v, w;
};

int parent[100005];
int sz[100005];

int find(int u) {
    if (parent[u] == u) return u;
    return parent[u] = find(parent[u]);
}

bool unite(int u, int v) {
    u = find(u);
    v = find(v);

    if (u == v) return false;

    if (sz[u] < sz[v]) swap(u,v);
    parent[v] = u;
    sz[u] += sz[v];
    
    return true;
}

int main() {
    int n,m; cin >> n >> m;

    vector<Edge> edges(m);

    for(int i = 0; i < m; i++) {
        cin >> edges[i].u >> edges[i].v >> edges[i].w;
    }

    for (int i = 1; i <= n; i++) {
        parent[i] = i;
        sz[i] = 1;
    }
    sort(edges.begin(), edges.end(),
        [](const Edge& a, const Edge& b) {
            return a.w < b.w;
        });

    long long ans = 0;
    int count = 0;

    for (auto [u,v,w] : edges) {
        if (unite(u, v)) {
            ans += w;
            count++;

            if (count == n-1) break;
        }
    }

    cout << ans << '\n';
}