#include<bits/stdc++.h>
using namespace std;

const long long MOD = 1e9 + 7;

long long mergeSot(vector<int>& a, int l, int r){
    if (l>=r) return 0;

    int mid = (l + r) / 2;
    
    long long cnt = 0;
    cnt += mergeSot(a, l, mid);
    cnt += mergeSot(a, mid+1, r);

    vector<int> temp;
    int i = l, j = mid+1;

    while (i <= mid && j <= r){
        if(a[i] <= a[j]) {
            temp.push_back(a[i++]);
        } else {
            temp.push_back(a[j++]);
            cnt += mid - i + 1;
        }
    }

    while (i <= mid) temp.push_back(a[i++]);
    while (j <= r) temp.push_back(a[j++]);

    for (int k = 0; k < (int)temp.size(); k++){
        a[l + k] = temp[k];
    }

    return cnt % MOD;
}

int main() {
    int n; cin >> n;
    vector<int> a(n);
    for(int& x: a) { cin >> x; }

    cout << mergeSot(a, 0, n-1) << '\n';
    return 0;
}