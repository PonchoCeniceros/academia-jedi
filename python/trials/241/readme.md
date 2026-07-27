![Algorithm](https://raw.githubusercontent.com/PonchoCeniceros/competitive_programming_solutions/main/exercises/241_different_ways_to_add_parentheses/imgs/algorithm.png)


### Memoization

```python
def diffWaysToCompute(self, expression: str) -> List[int]:
    # algoritmo recursivo
    memo = {}

    def solver(E, Ops):
        key = (tuple(E), tuple(Ops))

        if key in memo:
            return memo[key]

        if len(E) == 1:
            memo[key] = E
            return memo[key]

        ans = []
        for i, op in enumerate(Ops):
            for x in solver(E[0 : i + 1], Ops[0:i]):
                for y in solver(E[i + 1 : len(E)], Ops[i + 1 : len(Ops)]):
                    ans.append(
                        x + y if op == "+" else (x - y if op == "-" else x * y)
                    )
        memo[key] = ans
        return memo[key]

    # procesar la entrada
    tokens = re.findall(r"(?:[1-9][0-9]|[0-9])|[+\-*]", expression)
    E = [int(t) for t in tokens if t not in ["+", "-", "*"]]
    Ops = [t for t in tokens if t in ["+", "-", "*"]]

    # implementacion de la funcion
    return E if len(E) == 1 else solver(E, Ops)
```

### Memoization with `@cache`

```python
from functools import cache

def diffWaysToCompute(self, expression: str) -> List[int]:
    # algoritmo recursivo
    @cache
    def solver(E: Tuple, Ops: Tuple) -> List[int]:
        if len(E) == 1:
            return list(E)

        ans: List = []
        for i, op in enumerate(Ops):
            for x in solver(E[0 : i + 1], Ops[0:i]):
                for y in solver(E[i + 1 : len(E)], Ops[i + 1 : len(Ops)]):
                    ans.append(
                        x + y if op == "+" else (x - y if op == "-" else x * y)
                    )
        return ans

    # procesar la entrada
    tokens = re.findall(r"(?:[1-9][0-9]|[0-9])|[+\-*]", expression)
    E: Tuple = tuple(int(t) for t in tokens if t not in ["+", "-", "*"])
    Ops: Tuple = tuple(t for t in tokens if t in ["+", "-", "*"])

    # implementacion de la funcion
    return list(E) if len(E) == 1 else solver(E, Ops)
```
