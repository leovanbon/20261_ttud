#include <bits/stdc++.h>
using namespace std;

const int INF = 1e9;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n, k;
    cin >> n >> k;

    int N = 2 * n;
    vector<vector<int>> c(N + 1, vector<int>(N + 1));

    for (int i = 0; i <= N; i++)
        for (int j = 0; j <= N; j++)
            cin >> c[i][j];

    int FULL = (1 << N) - 1;

    // dp[mask][last] = minimum cost after visiting mask,
    // with the bus currently at point last.
    // mask stores the work status
    vector<vector<int>> dp(1 << N, vector<int>(N + 1, INF));
    dp[0][0] = 0;

    for (int mask = 0; mask <= FULL; mask++) {
        for (int last = 0; last <= N; last++) {
            if (dp[mask][last] == INF) continue;

            int picked = 0, delivered = 0;

            for (int i = 0; i < n; i++) {
                if (mask & (1 << i))
                    picked++;
                if (mask & (1 << (i + n)))
                    delivered++;
            }

            int onboard = picked - delivered;

            // pick up passenger i at point i + 1.
            if (onboard < k) {
                for (int i = 0; i < n; i++) {
                    int bit = 1 << i;

                    if (!(mask & bit)) {
                        int nextMask = mask | bit;
                        dp[nextMask][i + 1] =
                            min(dp[nextMask][i + 1],
                                dp[mask][last] + c[last][i + 1]);
                    }
                }
            }

            // deliver passenger i at point i + n + 1.
            for (int i = 0; i < n; i++) {
                int pickupBit = 1 << i;
                int deliveryBit = 1 << (i + n);

                if ((mask & pickupBit) &&
                    !(mask & deliveryBit)) {
                    int nextMask = mask | deliveryBit;

                    dp[nextMask][i + n + 1] =
                        min(dp[nextMask][i + n + 1],
                            dp[mask][last] +
                            c[last][i + n + 1]);
                }
            }
        }
    }

    int ans = INF;

    for (int last = 1; last <= N; last++)
        ans = min(ans, dp[FULL][last] + c[last][0]);

    cout << ans << '\n';

    return 0;
}