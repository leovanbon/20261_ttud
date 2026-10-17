#include <bits/stdc++.h>
using namespace std;

int m, n, ans = INT_MAX;
int load[11], assigned[31];
bool canTeach[11][31], conflict[31][31];

bool valid(int i, int t) {
    if (!canTeach[t][i]) return false;

    for (int j = 1; j < i; j++)
        if (assigned[j] == t && conflict[i][j])
            return false;

    return true;
}

void backtrack(int i) {
    if (i > n) {
        ans = min(ans, *max_element(load + 1, load + m + 1));
        return;
    }

    for (int t = 1; t <= m; t++) {
        if (!valid(i, t) || load[t] + 1 >= ans)
            continue;

        assigned[i] = t;
        load[t]++;

        backtrack(i + 1);

        load[t]--;
        assigned[i] = 0;
    }
}

int main() {
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

    int k;
    cin >> k;
    while (k--) {
        int u, v;
        cin >> u >> v;
        conflict[u][v] = conflict[v][u] = true;
    }

    backtrack(1);
    cout << (ans == INT_MAX ? -1 : ans);
}