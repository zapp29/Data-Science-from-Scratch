from math import sqrt

Vector = list[float]

def add(v1: Vector, v2: Vector) -> Vector:
    assert len(v1) == len(v2), "Vectors must be of the same length"
    return [v1[i] + v2[i] for i in range(len(v1))]

def subtract(v1: Vector, v2: Vector) -> Vector:
    assert len(v1) == len(v2), "Vectors must be of the same length"
    return [v1[i] - v2[i] for i in range(len(v1))]

def vector_sum(vectors: list[Vector]) -> Vector:
    assert len(vectors) > 0, "List of vectors must not be empty"
    num_elements = len(vectors[0])
    assert all(len(v) == num_elements for v in vectors), "All vectors must be of the same length"
    return [
        sum(vector[i] for vector in vectors) 
        for i in range(num_elements)
    ]

def scalar_multiply(c: float, v: Vector) -> Vector:
    return [c * v_i for v_i in v]

def vector_mean(vectors: list[Vector]) -> Vector:
    n = len(vectors)
    return scalar_multiply(1/n, vector_sum(vectors))

def dot(v1: Vector, v2: Vector) -> float:
    assert len(v1) == len(v2), "Vectors must be of the same length"
    return sum(
        v1[i] * v2[i] for i in range(len(v1))
    )

def sum_of_squares(v: Vector) -> float:
    return dot(v, v)

def magnitude(v: Vector) -> float:
    return sqrt(sum_of_squares(v))

def squared_distance(v1: Vector, v2: Vector) -> float:
    return sum_of_squares(subtract(v1, v2))

def distance(v1: Vector, v2: Vector) -> float:
    return sqrt(squared_distance(v1, v2))
