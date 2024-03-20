from heapq import heapify, heappushpop,  heappop, heappush


class KthLargest(object):

    def __init__(self, k, nums):
        """
        :type k: int
        :type nums: List[int]
        """

        self.k = k
        self.pq = nums[: min(k, len(nums))]
        heapify(self.pq)

        for i in range(k, len(nums)):
            heappushpop(self.pq, nums[i])

    def add(self, val):
        """
        :type val: int
        :rtype: int
        """
        heappush(self.pq, val)
        if (len(self.pq)) > self.k:
            heappop(self.pq)
        return self.pq[0]


if __name__ == "__main__":
    obj = KthLargest(3, [1, 2, 3, 4])

    assert obj.add(1) == 2
    assert obj.add(2) == 2
    assert obj.add(5) == 3
