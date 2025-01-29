from math import erf, exp, pi, sqrt
import random


def uniform_pdf(x: float) -> float:
    return 1 if 0 <= x < 1 else 0

def uniform_cdf(x: float) -> float:
    if x < 0:
        return 0
    elif x < 1:
        return x
    else:
        return 1

def normal_pdf(x: float, mu: float = 0, sigma: float = 1) -> float:
    return (1 / (sigma * sqrt(2 * pi))) * exp(-0.5 * ((x - mu) / sigma) ** 2)

def normal_cdf(x: float, mu: float = 0, sigma: float = 1) -> float:
    return (1 + erf((x - mu) / (sigma * sqrt(2)))) / 2

def inverse_normal_cdf(p: float, mu: float = 0, sigma: float = 1, tolerance: float = 1e-5) -> float:
    if mu != 0 or sigma != 1:
        return mu + sigma * inverse_normal_cdf(p, tolerance=tolerance)
    low_z = -10.0
    hi_z = 10.0
    while hi_z - low_z > tolerance:
        mid_z = (low_z + hi_z) / 2
        mid_p = normal_cdf(mid_z)
        if mid_p < p:
            low_z = mid_z
        else:
            hi_z = mid_z
    return mid_z

def bernoulli_trial(p: float) -> int:
    return 1 if random.random() < p else 0

def binomial(n: int, p: float) -> int:
    return sum(bernoulli_trial(p) for _ in range(n))
