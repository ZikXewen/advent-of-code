#include <bits/stdc++.h>

using namespace std;

int main() {
  vector<vector<char>> v;
  long long ct = 0;
  string str;
  while (getline(cin, str)) {
    if (str.empty())
      break;
    v.push_back(vector<char>(str.begin(), str.end()));
  }
  for (int i = 0; i < v.size(); ++i)
    for (int j = 0; j < v[0].size(); ++j)
      if (v[i][j] == 'X')
        for (int di = -1; di <= 1; ++di)
          if (i + 3 * di >= 0 && i + 3 * di < v.size())
            for (int dj = -1; dj <= 1; ++dj)
              if (j + 3 * dj >= 0 && j + 3 * dj < v[0].size() &&
                  v[i + di][j + dj] == 'M' &&
                  v[i + 2 * di][j + 2 * dj] == 'A' &&
                  v[i + 3 * di][j + 3 * dj] == 'S')
                ++ct;
  printf("%lld", ct);
}
