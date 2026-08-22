class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:

        result = []

        if nums.count(0) > 1:
            return [0] * len(nums)
        
        prod = 1

        for num in nums:
            if num != 0:
                prod *= num

        if nums.count(0) == 1:
            result = [0] * len(nums)
            result[nums.index(0)] = prod
            return result
        
        for num in nums:
            result.append(prod // num)

        
        return result