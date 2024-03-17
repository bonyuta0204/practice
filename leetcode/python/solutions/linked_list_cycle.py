from typing import Optional
from data_structures.linked_list import ListNode


class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        pass


if __name__ == "__main__":
    head = ListNode.create_from_head_and_pos([3, 2, 0, -4], 1)
    print(Solution().hasCycle(head))
