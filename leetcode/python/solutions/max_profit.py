class Solution(object):
    def maxProfit(self, prices):
        """
        :type prices: List[int]
        :rtype: int
        """
        if len(prices) == 0:
            return 0
        min_price = prices[0]
        max_profit = 0

        for i, p in enumerate(prices):
            if min_price > p:
                min_price = p
            profit = p - min_price
            if max_profit < profit:
                max_profit = profit

        return max_profit


if __name__ == "__main__":
    assert Solution().maxProfit([7, 1, 5, 3, 6, 4]) == 5
