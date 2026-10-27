#include<bits/stdc++.h>

using namespace std;

vector<vector<int>> build2DPrefixSum(const vector<vector<int>> &a) {
    int m = (int)a.size(), n = (int)a[0].size();
    vector<vector<int>> prefixSum(m+1, vector<int>(n+1, 0));

    for (int i = 1; i<=m; i++){
        for (int j = 1; j <= n ; j++) {
            prefixSum[i][j] = prefixSum[i-1][j] + prefixSum[i][j-1] - prefixSum[i-1][j-1] + a[i-1][j-1];
        }
    }
    return prefixSum;
}


int main(){

    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n, m;
    cin >> m >> n;
    vector<vector<int>> a(m, vector<int>(n, 0));

    for (int i = 0; i< m; i++){
        for (int j = 0; j < n; j++){
            cin >> a[i][j];
        }
    }

    vector<vector<int>> pref = build2DPrefixSum(a);

    int q; cin >> q;
    int x1,y1,x2,y2;

    while(q--){
        cin >> x1 >> y1 >> x2 >> y2;
        cout << pref[x2][y2] - pref[x1-1][y2] - pref[x2][y1-1] + pref[x1-1][y1-1] << '\n';
    }
    
    return 0;
}
