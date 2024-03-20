
class solution(object):
    def topkfrequent(self, nums, k):
        """
        :type nums: list[int]
        :type k: int
        :rtype: list[int]
        """

        counts = {}
        max_freq = 0

        for n in nums:
            if n in counts:
                counts[n] += 1
            else:
                counts[n] = 1
            if max_freq < counts[n]:
                max_freq = counts[n]
        freq_list = [[] for _ in range(0, max_freq)]

        for val, freq in counts.items():
            # Note that freq is One indexed
            freq_list[freq - 1].append(val)

        results = []

        for group in reversed(freq_list):
            for val in group:
                results.append(val)

            if len(results) == k:
                break

        return results


if __name__ == "__main__":

    assert solution().topkfrequent([1, 1, 1, 2, 2, 3, 3, 4, 5], 3) == [1, 2, 3]
