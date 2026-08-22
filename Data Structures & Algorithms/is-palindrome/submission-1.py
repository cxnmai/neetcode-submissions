class Solution:
    def isPalindrome(self, s: str) -> bool:
        stripped = ""

        for l in s:
            if l.isalnum():
                stripped += l

        rev = ""

        for l in stripped:
            rev = l + rev
        
        stripped = stripped.lower()
        rev = rev.lower()
        
        return stripped == rev