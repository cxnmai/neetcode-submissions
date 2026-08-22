class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        freqs = {}

        for i, num in enumerate(nums):
            if num not in freqs:
                freqs[num] = 1
            else:
                freqs[num] += 1
        
        nums = list(freqs.keys())
        print(nums)
        freqs = list(freqs.values())
        print(freqs)
        result = []

        for i in range(k):
            max_index = freqs.index(max(freqs))
            freqs[max_index] = 0
            result.append(nums[max_index])
        
        return result