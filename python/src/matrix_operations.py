from vector_operations import Vector

Matrix = list[list[float]]

def shape(m: Matrix) -> tuple[int, int]:
    return len(m), len(m[0]) if m else 0

def get_row(m: Matrix, r: int) -> Vector:
    return m[r]

def get_column(m: Matrix, c: int) -> Vector:
    return [row[c] for row in m]

def make_matrix(num_rows: int, num_cols: int, entry_fn: callable[[int, int], float]) -> Matrix:
    return [
        [
            entry_fn(r,c)
            for c in range(num_cols)
        ]
        for r in range(num_rows)
    ]

def identity_matrix(n: int) -> Matrix:
    return make_matrix(n, n, lambda i,j: 1 if i==j else 0)


