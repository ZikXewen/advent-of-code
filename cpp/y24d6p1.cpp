#include <bits/stdc++.h>

using namespace std;

int main() {
  vector<vector<char>> map;
  int x, y, dx[4] = {-1, 0, 1, 0}, dy[4] = {0, 1, 0, -1}, dir = 0, ans = 1;
  string str, num;

  while (getline(cin, str)) {
    if (str.empty())
      break;
    map.emplace_back(str.begin(), str.end());
  }
  for (int i = 0; i < map.size(); ++i)
    for (int j = 0; j < map[0].size(); ++j)
      if (map[i][j] == '^')
        map[x = i][y = j] = 'X';
  while (x && y && x != map.size() - 1 && y != map[0].size() - 1)
    switch (map[x + dx[dir]][y + dy[dir]]) {
    case '#':
      dir = (dir + 1) % 4;
      break;
    case 'X':
      x += dx[dir], y += dy[dir];
      break;
    case '.':
      map[x += dx[dir]][y += dy[dir]] = 'X', ++ans;
    }
  printf("%d", ans);
}
