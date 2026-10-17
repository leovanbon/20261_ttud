
#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    unordered_set<string> s;
    string k, cmd;

    while (cin >> k && k != "*") {
        s.insert(k);
    }

    while (cin >> cmd && cmd != "***") {
        cin >> k;

        if (cmd == "find") {
            cout << (s.count(k) > 0 ? 1 : 0) << '\n';
        }
        else if (cmd == "insert") {
            cout << (s.insert(k).second ? 1 : 0) << '\n';
        }
    }

    return 0;
}
