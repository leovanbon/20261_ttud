#include<bits/stdc++.h>
using namespace std;

int n;
string s;

void gen(int pos){
    if (pos == n){
        if (s.find("111") != string::npos){
            cout << s << '\n';
        }
        return;
    }

    s.push_back('0');
    gen(pos+1);
    s.pop_back();

    s.push_back('1');
    gen(pos+1);
    s.pop_back();
}

int main(){
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cin >> n;

    gen(0);

    return 0;
}