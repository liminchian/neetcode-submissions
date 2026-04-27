class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        stack = []

        for s in tokens:
            if s == "+":
                stack.append(stack.pop() + stack.pop())
            elif s == "-":
                last, last_2nd = stack.pop(), stack.pop()
                stack.append(last_2nd - last)
            elif s == "*":
                stack.append(stack.pop() * stack.pop())
            elif s == "/":
                last, last_2nd = stack.pop(), stack.pop()
                stack.append(int(float(last_2nd) / last))
            else:
                stack.append(int(s))
        
        return stack[0]
