class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        comps = {}

        for i, num in enumerate(nums):
            if num in comps:
                return [comps[num], i]

            comps[target - num] = i
        
        return [-1, -1]