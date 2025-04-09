/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

/*
我来帮你实现广度优先搜索（BFS）算法。BFS是一种图遍历算法，它从起始节点开始，逐层访问所有相邻节点。

让我编辑文件来实现这个功能：

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
让我详细解释一下BFS的实现：

1. 数据结构的准备：
   ```rust
   let mut visited = vec![false; self.adj.len()];
   let mut queue = VecDeque::new();
   let mut visit_order = Vec::new();
   ```
   - `visited`：布尔数组，记录每个节点是否被访问过
   - `queue`：双端队列，用于存储待访问的节点
   - `visit_order`：向量，记录节点的访问顺序

2. 初始化：
   ```rust
   visited[start] = true;
   queue.push_back(start);
   ```
   - 标记起始节点为已访问
   - 将起始节点加入队列

3. 主循环：
   while let Some(current) = queue.pop_front() {
       visit_order.push(current);
   - 从队列前端取出节点进行处理
   - 将当前节点加入访问顺序列表

4. 邻居处理：
   for &neighbor in &self.adj[current]
   - 遍历当前节点的所有邻居
   - 如果邻居未被访问，标记为已访问并加入队列

BFS的特点：
- 时间复杂度：O(V + E)，其中 V 是顶点数，E 是边数
- 空间复杂度：O(V)，用于存储访问标记和队列
- 保证找到最短路径（在无权图中）
- 使用队列实现先进先出的访问顺序

这个实现应该能够通过所有的测试用例：
1. `test_bfs_all_nodes_visited`：测试所有节点都被访问
2. `test_bfs_different_start`：测试从不同起始点开始
3. `test_bfs_with_cycle`：测试带环图的情况
4. `test_bfs_single_node`：测试单节点图的情况

注意事项：
1. 使用 `VecDeque` 作为队列，提供高效的入队和出队操作
2. 使用 `visited` 数组避免重复访问节点
3. 保持访问顺序的记录，这是题目要求的返回值
4. 处理无向图的情况（`add_edge` 方法已经处理了双向边）

*/
use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        // 创建一个数组来记录节点是否被访问过
        let mut visited = vec![false; self.adj.len()];
        // 创建一个队列来存储待访问的节点
        let mut queue = VecDeque::new();
        // 创建一个向量来存储访问顺序
        let mut visit_order = Vec::new();

        // 标记起始节点为已访问，并将其加入队列
        visited[start] = true;
        queue.push_back(start);

        // 当队列不为空时继续处理
        while let Some(current) = queue.pop_front() {
            // 将当前节点加入访问顺序列表
            visit_order.push(current);

            // 遍历当前节点的所有邻居
            for &neighbor in &self.adj[current] {
                // 如果邻居节点未被访问过
                if !visited[neighbor] {
                    // 标记为已访问并加入队列
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }

        visit_order
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

