
from typing import List, Optional
import random
import math

EPS = 1e-9

def generateTestData(count: int, seed: Optional[int] = None) -> List[float]:
    random.seed(seed)
    mu = [random.uniform(0, 10) for _ in range(count)]

    # normalize sum to 0
    s = sum(mu)
    mu = [x - s / count for x in mu]

    assert abs(sum(mu)) < EPS
    return mu

def rankedIndices(arr: List[float]) -> List[int]:
    return sorted(range(len(arr)), key=lambda i: arr[i], reverse=True)

def generateAdjMatrix(mu: List[float], seed: Optional[int] = None) -> List[List[int]]:
    random.seed(seed)
    n = len(mu)
    adj = [[0] * n for _ in range(n)]



    for k in range(20):
        for i in range(n):
            for j in range(i + 1, n):
                rnd = random.gauss(mu[i]-mu[j], 1) 
                if rnd > 0:
                    adj[i][j] += 1
                if rnd < 0:
                    adj[j][i] += 1
            
    return adj


if __name__ == "__main__":
    testData = generateTestData(10, 1)
    adj = generateAdjMatrix(testData, 1)
    print("-"*5 + " Test Data " + "-"*5)
    print(*testData)
    print("-"*5 + " Ranking " + "-"*5)
    print(*rankedIndices(testData))
    print("-"*5 + " Adj Matrix " + "-"*5)
    for row in adj:
        print(' '.join(map(str, row)))



