
class Solution(object):
    def intersection(self, nums1, nums2):
        """
        :type nums1: List[int]
        :type nums2: List[int]
        :rtype: List[int]
        """

        set1 = set(nums1)
        set2 = set(nums2)

        result_set = set()

        for el in set1:
            if el in set2:
                result_set.add(el)

        return list(result_set)


if __name__ == "__main__":
    assert Solution().intersection([1, 2, 3], [2, 1]) == [1, 2]
