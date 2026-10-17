#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n, M;
    cin >> n >> M;

    vector<int> a(n);
    int sum = 0;

    for (int i = 0; i < n; i++) {
        cin >> a[i];
        sum += a[i];
    }

    int R = M - sum;

    if (R < 0) {
        cout << 0;
        return 0;
    }

    vector<long long> dp(R + 1, 0);
    dp[0] = 1;

    for (int i = 0; i < n; i++) {
        for (int s = a[i]; s <= R; s++) {
            dp[s] += dp[s - a[i]];
        }
    }

    cout << dp[R];
    return 0;
}