/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/

/*
我来帮你实现深度优先搜索（DFS）算法。
DFS是一种图遍历算法，它沿着一条路径尽可能深地搜索，直到无法继续为止，然后回溯并尝试其他路径。
让我编辑文件来实现这个功能：
    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
让我详细解释一下DFS的实现：

1. `dfs_util` 方法的参数：
   - `v`：当前要访问的节点
   - `visited`：记录已访问节点的集合
   - `visit_order`：记录访问顺序的向量

2. 访问当前节点：
   ```rust
   visited.insert(v);
   visit_order.push(v);
   ```
   - 将当前节点标记为已访问
   - 将当前节点加入访问顺序列表

3. 递归访问邻居：
   ```rust
   for &neighbor in &self.adj[v] {
       if !visited.contains(&neighbor) {
           self.dfs_util(neighbor, visited, visit_order);
       }
   }
   ```
   - 遍历当前节点的所有邻居
   - 如果邻居未被访问，递归访问该邻居

DFS的特点：
- 时间复杂度：O(V + E)，其中 V 是顶点数，E 是边数
- 空间复杂度：O(V)，用于存储访问标记和递归栈
- 使用递归实现，代码简洁
- 适合寻找路径、检测环等场景

这个实现应该能够通过所有的测试用例：
1. `test_dfs_simple`：测试简单图的情况
2. `test_dfs_with_cycle`：测试带环图的情况
3. `test_dfs_disconnected_graph`：测试不连通图的情况

注意事项：
1. 使用 `HashSet` 来存储已访问节点，提供高效的查找操作
2. 使用递归实现，需要注意栈溢出的风险（对于非常大的图）
3. 保持访问顺序的记录，这是题目要求的返回值
4. 处理无向图的情况（`add_edge` 方法已经处理了双向边）

与BFS的主要区别：
1. DFS使用栈（递归调用栈）而不是队列
2. DFS优先深入而不是广度优先
3. DFS的访问顺序通常与BFS不同
4. DFS更适合寻找路径，BFS更适合寻找最短路径

*/
use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src); 
    }

    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        // 标记当前节点为已访问
        visited.insert(v);
        // 将当前节点加入访问顺序列表
        visit_order.push(v);

        // 遍历当前节点的所有邻居
        for &neighbor in &self.adj[v] {
            // 如果邻居节点未被访问过
            if !visited.contains(&neighbor) {
                // 递归访问邻居节点
                self.dfs_util(neighbor, visited, visit_order);
            }
        }
    }

    // Perform a depth-first search on the graph, return the order of visited nodes
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut visit_order = Vec::new(); 
        self.dfs_util(start, &mut visited, &mut visit_order);
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]); 
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]); 
    }
}

