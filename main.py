from typing import Optional

def main():
    solution = Solution
    node = Node(1, [2,4])
    solution.cloneGraph(node)
    
class Node(object):
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []

class Solution(object):
    def cloneGraph(self, node: Optional['Node']) -> Optional['Node']:
        if node is None:
            return None
        memo = { node.val : node.neighbors}
        graph = {node : Node(node.val)}
        self.dfs(node, memo, graph)
        
        print(f"memo:\n{memo}")
        print(f"graph:\n{graph}")
        for _, newNode in graph.items():
            newNode.neighbors = list(map(lambda old: graph[old], memo[newNode.val]))

        return graph[node]

    def dfs(self, node, memo, graph):
        for neighbor in node.neighbors:
            if neighbor not in graph:
                graph[neighbor] = Node(neighbor.val)
            if neighbor.val not in memo:
                memo[neighbor.val] =  neighbor.neighbors
                self.dfs(neighbor, memo, graph)



if __name__ == "__main__":
    main()