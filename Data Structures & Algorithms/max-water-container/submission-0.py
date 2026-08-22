class Solution:
    def maxArea(self, heights: List[int]) -> int:
        res = 0

        l = 0

        r  = len(heights) - 1

        while l < r:
            cap = min(heights[l], heights[r]) 
            area = cap * (r - l)
            if area > res: res = area
            if cap == heights[l]:
                l += 1
            elif cap == heights[r]:
                r -= 1
        
        return res
