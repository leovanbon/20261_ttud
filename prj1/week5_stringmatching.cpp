#include <bits/stdc++.h>
using namespace std;

int main() {
    string p, t;
    getline(cin, p);
    getline(cin, t);

    int n = p.size(), m = t.size();

    vector<int> lps(n, 0);

    // Build LPS array
    int j = 0;
    for (int i = 1; i < n; i++) {
        while (j > 0 && p[i] != p[j]) {
            j = lps[j - 1];
        }

        if (p[i] == p[j]) j++;

        lps[i] = j;
    }

    // KMP matching
    int count = 0;
    j = 0;

    for (int i = 0; i < m; i++) {
        while (j > 0 && t[i] != p[j]) {
            j = lps[j - 1];
        }

        if (t[i] == p[j]) j++;

        if (j == n) {
            count++;
            j = lps[j - 1];
        }
    }

    cout << count << '\n';
    return 0;
}
