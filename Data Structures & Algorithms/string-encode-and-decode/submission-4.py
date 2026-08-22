class Solution:

    def encode(self, strs: List[str]) -> str:
        encoded = ""

        for s in strs:
            encoded += chr(len(s)) + s
        
        return encoded

    def decode(self, s: str) -> List[str]:
        decoded = []
        
        start = 0
        end = 0
        while end < len(s):
            end = start + ord(s[start]) + 1
            decoded.append(s[start + 1 : end])
            print(decoded)
            start = end
        
        return decoded