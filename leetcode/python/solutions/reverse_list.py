from data_structures.linked_list import ListNode


class Solution(object):
    def reverseList(self, head):
        """
        :type head: ListNode
        :rtype: ListNode
        """

        stack = []

        current = head

        while current:
            stack.append(current)
            current = current.next

        if len(stack) > 0:
            head = stack.pop()

            current = head

            while True:
                if len(stack) == 0:
                    break
                next = stack.pop()
                current.next = ListNode(next.val, None)
                current = current.next

            return head


if __name__ == "__main__":
    head = ListNode.create_from_head_and_pos([3, 2, 0, -4], 0)
    print(Solution().reverseList(head))
