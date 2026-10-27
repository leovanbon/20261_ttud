#include<bits/stdc++.h>

using namespace std;

int main() {
    int n, q; cin >> n >> q;
    long long s = 0;
    vector<int> a(n);

    for (int &x : a) cin >> x;

    long long sum = 0;
    int l = 0, ans = 0;
    
    for (int r = 0; r < n; r ++) {
        sum += a[r];

        while (l <= r && sum > q) {
            sum -= a[l++];
        }

        ans = max(ans , r - l + 1);
    }

    cout << ans;

    return 0;
}