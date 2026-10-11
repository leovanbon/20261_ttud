#include <bits/stdc++.h>
using namespace std;

int board[9][9];
vector<pair<int,int>> mty;

bool row_check(int r, int fill){
    for(int i = 0; i < 9; i++){
        if(fill == board[r][i]) return false;
    }
    return true;
}

bool col_check(int c, int fill){
    for(int i = 0; i < 9; i++){
        if(fill == board[i][c]) return false;
    }
    return true;
}

bool subsq_check(int x, int y, int fill){
    for(int i = 0; i < 3; i++){
        for(int j = 0; j < 3; j++){
            if(fill == board[x+i][y+j])
                return false;
        }
    }
    return true;
}

bool fill_check(int x, int y, int val){
    return row_check(x, val)
        && col_check(y, val)
        && subsq_check((x/3)*3, (y/3)*3, val);
}

int ans = 0;

void try_fill(){
    if(mty.empty()){
        ans++;
        return;
    }

    auto cur = mty.back();
    mty.pop_back();

    int x = cur.first;
    int y = cur.second;

    for(int v = 1; v <= 9; v++){
        if(fill_check(x, y, v)){
            board[x][y] = v;

            try_fill();

            board[x][y] = 0;
        }
    }

    mty.push_back(cur);
}

int main(){
    for(int i = 0; i < 9; i++){
        for(int j = 0; j < 9; j++){
            cin >> board[i][j];

            if(board[i][j] == 0)
                mty.push_back({i, j});
        }
    }

    try_fill();

    cout << ans << '\n';
}