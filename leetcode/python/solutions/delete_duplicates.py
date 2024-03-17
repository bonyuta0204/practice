from typing import Optional
from data_structures.linked_list import ListNode


class Solution:
    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:

        # Algorithm:
        # Visit each node
        # if the next node has same value, update next to next next
        # else, move to next node
        if not head:
            return None

        current = head

        while current.next:
            if current.val == current.next.val:
                current.next = current.next.next

            else:
                current = current.next
        return head


if __name__ == "__main__":
    head = ListNode.create_from_head_and_pos([1, 1, 2, 3], 0)
    print(Solution().deleteDuplicates(head))
