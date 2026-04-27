class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        mp = {}

        for s in strs:
            anagram = str(sorted(s))
            if anagram not in mp:
                mp[anagram] = [s]
            else:
                mp[anagram].append(s)
        
        return list(mp.values())