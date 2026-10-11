#include <bits/stdc++.h>
using namespace std;

const long long MOD = 1000000007;

int main() {
    string s;
    getline(cin, s);

    string t;
    for (char c : s)
        if (!isspace(c)) t += c;

    if (t.empty()) {
        cout << "NOT_CORRECT";
        return 0;
    }

    long long ans = 0, term = 1;
    int n = t.size(), i = 0;
    char op = '*';

    while (i < n) {
        if (!isdigit(t[i])) {
            cout << "NOT_CORRECT";
            return 0;
        }

        long long x = 0;
        bool positive = false;

        while (i < n && isdigit(t[i])) {
            if (t[i] != '0') positive = true;
            x = (x * 10 + (t[i] - '0')) % MOD;
            i++;
        }

        if (!positive) {
            cout << "NOT_CORRECT";
            return 0;
        }

        if (op == '*')
            term = term * x % MOD;
        else {
            ans = (ans + term) % MOD;
            term = x;
        }

        if (i == n) break;

        if (t[i] != '+' && t[i] != '*') {
            cout << "NOT_CORRECT";
            return 0;
        }

        op = t[i++];
        
        if (i == n) {
            cout << "NOT_CORRECT";
            return 0;
        }
    }

    ans = (ans + term) % MOD;
    cout << ans;
}