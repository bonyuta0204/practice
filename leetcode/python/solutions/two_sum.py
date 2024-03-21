class Solution(object):
    def twoSum(self, nums, target):
        """
        :type nums: List[int]
        :type target: int
        :rtype: List[int]
        """

        hash_map = {}

        for i in range(len(nums)):
            hash_map[nums[i]] = i

        for i in range(len(nums)):
            key = target - nums[i]
            if key in hash_map:
                if i != hash_map[key]:
                    return [i, hash_map[key]]


if __name__ == "__main__":
    assert Solution().twoSum([1, 2, 3, 4],  7) == [2, 3]
