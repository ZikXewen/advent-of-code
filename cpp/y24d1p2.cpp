#include <bits/stdc++.h>

const int SIZE = 1e5+5;

int a[SIZE], b[SIZE];
int x, y, z;
long long ans;

int main() {
  while(scanf("%d%d", &x, &y) != -1)
    a[z++] = x, ++b[y];
  for(int i = 0; i < z; ++i)
    ans += a[i] * b[a[i]];
  printf("%lld", ans);
}
