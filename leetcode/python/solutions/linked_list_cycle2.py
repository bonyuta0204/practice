from typing import Optional
from data_structures.linked_list import ListNode


class Solution:
    def detectCycle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        # Algorithm:
        # Visit each node and store the node to a set
        # if the node is already in the set, then the list has a cycle

        visited = set()

        while head:
            if head in visited:
                return True
            visited.add(head)
            head = head.next
        return False



if __name__ == "__main__":
    head = ListNode.create_from_head_and_pos([3, 2, 0, -4], 1)
    print(Solution().hasCycle(head))
