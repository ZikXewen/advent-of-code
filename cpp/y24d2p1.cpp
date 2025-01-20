#include <bits/stdc++.h>

using namespace std;

int main() {
  int ct = 0;
  while (true) {
    vector<int> v;
    string str, num;
    bool work = 1;
    getline(cin, str);
    if (str.empty())
      break;
    istringstream stream(str);
    while (getline(stream, num, ' '))
      v.push_back(atoi(num.c_str()));
    if (v.size() < 2)
      throw;
    for (int i = 1; i < v.size(); ++i)
      if ((v[i] > v[i - 1]) != (v[1] > v[0]) || abs(v[i] - v[i - 1]) > 3 ||
          v[i] == v[i - 1]) {
        work = 0;
        break;
      }
    ct += work;
  }
  cout << ct;
}
