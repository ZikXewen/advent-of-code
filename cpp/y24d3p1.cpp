#include <bits/stdc++.h>

using namespace std;

int main() {
  string str, inp;
  long long ans = 0;
  while (getline(cin, inp)) {
    if (inp.empty())
      break;
    str += inp;
  }
  regex reg("mul\\((\\d+),(\\d+)\\)");
  for (auto cur = sregex_iterator(str.begin(), str.end(), reg);
       cur != sregex_iterator(); ++cur)
    ans += stoi((*cur)[1]) * stoi((*cur)[2]);
  printf("%lld", ans);
}
