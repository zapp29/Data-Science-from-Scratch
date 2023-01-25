#include <gtest/gtest.h>
#include <vector>
#include "vector_operations.h"

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
