#include <gtest/gtest.h>
#include <vector>
#include "../src/vector_operations/include/vector_operations.h"


using namespace std;

vector<float> v{ 10, 20, 30 };
vector<float> w{ 40, 50, 60 };
vector<float> q{ -10, -20, -30 };
vector<float> z{ 50, 60, 70 };

TEST(VectorOperationsTests, Add) {
    vector<float> add_expected_result{50, 70, 90};
    vector<float> add_actual_result = add(v,w);

    for (int i = 0; i < add_actual_result.size(); ++i) {
        EXPECT_EQ(add_actual_result[i], add_expected_result[i]) << "Vectors x and y differ at index " << i;
    }
}

TEST(VectorOperationsTests, Subtract) {
    vector<float> subtract_expected_result{-30, -30, -30};
    vector<float> subtract_actual_result = subtract(v,w);

    for (int i = 0; i < subtract_actual_result.size(); ++i) {
        EXPECT_EQ(subtract_actual_result[i], subtract_expected_result[i]) << "Vectors v and w differ at index " << i;
    }
}

TEST(VectorOperationsTests, VectorSum) {
    vector<float> vector_sum_expected_result{90, 110, 130};
    vector<float> vector_sum_actual_result = vector_sum({v, w, q, z});

    for (int i = 0; i < vector_sum_actual_result.size(); ++i) {
        EXPECT_EQ(vector_sum_actual_result[i], vector_sum_expected_result[i]) << "Vectors v and w differ at index " << i;
    }
}

TEST(VectorOperationsTests, ScalarMultiply) {
    vector<float> scalar_multiply_expected_result{30, 60, 90};
    vector<float> scalar_multiply_actual_result = scalar_multiply(3, v);

    for (int i = 0; i < scalar_multiply_actual_result.size(); ++i) {
        EXPECT_EQ(scalar_multiply_actual_result[i], scalar_multiply_expected_result[i]) << "Vectors v and w differ at index " << i;
    }
}

TEST(VectorOperationsTests, VectorMean) {
    vector<float> vector_mean_expected_result{3, 4};
    vector<float> vector_mean_actual_result = vector_mean({{1, 2}, {3, 4}, {5, 6}});

   for (int i = 0; i < vector_mean_actual_result.size(); ++i) {
       EXPECT_EQ(vector_mean_actual_result[i], vector_mean_expected_result[i]) << "Vectors v and w differ at index " << i;
   }
}

TEST(VectorOperationsTests, Dot) {
    float dot_expected_result = 32;
    float dot_actual_result = dot({1, 2, 3}, {4, 5, 6});

    EXPECT_EQ(dot_actual_result, dot_expected_result) << "Results differ.";
}

TEST(VectorOperationsTests, SumOfSquares) {
    float sum_of_squares_expected_result = 14;
    float sum_of_squares_actual_result = sum_of_squares({1, 2, 3});

    EXPECT_EQ(sum_of_squares_expected_result, sum_of_squares_actual_result) << "Results differ.";
}

TEST(VectorOperationsTests, Magnitude) {
    float magnitude_expected_result = 5;
    float magnitude_actual_result = magnitude({3, 4});

    EXPECT_EQ(magnitude_expected_result, magnitude_actual_result) << "Results differ.";
}

TEST(VectorOperationsTests, SquaredDistance){
    float squared_distance_expected_result = 8;
    float squared_distance_actual_result = squared_distance({3, 4}, {5, 6});

    EXPECT_EQ(squared_distance_expected_result, squared_distance_actual_result) << "Results differ.";
}

TEST(VectorOperationsTests, Distance){
    float distance_expected_result = 5;
    float distance_actual_result = distance({4, 5}, {1, 1});

    EXPECT_EQ(distance_expected_result, distance_actual_result) << "Results differ.";
}

