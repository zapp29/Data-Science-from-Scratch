#include <gtest/gtest.h>
extern "C" {
    #include "../src/include/scalar_operations.h"
}

class ScalarOperationsTest : public ::testing::Test {
protected:
    void SetUp() override {
        // Setup code if needed
    }
};

TEST_F(ScalarOperationsTest, AdditionTest) {
    EXPECT_FLOAT_EQ(add(2.0f, 3.0f), 5.0f);
    EXPECT_FLOAT_EQ(add(-2.0f, 3.0f), 1.0f);
    EXPECT_FLOAT_EQ(add(0.0f, 0.0f), 0.0f);
}

TEST_F(ScalarOperationsTest, SubtractionTest) {
    EXPECT_FLOAT_EQ(subtract(5.0f, 3.0f), 2.0f);
    EXPECT_FLOAT_EQ(subtract(-2.0f, 3.0f), -5.0f);
    EXPECT_FLOAT_EQ(subtract(0.0f, 0.0f), 0.0f);
}

TEST_F(ScalarOperationsTest, MultiplicationTest) {
    EXPECT_FLOAT_EQ(multiply(2.0f, 3.0f), 6.0f);
    EXPECT_FLOAT_EQ(multiply(-2.0f, 3.0f), -6.0f);
    EXPECT_FLOAT_EQ(multiply(0.0f, 5.0f), 0.0f);
}

TEST_F(ScalarOperationsTest, DivisionTest) {
    EXPECT_FLOAT_EQ(divide(6.0f, 2.0f), 3.0f);
    EXPECT_FLOAT_EQ(divide(-6.0f, 2.0f), -3.0f);
    EXPECT_FLOAT_EQ(divide(0.0f, 5.0f), 0.0f);
} 