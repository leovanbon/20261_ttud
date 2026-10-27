#include <bits/stdc++.h>
using namespace std;

int main() { 
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);

    stack<int> s;
    string command;

    while (cin >> command && command != "#") {
        if (command == "PUSH") {
            int value;
            cin >> value;
            s.push(value);
        } else if (command == "POP") {
            if (s.empty()) {
                cout << "NULL\n";
            } else {
                cout << s.top() << "\n";
                s.pop();
            }
        }
    }

    return 0;
}