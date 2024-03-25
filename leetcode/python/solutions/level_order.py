from data_structures.tree import TreeNode
from collections import deque


class Solution(object):
    def levelOrder(self, root):
        """
        :type root: TreeNode
        :rtype: List[List[int]]
        """

        if not root:
            return []

        queue = deque([(root, 0)])

        result = []

        while queue:
            node, depth = queue.popleft()

            if len(result) <= depth:
                result.append([])


            result[depth].append(node.val)

            if node.left:
                queue.append((node.left, depth + 1))
            if node.right:
                queue.append((node.right, depth + 1))

        return result


if __name__ == "__main__":
    tree = TreeNode.from_array(
        [3, 9, 20, None, None, 15, 7]
    )

    print(Solution().levelOrder(tree))
