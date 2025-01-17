from vector_operations import add, subtract, scalar_multiply, dot, vector_sum, vector_mean, sum_of_squares, magnitude, squared_distance, distance
from math import sqrt

def test_add():
    assert add([1, 2, 3], [4, 5, 6]) == [5, 7, 9]
    assert add([1, 2], [1, 2]) == [2, 4]

def test_subtract():
    assert subtract([1, 2, 3], [4, 5, 6]) == [-3, -3, -3]
    assert subtract([1, 2], [1, 2]) == [0, 0]

def test_vector_sum():
    assert vector_sum([[1, 2, 3], [4, 5, 6]]) == [5, 7, 9]
    assert vector_sum([[1, 2], [1, 2]]) == [2, 4]

def test_scalar_multiply():
    assert scalar_multiply(2, [1, 2, 3]) == [2, 4, 6]
    assert scalar_multiply(3, [1, 2]) == [3, 6]

def test_vector_mean():
    assert vector_mean([[1, 2, 3], [4, 5, 6]]) == [2.5, 3.5, 4.5]
    assert vector_mean([[1, 2], [1, 2]]) == [1, 2]

def test_dot():
    assert dot([1, 2, 3], [4, 5, 6]) == 32
    assert dot([1, 2], [1, 2]) == 5

def test_sum_of_squares():
    assert sum_of_squares([1, 2, 3]) == 14
    assert sum_of_squares([1, 2]) == 5

def test_magnitude():
    assert magnitude([1, 2, 3]) == sqrt(14)
    assert magnitude([1, 2]) == sqrt(5)

def test_squared_distance():
    assert squared_distance([1, 2, 3], [4, 5, 6]) == 27
    assert squared_distance([1, 2], [1, 2]) == 0

def test_distance():
    assert distance([1, 2, 3], [4, 5, 6]) == sqrt(27)
    assert distance([1, 2], [1, 2]) == 0
