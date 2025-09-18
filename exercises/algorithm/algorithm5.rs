/*
	bfs
	This problem requires you to implement a basic BFS algorithm
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
        
		//TODO
   let n = self.adj.len();
    // 处理空图的边界情况
    if n == 0 {
        return vec![];
    }
    
    // `queue` 用于存储待访问的节点，这是 BFS 的核心数据结构
    let mut queue = VecDeque::new();
    // `visited` 用于标记一个节点是否已经被访问过，防止重复访问和死循环
    let mut visited = vec![false; n];
    // `visit_order` 用于存储最终的访问顺序
    let mut visit_order = vec![];

    // 1. 从起始节点开始
    // 将起始节点标记为已访问
    visited[start] = true;
    // 将起始节点加入队列
    queue.push_back(start);

    // 2. 只要队列不为空，就继续循环
    while let Some(u) = queue.pop_front() {
        // a. 从队列中取出一个节点 `u`
        // b. 将该节点加入到我们的结果列表中
        visit_order.push(u);

        // c. 遍历该节点的所有邻居 `v`
        for &v in &self.adj[u] {
            // d. 如果邻居 `v` 还没有被访问过
            if !visited[v] {
                // 将其标记为已访问
                visited[v] = true;
                // 并将其加入队列，等待后续访问
                queue.push_back(v);
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

