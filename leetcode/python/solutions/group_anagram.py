class Solution(object):
    def groupAnagrams(self, strs):
        """
        :type strs: List[str]
        :rtype: List[List[str]]
        """
        hash_map = {}

        for s in strs:
            k = self.str_to_count(s)

            if k in hash_map:
                hash_map[k].append(s)
            else:
                hash_map[k] = [s]

        return [v for v in hash_map.values()]

    def str_to_count(self, s):
        counts = [0] * 26
        for c in s:
            counts[ord(c) - ord("a")] += 1
        return tuple(counts)


if __name__ == "__main__":
    print(Solution().groupAnagrams(["abc", "bca",  "aaa"]))
