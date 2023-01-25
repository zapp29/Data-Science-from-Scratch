//
// Created by piotr on 25.01.2023.
//
#include <vector>
#include <stdexcept>

using namespace std;

vector<float> add(vector<float> v, vector<float> w)
{
    if(v.size() != w.size())
    {
        throw invalid_argument("matrices must be of the same size");
    }
    else {
        vector<float> result(v.size(), 0);
        for (int i = 0; i < v.size(); ++i) {
            result.at(i) = v.at(i) + w.at(i);
        }
        return result;
    }
}

vector<float> subtract(vector<float> v, vector<float> w)
{
    if(v.size() != w.size())
    {
        throw invalid_argument("matrices must be of the same size");
    }
    else {
        vector<float> result(v.size(), 0);
        for (int i = 0; i < v.size(); ++i) {
            result.at(i) = v.at(i) - w.at(i);
        }
        return result;
    }
}

vector<float> vector_sum(vector<vector<float>> vectors) {
    int len = int(vectors.at(0).size());
    for (int i = 1; i<vectors.size(); ++i) {
        if(vectors.at(i).size() == len) {
            len = int(vectors.at(i).size());
        }
        else {
            throw invalid_argument("vectors must be of the same size");
        }
    }
    vector<float> result(len, 0);
    for (vector<float> v: vectors) {
        for (int i = 0; i < len; ++i) {
            result.at(i) = v.at(i) - result.at(i);
        }
    }
    return result;
}


vector<float> scalar_multiply(float c, vector<float> v) {
    vector<float> result(v.size(), 0);
    for (int i = 0; i < v.size(); ++i) {
        result.at(i) = c * v.at(i);
    }
    return result;
}

/*
def vector_mean(vectors: List[Vector]) -> Vector:
    """Computes the element-wise average"""
    n = len(vectors)
    return scalar_multiply(1/n, vector_sum(vectors))

assert vector_mean([[1, 2], [3, 4], [5, 6]]) == [3, 4]
 */



/*
def dot(v: Vector, w: Vector) -> float:
    """Computes v_1 * w_1 + ... + v_n * w_n"""
    assert len(v) == len(w), "vectors must be same length"

    return sum(v_i * w_i for v_i, w_i in zip(v, w))

assert dot([1, 2, 3], [4, 5, 6]) == 32  # 1 * 4 + 2 * 5 + 3 * 6
 */


/*
def sum_of_squares(v: Vector) -> float:
    """Returns v_1 * v_1 + ... + v_n * v_n"""
    return dot(v, v)

assert sum_of_squares([1, 2, 3]) == 14  # 1 * 1 + 2 * 2 + 3 * 3
 */

