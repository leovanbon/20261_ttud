#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n;
    cin >> n;

    vector<pair<int, int>> segs(n);

    for (int i = 0; i < n; i++) {
        cin >> segs[i].first >> segs[i].second;
    }

    sort(segs.begin(), segs.end(),
        [](const auto& a, const auto& b) {
            return a.second < b.second;
        });

    int ans = 0;
    int last_end = -1;

    for (auto [a, b] : segs) {
        if (a > last_end) {
            ans++;
            last_end = b;
        }
    }

    cout << ans << '\n';

    return 0;
}
