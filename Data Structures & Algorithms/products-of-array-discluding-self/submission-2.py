class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        left = []
        left.append(nums[0])

        for i in range(1, len(nums)):
            left.append(left[i - 1] * nums[i])
        
        right = []
        right.append(nums[-1])

        for i in range (len(nums) - 2, -1, -1):
            right.append(right[-1] * nums[i])

        right.reverse()

        print(left, right)
        
        result = []

        for i in range(len(nums)):
            if i == 0:
                result.append(right[1])
            elif i == len(nums) - 1:
                result.append(left[-2])
            else:
                result.append(left[i - 1] * right[i + 1 ])
        
        return result
