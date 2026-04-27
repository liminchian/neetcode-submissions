class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        rows = set()
        cols = set()
        squares = set()

        for r in range(9):
            for c in range(9):
                val = board[r][c]

                if val == ".":
                    continue
                
                if (
                    f"{r}{val}" in rows
                    or f"{c}{val}" in cols
                    or f"{int(r / 3)}{int(c / 3)}{val}" in squares
                ):
                    return False
                
                rows.add(f"{r}{val}")
                cols.add(f"{c}{val}")
                squares.add(f"{int(r / 3)}{int(c / 3)}{val}")
        
        return True
