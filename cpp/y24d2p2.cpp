#include <bits/stdc++.h>
#define vat(rem, po) v[po+(po>=rem)]

using namespace std;

int main() {
  int ct = 0;
  while (true) {
    vector<int> v;
    string str, num;
    getline(cin, str);
    if (str.empty())
      break;
    istringstream stream(str);
    while (getline(stream, num, ' '))
      v.push_back(atoi(num.c_str()));
    if (v.size() < 2)
      throw;
    for (int rem = 0; rem < v.size(); ++rem) {
      bool work = 1;
      for (int i = 1; i < v.size() - 1; ++i)
        if ((vat(rem, i) > vat(rem, i - 1)) != (vat(rem, 1) > vat(rem, 0)) ||
            abs(vat(rem, i) - vat(rem, i - 1)) > 3 ||
            vat(rem, i) == vat(rem, i - 1)) {
          work = 0;
          break;
        }
      if (work) {
        ++ct;
        break;
      }
    }
  }
  cout << ct;
}
