class Solution:
    def trap(self, height: List[int]) -> int:

        def trapped(seg: List[int], cap: int):
            sum = 0
            for h in seg:
                sum += cap - h
            return sum

        res = 0
        seg = []
        cap = 0

        for i, h in enumerate(height):
            if h >= cap:
                res += trapped(seg, cap)
                cap = h
                seg = []
            elif i == len(height) - 1:
                seg.append(h)
                while len(seg) != 0:
                    cap = max(seg)
                    wall = seg.index(cap)
                    res += trapped(seg[0 : wall], cap)
                    seg = seg[wall + 1: ]
            else:
                seg.append(h)
            
        
        return res
