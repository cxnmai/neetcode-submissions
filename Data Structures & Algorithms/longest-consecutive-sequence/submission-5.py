class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        if len(nums) == 0: return 0
        nums_set = set(nums)

        starts = set()

        for num in nums:
            if num - 1 not in nums_set:
               starts.add(num)
        
        print(starts)
        res = 1
        tmp = 1
        
        for num in starts:
            start = num
            while start + 1 in nums_set:
                tmp += 1
                if tmp > res: res = tmp
                start += 1
            tmp = 1

        return res