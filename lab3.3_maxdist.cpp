#include<bits/stdc++.h>
using namespace std;

bool can(const vector<int>& a, int c, int d) {
    int count = 1;
    int last = a[0];

    for(int i = 1; i < (int)a.size(); i++) {
        if (a[i] - last >= d) {
            count++;
            last = a[i];

            if (count >= c) return true;
        }
    }

    return count >= c;
}

void solve() {
    int n, c; cin >> n >> c;
    vector<int> a(n);
    for(int& x : a) { cin >> x; }

    sort(a.begin(), a.end());

    int low = 0;
    int high = a[n-1] - a[0];
    int ans = 0;

    while(low <= high) {
        int mid = (low +high) / 2;
        if (can(a,c,mid)) {
            ans = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    cout << ans << '\n';
}

int main() {
    int t; cin >> t;

    while(t--){
        solve();
    }

    return 0;
}