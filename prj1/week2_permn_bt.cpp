#include<bits/stdc++.h>

using namespace std;

int n;
vector<int> a;
vector<bool> used;

void gen() {
    if(a.size() == n){
        for(int x : a){
            cout << x+1 << ' ';
        }
        cout << '\n';
        return;
    }

    for (int i = 0; i< n ; i++){
        if(!used[i]){
            used[i] = true;
            a.push_back(i);

            gen();

            a.pop_back();
            used[i] = false;
        }
    }
}

int main(){
    cin >> n;
    used.resize(n+1);
    gen();
}