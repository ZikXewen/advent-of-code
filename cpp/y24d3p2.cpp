#include <bits/stdc++.h>

using namespace std;

int main() {
  string str, inp;
  long long ans = 0;
  bool en = 1;
  while (getline(cin, inp)) {
    if (inp.empty())
      break;
    str += inp;
  }
  regex reg("mul\\((\\d+),(\\d+)\\)|do\\(\\)|don't\\(\\)");
  for (auto cur = sregex_iterator(str.begin(), str.end(), reg);
       cur != sregex_iterator(); ++cur)
    switch (cur->length()) {
    case 4:
      en = 1;
      break;
    case 7:
      en = 0;
      break;
    default:
      if (en)
        ans += stoi((*cur)[1]) * stoi((*cur)[2]);
    }
  printf("%lld", ans);
}
