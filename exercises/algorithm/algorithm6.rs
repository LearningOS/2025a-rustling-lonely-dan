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

    // 递归辅助函数：处理单个节点的DFS遍历
    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        // 标记当前节点为已访问
        visited.insert(v);
        // 将当前节点加入访问顺序
        visit_order.push(v);

        // 遍历当前节点的所有邻接节点
        for &neighbor in &self.adj[v] {
            // 若邻接节点未被访问，则递归遍历
            if !visited.contains(&neighbor) {
                self.dfs_util(neighbor, visited, visit_order);
            }
        }
    }

    // 执行DFS，返回访问顺序
    fn dfs(&self, start: usize) -> Vec<usize> {
        // 处理起始节点超出图范围的情况（增强鲁棒性）
        if start >= self.adj.len() {
            return vec![];
        }

        let mut visited = HashSet::new();
        let mut visit_order = Vec::new();
        // 调用辅助函数开始DFS
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
        graph.add_edge(3, 3); // 自环

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

    #[test]
    fn test_dfs_single_node() {
        let mut graph = Graph::new(1);
        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0]);
    }
}