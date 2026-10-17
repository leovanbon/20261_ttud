#include <bits/stdc++.h>
using namespace std;

int n;
map<int, bool> first;
int a[100005];


int main() {
    cin >> n;
    for(int i = 0; i< n; i++){
        cin >> a[i];
        first[a[i]] = true;
    }

    for(int i = 0; i< n; i++){
        cout << (!first[a[i]] ? 1 : 0) << "\n";
        first[a[i]] = false;
    }

    return 0;
}