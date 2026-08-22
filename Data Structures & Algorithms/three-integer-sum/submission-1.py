class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        pairs = {}
        for i in range(len(nums)):
            for j in range(i + 1, len(nums)):
                pairs[frozenset([i, j])] = (nums[i] + nums[j]) * -1

        res_set = set()

        for i, num in enumerate(nums):
            for pair in pairs:
                if pairs[pair] == num and i not in pair:
                    triplet = set(pair)
                    triplet.add(i)
                    triplet = frozenset(triplet)
                    if len(triplet) == 3: res_set.add(triplet)

        nums_res_set = set()

        for fset in res_set:
            triplet = list(fset)
            triplet[0] = nums[triplet[0]]
            triplet[1] = nums[triplet[1]]
            triplet[2] = nums[triplet[2]]
            triplet.sort()
            nums_res_set.add(tuple(triplet))
        
        res = []

        for triplet in nums_res_set:
            res.append(list(triplet))
        
        return res