class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        for row in board:
            seen = set()
            for val in row:
                if val == ".": continue
                if val in seen:
                    return False
                else:
                    seen.add(val)
            
        boxes = [[],  [],  [],  [],  [],  [],  [],  [],  []]
        
        for i in range(len(board)):
            col = [row[i] for row in board]
            seen = set()
            for j, val in enumerate(col):
                if val == ".": continue
                if val in seen:
                    return False
                else:
                    seen.add(val)
                
                boxes[(i // 3) * 3 + j // 3].append(val)
        
        for box in boxes:
            seen = set()
            for val in box:
                if val in seen:
                    return False
                else:
                    seen.add(val)
        
        return True