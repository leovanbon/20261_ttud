#include<bits/stdc++.h>
using namespace std;

int n,m, D;
int c[40][20];

int cnt[40];

long long ans = LLONG_MAX;

void dfs(int task, long long cost) {
    if (cost >= ans) return;

    if (task == n){
        ans  = min(ans, cost);
        return;
    }

    for (int i = 0; i<m ; i++){
        if (cnt[i] < D){
            cnt[i] ++;
            
            dfs(task + 1, cost + c[i][task]);

            cnt[i] --;
        }
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    
    cin >> m >> n >> D;

    for (int i = 0; i< m; i++){
        for (int j =0 ; j < n ; j ++){
            cin >> c[i][j];
        }
    }

    if (m*D < n){
        cout << -1;
        return 0;
    }

    dfs(0, 0);
    if (ans == LLONG_MAX) cout << -1;
    else cout << ans;
    return 0;
}
