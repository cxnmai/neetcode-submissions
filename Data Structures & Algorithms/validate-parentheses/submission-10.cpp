class Solution {
public:
    bool isValid(string s) {
        vector<char> stack;

        unordered_map<char, char> matches = {{'}', '{'}, {')', '('}, {']', '['}};

        for (char c : s) {
            if (c == '(' || c == '[' || c == '{') stack.push_back(c);
            else if (stack.size() == 0) return false;
            else if (matches[c] == stack.back()) {
                stack.pop_back();
            }
            else return false;
        }

        if (stack.size() != 0) return false;

        return true;        
    }
};
