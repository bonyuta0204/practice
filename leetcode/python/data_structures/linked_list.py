# Definition for singly-linked list.
from typing import List


class ListNode:
    def __init__(self, x, next=None):
        self.val = x
        self.next = None

    @classmethod
    def create_from_head_and_pos(cls, head: List[int], pos: int):
        if not head:
            return None
        nodes = [ListNode(x) for x in head]
        for i in range(len(nodes) - 1):
            nodes[i].next = nodes[i + 1]
        if pos > 0:
            nodes[-1].next = nodes[pos]
        return nodes[0]


if __name__ == "__main__":
    head = [3, 2, 0, -4]
    pos = 1
    head = ListNode.create_from_head_and_pos(head, pos)
    print(head)
