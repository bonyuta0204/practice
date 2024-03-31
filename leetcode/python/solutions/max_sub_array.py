from typing import List


class Solution(object):
    def maxSubArray(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        DP = [-float('inf')] * len(nums)

        for i in reversed(range(len(nums))):
            if i + 1 >= len(nums):
                DP[i] = nums[i]
            else:
                DP[i] = max(nums[i] + DP[i+1], nums[i])

        return max(DP)


if __name__ == "__main__":
    Solution().maxSubArray([-2, 1, -3, 4, -1, 2, 1, -5, 4])
