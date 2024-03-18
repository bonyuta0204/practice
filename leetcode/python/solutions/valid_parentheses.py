from typing import Optional
from data_structures.linked_list import ListNode


class Solution:
    def isValid(self, s):

        stack = []

        pairs = {"(": ")", "[": "]", "{": "}"}

        for c in s:
            if len(stack) > 0:
                last = stack[-1]

                if last in pairs and pairs[last] == c:
                    stack.pop()
                else:
                    stack.append(c)

            else:
                stack.append(c)
        return len(stack) == 0


if __name__ == "__main__":
    print(Solution().isValid("()"))
    print(Solution().isValid("([)"))
    print(Solution().isValid("([])"))
