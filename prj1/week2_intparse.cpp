#include <bits/stdc++.h>
using namespace std;

struct State {
    int sum;
    int next;
};

int main() {
    int n;
    cin >> n;

    vector<int> a;
    vector<State> st;

    st.push_back({0, 1});

    while (!st.empty()) {
        State &cur = st.back();

        if (cur.next > n - cur.sum) {
            st.pop_back();

            if (!a.empty())
                a.pop_back();

            continue;
        }

        int x = cur.next++;

        a.push_back(x);
        int newSum = cur.sum + x;

        if (newSum == n) {
            for (int v : a)
                cout << v << " ";
            cout << '\n';

            a.pop_back();
        }
        else {
            st.push_back({newSum, x});
        }
    }
}