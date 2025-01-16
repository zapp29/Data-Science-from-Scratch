#include <gtest/gtest.h>
#include <vector>
#include <tuple>
#include "../src/matrix_operations/include/matrix_operations.h"



using namespace std;

vector<vector<float>> A{{1,2,3,4},{5,6,7,8},{9,10,11,12}};

TEST(MatrixOperationsTests, Shape) {
    tuple<float, float> shape_expected_result{3, 4};
    tuple<float, float> shape_actual_result = shape(A);

    EXPECT_EQ(shape_expected_result, shape_actual_result) << "Tuples differ." << endl;
}