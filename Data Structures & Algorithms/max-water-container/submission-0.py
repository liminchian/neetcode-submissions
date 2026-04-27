class Solution:
    def maxArea(self, heights: List[int]) -> int:
        left, right = 0, len(heights) - 1
        max_water_container = 0

        while left < right:
            current_area = min(heights[left], heights[right]) * (right - left)
            max_water_container = max(max_water_container, current_area)

            if heights[left] >= heights[right]:
                right -= 1
            else:
                left += 1
        
        return max_water_container