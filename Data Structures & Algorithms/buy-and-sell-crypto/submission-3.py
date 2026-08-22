class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        prev = float("inf")
        profit = 0

        for p in prices:
            prev = min(p, prev)
            if p - prev > profit:
                profit = p - prev
        
        return profit