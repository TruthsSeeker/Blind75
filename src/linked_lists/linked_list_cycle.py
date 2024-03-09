from typing import Optional, Set

def main():
    solution = Solution()
    solution.hasCycle(None)

if __name__ == "__main__":
    main()

class ListNode:
    def __init__(self, x) -> None:
        self.val = x
        self.next = None

class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        if head is None:
            return False
        
        visited = Set()
        visited.add(head)
        current = head.next

        while current is not None:
            if current in visited:
                return True
            visited.add(current)
            current = current.next

        return False