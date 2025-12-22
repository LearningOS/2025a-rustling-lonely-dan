use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    /// 添加节点到图中
    /// 若节点已存在，返回false；否则返回true
    fn add_node(&mut self, node: &str) -> bool {
        // 尝试插入节点，空的邻接列表作为初始值
        self.adjacency_table_mutable()
            .insert(node.to_string(), Vec::new())
            .is_none()
    }

    /// 添加无向边到图中（自动创建不存在的节点）
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (src, dest, weight) = edge;
        // 确保两个节点都存在（不存在则创建）
        self.add_node(src);
        self.add_node(dest);

        // 向src的邻接表中添加dest和权重
        self.adjacency_table_mutable()
            .get_mut(src)
            .unwrap()
            .push((dest.to_string(), weight));

        // 向dest的邻接表中添加src和权重（无向边的双向性）
        self.adjacency_table_mutable()
            .get_mut(dest)
            .unwrap()
            .push((src.to_string(), weight));
    }

    /// 检查节点是否存在于图中
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    /// 返回图中所有节点的集合
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    /// 返回图中所有边的列表（包含双向边）
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut graph = UndirectedGraph::new();
        // 首次添加节点返回true
        assert_eq!(graph.add_node("a"), true);
        // 重复添加返回false
        assert_eq!(graph.add_node("a"), false);
        // 验证节点存在
        assert_eq!(graph.contains("a"), true);
        assert_eq!(graph.contains("b"), false);
    }

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
        ];

        let edges = graph.edges();
        for edge in expected_edges.iter() {
            assert!(edges.contains(edge), "Missing edge: {:?}", edge);
        }

        // 验证节点数量
        assert_eq!(graph.nodes().len(), 3);
    }

    #[test]
    fn test_edges_empty() {
        let graph = UndirectedGraph::new();
        assert!(graph.edges().is_empty());
    }
}