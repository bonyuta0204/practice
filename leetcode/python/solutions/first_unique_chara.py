class Solution(object):
    def firstUniqChar(self, s):
        """
        :type s: str
        :rtype: int
        """

        counts = {}

        for c in s:
            if c not in counts:
                counts[c] = 1
            else:
                counts[c] += 1
        for i, c in enumerate(s):
            if counts[c] == 1:
                return i
        return -1


if __name__ == "__main__":
    assert Solution().firstUniqChar("leetcode") == 0
    assert Solution().firstUniqChar("loveleetcode") == 2
    assert Solution().firstUniqChar("aabb") == -1
