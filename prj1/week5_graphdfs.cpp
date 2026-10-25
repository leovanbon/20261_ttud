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

    stack<int> st;
    bool first = true; //for printing

    for (int s= 1; s<=n ; s++) {
        if (visited[s]) continue;
        
        st.push(s);

        while (!st.empty()) {
            int u = st.top();
            st.pop();

            if (visited[u]) continue;

            visited[u] = true;

            if (!first) cout << " ";
            cout << u;
            first = false;

            //backward so the smallest get popped first
            for (int i = (int)adj[u].size() - 1; i >= 0; i--) {
                int v = adj[u][i];
                
                if (!visited[v]) st.push(v);
            }
        }
    }

    cout << '\n';
}