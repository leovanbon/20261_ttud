#include <bits/stdc++.h>
using namespace std;

int m, n;
int crd[31], LB[11];
int load[11], cnt[11], assigned[31];
bool canTeach[11][31], conflict[31][31];

int ans = INT_MAX;

void backtrack(int i) {
    if (i > n) {
        for (int t = 1; t <= m; t++) {
            if (cnt[t] < LB[t]) return;
        }

        int mx = 0;
        for (int t = 1; t <= m; t++)
            mx = max(mx, load[t]);

        ans = min(ans, mx);
        return;
    }

    for (int t = 1; t <= m; t++) {
        if (!canTeach[t][i]) continue;

        bool valid = true;

        for (int j = 1; j < i; j++) {
            if (assigned[j] == t && conflict[i][j]) {
                valid = false;
                break;
            }
        }

        if (!valid) continue;
        if (load[t] + crd[i] >= ans) continue;

        assigned[i] = t;
        load[t] += crd[i];
        cnt[t]++;

        bool possible = true;
        int remaining = n - i;

        for (int j = 1; j <= m; j++) {
            if (cnt[j] + remaining < LB[j]) {
                possible = false;
                break;
            }
        }

        if (possible)
            backtrack(i + 1);

        cnt[t]--;
        load[t] -= crd[i];
        assigned[i] = 0;
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cin >> m >> n;

    for (int t = 1; t <= m; t++) {
        int k;
        cin >> k;

        while (k--) {
            int c;
            cin >> c;
            canTeach[t][c] = true;
        }
    }

    for (int i = 1; i <= n; i++)
        cin >> crd[i];

    int sumLB = 0;

    for (int t = 1; t <= m; t++) {
        cin >> LB[t];
        sumLB += LB[t];
    }

    int k;
    cin >> k;

    while (k--) {
        int u, v;
        cin >> u >> v;
        conflict[u][v] = conflict[v][u] = true;
    }

    if (sumLB > n) {
        cout << -1;
        return 0;
    }

    backtrack(1);

    cout << (ans == INT_MAX ? -1 : ans);
}