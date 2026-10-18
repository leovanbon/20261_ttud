
#include <bits/stdc++.h>
using namespace std;

int a, b, c;

int bfs() {
    if (c > max(a, b) || c % gcd(a, b) != 0)
        return -1;

    queue<pair<int, int>> q;
    vector<vector<int>> dist(a + 1, vector<int>(b + 1, -1));

    q.push({0, 0});
    dist[0][0] = 0;

    while (!q.empty()) {
        auto [x, y] = q.front();
        q.pop();

        if (x == c || y == c)
            return dist[x][y];

        int t1 = min(x, b - y);
        int t2 = min(y, a - x);

        vector<pair<int, int>> next = {
            {a, y},           // Fill jug A
            {x, b},           // Fill jug B
            {0, y},           // Empty jug A
            {x, 0},           // Empty jug B
            {x - t1, y + t1}, // Pour A -> B
            {x + t2, y - t2}  // Pour B -> A
        };

        for (auto [nx, ny] : next) {
            if (dist[nx][ny] == -1) {
                dist[nx][ny] = dist[x][y] + 1;
                q.push({nx, ny});
            }
        }
    }

    return -1;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cin >> a >> b >> c;
    cout << bfs() << '\n';

    return 0;
}
