#include <bits/stdc++.h>

using namespace std;

int main() {
  vector<vector<char>> map;
  int x, y, ct, dx[4] = {-1, 0, 1, 0}, dy[4] = {0, 1, 0, -1}, dir = 0, ans = 0;
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
  int sx = x, sy = y;
  while (x && y && x != map.size() - 1 && y != map[0].size() - 1)
    if (map[x + dx[dir]][y + dy[dir]] == '#')
      dir = (dir + 1) % 4;
    else
      map[x += dx[dir]][y += dy[dir]] = 'X';
  for (int i = 0; i < map.size(); ++i)
    for (int j = 0; j < map[0].size(); ++j)
      if (map[i][j] == 'X') {
        x = sx, y = sy, dir = 0, map[i][j] = '#', ct = 0;
        while (x && y && x != map.size() - 1 && y != map[0].size() - 1 &&
               ct < map.size() * map[0].size())
          if (map[x + dx[dir]][y + dy[dir]] == '#')
            dir = (dir + 1) % 4;
          else
            x += dx[dir], y += dy[dir], ++ct;
        if (ct == map.size() * map[0].size())
          ++ans;
        map[i][j] = 'X';
      }
  printf("%d", ans);
}
