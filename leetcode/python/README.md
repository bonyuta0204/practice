
# Personal LeetCode Practice Project

This is sub package is for practicing and reinforcing algorithmic problem-solving skills using Python on LeetCode challenges.

## Setup and Usage

This section provides a quick reference of how to set up and run solutions within this project.

### Prerequisites
- Ensure Python and Poetry are installed on your local machine.

### Running Solutions

1. Solutions are organized within the `solutions` package. Navigate to the solution file you wish to run.
2. Use Poetry to run the solution file. Example:

   ```sh
   poetry run python -m solutions.delete_duplicates
   ```

Refer to the `data_structures` package for implementing common data structures like linked lists, trees, and graphs.

### Example: Remove Duplicates from Sorted List

```python
from typing import Optional
from data_structures.linked_list import ListNode  # Import the ListNode data structure for linked list operations.

class Solution:
    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head:
            return None  # Handle an empty list.

        current = head  # Start with the head of the list.

        while current.next:
            if current.val == current.next.val:
                # Remove the duplicate node by skipping it.
                current.next = current.next.next
            else:
                # Move to the next unique node.
                current = current.next
        return head  # Return the modified list without duplicates.
```

