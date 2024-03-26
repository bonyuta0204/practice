from typing import List


class Solution(object):  # 2516 ms, faster than 64.96%
    def lengthOfLIS(self, nums: List[int]) -> int:
        """
        :type nums: List[int]
        :rtype: int
        """
        n = len(nums)
        # LSP starting from each index
        LSP = [1] * n

        for i in reversed(range(n)):
            # Caliculate LSP[i]
            canditates = [1]
            for j in range(i, n):
                if (nums[i] < nums[j]):
                    canditates.append(1 + LSP[j])
            print(f"{i =}")
            print(f"{canditates =}")
            LSP[i] = max(canditates)
        return max(LSP)


if __name__ == "__main__":
    print(f"{Solution().lengthOfLIS([0, 1, 0, 3, 2, 3]) =}")
