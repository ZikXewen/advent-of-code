#include <bits/stdc++.h>

int main() {
  std::vector<int> a, b;
  int x, y;
  while (scanf("%d%d", &x, &y) != -1) {
    a.push_back(x);
    b.push_back(y);
  }
  std::sort(a.begin(), a.end());
  std::sort(b.begin(), b.end());
  for(int i = 0; i < a.size(); ++i)
    a[i] = std::abs(a[i] - b[i]);
  printf("%d\n", std::accumulate(a.begin(), a.end(), 0));
}
