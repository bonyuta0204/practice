class Solution(object):
    def subarraySum(self, nums, k):
        """
        :type nums: List[int]
        :type k: int
        :rtype: int
        """

        # prefix sum

        cumulative_sum = 0
        cumulative_sums = {0: 1}
        result = 0

        for n in nums:
            cumulative_sum += n
            if cumulative_sum - k in cumulative_sums:
                result += cumulative_sums[cumulative_sum - k]
            # Update the hash map after checking
            if cumulative_sum in cumulative_sums:
                cumulative_sums[cumulative_sum] += 1
            else:
                cumulative_sums[cumulative_sum] = 1

        return result



if __name__ == "__main__":
    assert Solution().subarraySum([1, 1, 1],  2) == 2
    assert Solution().subarraySum([1, 2, 3],  3) == 2
    assert Solution().subarraySum([1],  0) == 0
