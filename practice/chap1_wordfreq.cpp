#include<bits/stdc++.h>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    map<string, int> cnt;
    string line;

    while (getline(cin,line)) {
        if (line.empty()) break;

        string word;

        for (char c : line) {
            if (isalnum((unsigned char) c)) word += c;
            else {
                if (!word.empty()) {
                    cnt[word]++;
                    word.clear();
                }
            }
        }

        if (!word.empty()) cnt[word]++;
    }

    for (auto &[word,count] : cnt){
        cout << word << ' ' << count << '\n';
    }
    
    return 0;
}