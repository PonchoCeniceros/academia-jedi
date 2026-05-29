## Exercise 338

| Number (`i`) | Binary | Relation | DP equation | Result (`memo[i]`) |
| :--- | :--- | :--- | :--- | :--- |
| **0** | `000` | Base | - | **0** |
| **1** | `001` | 0 + extra bit (sub=1) | `memo[0] + 1` | **1** |
| **2** | `010` | 0 + extra bit (sub=2) | `memo[0] + 1` | **1** |
| **3** | `011` | 1 + extra bit | `memo[1] + 1` | **2** |
| **4** | `100` | 0 + extra bit (sub=4) | `memo[0] + 1` | **1** |
| **5** | `101` | 1 + extra bit | `memo[1] + 1` | **2** |
| **6** | `110` | 2 + extra bit | `memo[2] + 1` | **2** |
| **7** | `111` | 3 + extra bit | `memo[3] + 1` | **3** |

```python
memo = [0] * (n + 1)
sub = 1

for i in range(1, n + 1):
    if sub * 2 == i:
        sub = i

    memo[i] = memo[i - sub] + 1

return memo[n]
```

## Diferences beetwen Memoization and Tabulation

| Feature | Tabulation (Bottom-Up) | Memoization (Top-Down) |
| :--- | :--- | :--- |
| **Implementation** | Iterative (`for`,`while` loops) | Recursive |
| **Data structure** | Table / Array | Diccionary / Hash Map / Caché |
| **filling order** | 0 to `n` | On demand (as per the recursion) |
