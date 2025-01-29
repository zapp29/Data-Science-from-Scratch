from math import sqrt
from typing import Counter
from vector_operations import dot


def mean(xs: list[float]) -> float:
    return sum(xs) / len(xs)

def _median_even(xs: list[float]) -> float:
    sorted_xs = sorted(xs)
    mid = len(sorted_xs) // 2
    return (sorted_xs[mid - 1] + sorted_xs[mid]) / 2

def _median_odd(xs: list[float]) -> float:
    sorted_xs = sorted(xs)
    mid = len(sorted_xs) // 2
    return sorted_xs[mid]

def median(xs: list[float]) -> float:
    return _median_even(xs) if len(xs) % 2 == 0 else _median_odd(xs)

def quantile(xs: list[float], p: float) -> float:
    p_index = int(p * len(xs))
    return sorted(xs)[p_index]

def mode(xs: list[float]) -> float:
    counts = Counter(xs)
    max_count = max(counts.values())
    return [x for x, count in counts.items() if count == max_count][0]

def data_range(xs: list[float]) -> float:
    return max(xs) - min(xs)

def de_mean(xs: list[float]) -> list[float]:
    x_bar = mean(xs)
    return [x - x_bar for x in xs]

def variance(xs: list[float]) -> float:
    n = len(xs)
    deviations = de_mean(xs)
    return sum(x ** 2 for x in deviations) / (n - 1)

def standard_deviation(xs: list[float]) -> float:
    return sqrt(variance(xs))   

def interquartile_range(xs: list[float]) -> float:
    return quantile(xs, 0.75) - quantile(xs, 0.25)

def covariance(xs: list[float], ys: list[float]) -> float:
    n = len(xs)
    return dot(de_mean(xs), de_mean(ys)) / (n - 1)

def correlation(xs: list[float], ys: list[float]) -> float:
    stdev_x = standard_deviation(xs)
    stdev_y = standard_deviation(ys)
    if stdev_x > 0 and stdev_y > 0:
        return covariance(xs, ys) / stdev_x / stdev_y
    else:
        return 0
