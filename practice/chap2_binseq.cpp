#include <bits/stdc++.h>
using namespace std;

int n;
string s, cur;
long long ans = 0;

bool check() {
    int m = s.size();

    if ((int)cur.size() < m)
        return true;

    for (int i = 0; i < m; i++) {
        if (cur[cur.size() - m + i] != s[i])
            return true;
    }

    return false;
}

void Try(int k) {
    if (k == n) {
        ans++;
        return;
    }

    for (char c : {'0', '1'}) {
        cur.push_back(c);

        if (check())
            Try(k + 1);

        cur.pop_back();
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cin >> n >> s;

    Try(0);

    cout << ans << '\n';

    return 0;
}