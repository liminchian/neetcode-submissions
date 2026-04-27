class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        freq = {}

        for num in nums:
            freq[num] = freq.get(num, 0) + 1
        
        sorted_freq = dict(sorted(freq.items(), key=lambda i: i[1], reverse=True))

        return list(sorted_freq.keys())[:k]