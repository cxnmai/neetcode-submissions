class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        sets = {}

        for i, s in enumerate(strs):
            letters = {}
            for l in s:
                if l not in letters:
                    letters[l] = 1
                else:
                    letters[l] += 1
                
            fletters = frozenset(letters.items())
            
            if fletters not in sets:
                sets[fletters] = [strs[i]]
            else:
                sets[fletters].append(strs[i])
        
        return list(sets.values())
