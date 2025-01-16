//
// Created by piotr on 25.01.2023.
//
#include <vector>

using namespace std;

vector<float> add(vector<float> v, vector<float> w);
vector<float> subtract(vector<float> v, vector<float> w);
vector<float> vector_sum(vector<vector<float>> vectors);
vector<float> scalar_multiply(float c, vector<float> v);
vector<float> vector_mean(const vector<vector<float>>& vectors);
float dot(const vector<float>& v, const vector<float>& w);
float sum_of_squares(const vector<float>& v);
float magnitude(const vector<float>& v);
float squared_distance(const vector<float>& v, const vector<float>& w);
float distance(const vector<float>& v, const vector<float>& w);
