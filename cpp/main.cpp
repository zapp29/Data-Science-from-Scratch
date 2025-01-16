#include <iostream>
#include <vector>
#include <stdexcept>
#include "vector_operations.cpp"

using namespace std;

int main()
{
    vector<int> v{ 10, 20, 30 };
    vector<int> w{ 40, 50, 60 };

    vector<int> result = add(v,w);

    try {
        cout << "1:" << result.at(0) << endl;
        cout << "2:" << result.at(1) << endl;
        cout << "3:" << result.at(2) << endl;
    }
    catch (invalid_argument& e) {
        cerr << e.what() <<endl;
        return -1;
    }
    return 0;
}