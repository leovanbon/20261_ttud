#include<bits/stdc++.h>

using namespace std;

int a[100005];

int main(){
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n,q; cin >>n>>q;
    for (int i=0; i<n; i++){
        cin >> a[i];
    }
    sort(a,a+n);

    int l = 0, r = n-1;
    int ans =0;
    while(l!=r){
        long long sum = a[l] + a[r];
        if(sum == q){
            ans++; l++; r--;
        } else if (sum < q){
            l++;
        } else {
            r--;
        }
    }

    cout << ans;
}