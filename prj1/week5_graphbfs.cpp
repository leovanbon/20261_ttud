#include<bits/stdc++.h>

using namespace std;

int main() {
    int n,m; cin>>n>>m;

    vector<vector<int>> adj(n+1);
    vector<bool> visited(n+1, false);

    for (int i = 0; i < m ; i++) {
        int u, v;
        cin >> u >> v;
        adj[u].push_back(v);
        adj[v].push_back(u);
    }

    for (int u = 1; u <= n; u++) {
        sort(adj[u].begin(), adj[u].end());
    }

    queue<int> q;

    for (int s= 1; s<=n ; s++) {
        if (visited[s]) continue;
        
        q.push(s);
        visited[s] = true; 

        while (!q.empty()) {
            int u = q.front();
            q.pop();

            cout << u << " ";
            
            for (int v : adj[u]) {
                if (!visited[v]){
                    visited[v] = true;
                    q.push(v);
                }
            }
        }
    }

    cout << '\n';
}