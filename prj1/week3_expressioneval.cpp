#include <bits/stdc++.h>
using namespace std;

const long long MOD = 1000000007;

string s;
int pos = 0;
bool ok = true;

long long expression();

long long factor() {
    if (pos >= s.size()) {
        ok = false;
        return 0;
    }

    // ( expression )
    if (s[pos] == '(') {
        pos++;

        long long val = expression();

        if (pos >= s.size() || s[pos] != ')') {
            ok = false;
            return 0;
        }

        pos++;
        return val;
    }

    // number
    if (!isdigit(s[pos])) {
        ok = false;
        return 0;
    }

    long long val = 0;
    bool positive = false;

    while (pos < s.size() && isdigit(s[pos])) {
        if (s[pos] != '0')
            positive = true;

        val = (val * 10 + s[pos] - '0') % MOD;
        pos++;
    }

    // positive integer => cannot be 0
    if (!positive)
        ok = false;

    return val;
}

long long term() {
    long long val = factor();

    while (ok && pos < s.size() && s[pos] == '*') {
        pos++;

        long long x = factor();
        val = val * x % MOD;
    }

    return val;
}

long long expression() {
    long long val = term();

    while (ok && pos < s.size() && s[pos] == '+') {
        pos++;

        long long x = term();
        val = (val + x) % MOD;
    }

    return val;
}

int main() {
    string input;
    getline(cin, input);

    // remove spaces
    for (char c : input) {
        if (!isspace(c))
            s += c;
    }

    if (s.empty()) {
        cout << "NOT_CORRECT";
        return 0;
    }

    long long ans = expression();

    if (!ok || pos != s.size())
        cout << "NOT_CORRECT";
    else
        cout << ans;

    return 0;
}