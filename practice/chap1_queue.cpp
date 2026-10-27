#include <bits/stdc++.h>
using namespace std;

int main() { 
    queue<int> q;
    string command;

    while (std::cin >> command && command != "#") {
        if (command == "PUSH") {
            int value;
            std::cin >> value;
            q.push(value);
        } else if (command == "POP") {
            if (q.empty()) {
                std::cout << "NULL\n";
            } else {
                std::cout << q.front() << "\n";
                q.pop();
            }
        }
    }

    return 0;
}