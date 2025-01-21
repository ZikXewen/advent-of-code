#include <bits/stdc++.h>

using namespace std;

int main() {
  bool rules[100][100] = {};
  long long ans = 0;
  string str, num;
  auto comp = [&](int l, int r) { return rules[l][r]; };
  while (getline(cin, str)) {
    if (str.empty())
      break;
    rules[stoi(str.substr(0, 2))][stoi(str.substr(3))] = 1;
  }
  while (getline(cin, str)) {
    if (str.empty())
      break;
    istringstream stream(str);
    vector<int> v;
    while (getline(stream, num, ','))
      v.push_back(stoi(num));
    if (!is_sorted(v.begin(), v.end(), comp)) {
      sort(v.begin(), v.end(), comp);
      ans += v[v.size() >> 1];
    }
  }
  printf("%lld", ans);
}
