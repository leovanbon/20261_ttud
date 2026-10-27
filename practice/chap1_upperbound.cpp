#include<bits/stdc++.h>


const int MAXN = 1e5 + 5;

int a[MAXN];

int upperBound(int a[], int n, int k) {
    int l = 0, r = n - 1;
    int ans = -1;
    while (l <= r){
        int mid = (l + r) / 2;
        if (a[mid] > k){
            ans = a[mid];
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }
    
    return ans;
}

int main(){
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);

    int n;
    std::cin >> n;

    for (int i = 0; i < n; i++){
        std::cin >> a[i];
    }

    std::sort(a, a+n);

    std::string op;

    while (std::cin >>op && op != "#") {
        int k;
        std::cin >> k;

        std::cout << upperBound(a,n,k) << "\n";
    }

    return 0;
}