class Solution(object):
    def rob(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        DP = [0] * len(nums)

        for i in reversed(range(len(nums))):
            if i + 2 >= len(nums):
                DP[i] = nums[i]
            else:
                DP[i] = max(nums[i], nums[i] + max(DP[i+2:]))

        return max(DP)


if __name__ == "__main__":
    assert Solution().rob([2, 7, 9, 3, 1]) == 12
