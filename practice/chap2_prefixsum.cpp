#include<bits/stdc++.h>

using namespace std;

const int MAXN = 1e5+5;

int a[MAXN];

int main(){

    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n;
    cin >> n;
    
    for (int i = 0; i< n; i++){
        cin >> a[i];
    }
    vector<int>prefixSum(n+2);

    prefixSum[0] = 0;

    for (int i = 0; i < n; i++){
        prefixSum[i+1] = prefixSum[i] + a[i];
    }

    int q; cin >> q;
    int l, r;
    while(q--){
        cin >> l >> r;
        cout << prefixSum[r] - prefixSum[l-1] << '\n';
    }
    
    return 0;
}
