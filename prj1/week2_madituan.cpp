#include <bits/stdc++.h>
using namespace std;

int n;
long long g[10][10];
bool visited[10][10];

long long ans;

int dx[] = {-2, -2, -1, -1, 1, 1, 2, 2};
int dy[] = {-1,  1, -2,  2,-2, 2,-1, 1};

vector<pair<int,int>> path;
vector<pair<int,int>> bestPath;

bool inside(int x, int y) {
    return x >= 0 && x < n && y >= 0 && y < n;
}

void dfs(int x, int y, long long sum) {
    if (sum > ans) {
        ans = sum;
        bestPath = path;
    }

    for (int k = 0; k < 8; k++) {
        int nx = x + dx[k];
        int ny = y + dy[k];

        if (inside(nx, ny) && !visited[nx][ny]) {
            visited[nx][ny] = true;
            path.push_back({nx, ny});

            dfs(nx, ny, sum + g[nx][ny]);

            path.pop_back();
            visited[nx][ny] = false;
        }
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int r, c;
    cin >> n >> r >> c;

    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            cin >> g[i][j];

    r--;
    c--;

    visited[r][c] = true;

    path.push_back({r, c});

    ans = g[r][c];
    bestPath = path;

    dfs(r, c, g[r][c]);

    cout << ans << '\n';

    // cout << "Route: ";
    // for (auto [x, y] : bestPath) {
    //     cout << "(" << x + 1 << "," << y + 1 << ") ";
    // }
    // cout << '\n';

}