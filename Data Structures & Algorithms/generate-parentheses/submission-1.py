class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        def backtracking(n: int, open: int, close: int, current: List[str], res: List[str]):
            if len(current) == 2*n:
                res.append("".join(current))
                return
            
            if open < n:
                current.append("(")
                backtracking(n, open + 1, close, current, res)
                current.pop()

            
            if close < open:
                current.append(")")
                backtracking(n, open, close + 1, current, res)
                current.pop()

        res = []

        backtracking(n, 0, 0, [], res)

        return res
        