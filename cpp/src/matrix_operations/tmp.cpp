
#include <vector>
#include <iostream>

using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    explicit TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

TreeNode one = TreeNode(300);
TreeNode two = TreeNode(200, &one, nullptr);
TreeNode zero = TreeNode(100, nullptr, &two);


class Solution {
    void recursiveCall(TreeNode *node, vector<int>& ans){
        if(node==nullptr) {
            return;
        }
        ans.push_back(node->val);
        recursiveCall(node->left, ans);
        recursiveCall(node->right, ans);

    }
public:
    vector<int> preorderTraversal(TreeNode* root) {
        vector<int> dest;
        recursiveCall(root, dest);
        return dest;
    }
};

int main(){
    Solution solution = Solution();
    vector<int> solutions = solution.preorderTraversal(&zero);
    for(auto it=solutions.begin(); it<solutions.end(); it++){
        cout << *it << endl;
    }
}