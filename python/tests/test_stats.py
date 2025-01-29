from math import sqrt
from stats import mean, median, quantile, mode, data_range, de_mean, variance, standard_deviation, interquartile_range, covariance, correlation


def test_mean():
    assert mean([1, 2, 3]) == 2
    assert mean([1, 2]) == 1.5

def test_median():
    assert median([1, 2, 3]) == 2
    assert median([1, 2]) == 1.5

def test_quantile():
    assert quantile([1, 2, 3], 0.5) == 2
    assert quantile([1, 2, 3], 0.25) == 1

def test_mode():
    assert mode([1, 2, 2, 3]) == 2

def test_data_range():
    assert data_range([1, 2, 3]) == 2

def test_de_mean():
    assert de_mean([1, 2, 3]) == [-1, 0, 1] 

def test_variance():
    assert variance([1, 2, 3]) == 1

def test_standard_deviation():
    assert standard_deviation([1, 2, 3]) == sqrt(1) 

def test_interquartile_range():
    assert interquartile_range([1, 2, 3]) == 2

def test_covariance():
    assert covariance([1, 2, 3], [4, 5, 6]) == 1

def test_correlation():
    assert correlation([1, 2, 3], [4, 5, 6]) == 1
