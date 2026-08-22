class Solution:
    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        comps = {}
        for i, num in enumerate(numbers):
            if num in comps:
                return [comps[num] + 1, i + 1]
            comps[target - num] = i