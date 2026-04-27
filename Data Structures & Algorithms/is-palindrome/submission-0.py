class Solution:
    def isPalindrome(self, s: str) -> bool:
        left, right = 0, len(s) - 1

        while left < right:
            if not self.is_alphanumeric(s[left]):
                left += 1
                continue
            if not self.is_alphanumeric(s[right]):
                right -= 1
                continue
            
            if s[left].lower() != s[right].lower():
                return False
            
            left += 1
            right -= 1
        
        return True
        
    def is_alphanumeric(self, s: str) -> bool:
        return (
            ord('A') <= ord(s) <= ord('Z')
            or ord('a') <= ord(s) <= ord('z')
            or ord('0') <= ord(s) <= ord('9')
        )