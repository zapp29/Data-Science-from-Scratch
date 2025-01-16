//
// Created by piotr on 25.01.2023.
//
#include <vector>
#include <stdexcept>
#include <cmath>
#include <vector_operations.h>

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
    unsigned long len = vectors.at(0).size();
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
            result.at(i) = v.at(i) + result.at(i);
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


vector<float> vector_mean(const vector<vector<float>>& vectors) {
    unsigned long n = vectors.size();
    return scalar_multiply(float(1.0)/float(n), vector_sum(vectors));
}


float dot(const vector<float>& v, const vector<float>& w) {
    unsigned long size_v = v.size();
    unsigned long size_w = w.size();
    if(size_v != size_w) {
        throw invalid_argument("vectors must be of the same size");
    }
    float sum = 0;
    for (int i = 0; i < size_v; ++i) {
        sum += v.at(i) * w.at(i);
    }
    return sum;
}


float sum_of_squares(const vector<float>& v) {
    return dot(v, v);
}


float magnitude(const vector<float>& v) {
    return sqrt(sum_of_squares(v));
}


float squared_distance(const vector<float>& v, const vector<float>& w) {
    return sum_of_squares(subtract(v, w));
}


float distance(const vector<float>& v, const vector<float>& w) {
    return magnitude(subtract(v, w));
}
