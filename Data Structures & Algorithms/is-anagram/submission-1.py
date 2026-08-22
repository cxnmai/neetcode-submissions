class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        if len(s) != len(t):
            return False
        
        letters = {}
        target_letters = {}

        for i in range(len(s)):
            if s[i] not in letters:
                letters[s[i]] = 0
            else:
                letters[s[i]] += 1
            
            if t[i] not in target_letters:
                target_letters[t[i]] = 0
            else:
                target_letters[t[i]] += 1
        
        return letters == target_letters
        
