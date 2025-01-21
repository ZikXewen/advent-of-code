#include <bits/stdc++.h>

using namespace std;

int main() {
  vector<vector<char>> v;
  long long ct = 0;
  string str;
  char pat[4] = {'M', 'M', 'S', 'S'};
  while (getline(cin, str)) {
    if (str.empty())
      break;
    v.push_back(vector<char>(str.begin(), str.end()));
  }
  for (int i = 1; i < v.size() - 1; ++i)
    for (int j = 1; j < v[0].size() - 1; ++j)
      if (v[i][j] == 'A')
        for (int k = 0; k < 4; ++k)
          if (v[i - 1][j - 1] == pat[k] &&
              v[i - 1][j + 1] == pat[(k + 1) % 4] &&
              v[i + 1][j + 1] == pat[(k + 2) % 4] &&
              v[i + 1][j - 1] == pat[(k + 3) % 4])
            ++ct;
  printf("%lld", ct);
}
