//! Type dependency graph for analyzing relationships between Salesforce metadata types.
//!
//! This module builds a directed graph of type dependencies from TypeScript AST,
//! enabling automated categorization and detection of shared/common types.

use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of relationship between types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationshipType {
    /// Type A contains a field of Type B
    Contains,
    /// Type A extends Type B
    Extends,
    /// Type A is a generic instantiation of Type B
    Generic,
}

/// A directed graph of type dependencies
#[derive(Debug)]
pub struct TypeGraph {
    /// The underlying graph structure
    graph: DiGraph<String, RelationshipType>,
    /// Map from type name to node index for fast lookup
    node_indices: HashMap<String, NodeIndex>,
}

impl TypeGraph {
    /// Create a new empty type graph
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_indices: HashMap::new(),
        }
    }

    /// Add a node for a type (idempotent - returns existing node if already present)
    pub fn add_node(&mut self, type_name: &str) -> NodeIndex {
        if let Some(&idx) = self.node_indices.get(type_name) {
            return idx;
        }

        let idx = self.graph.add_node(type_name.to_string());
        self.node_indices.insert(type_name.to_string(), idx);
        idx
    }

    /// Add a dependency edge from one type to another
    ///
    /// # Arguments
    /// * `from` - The type that depends on another type
    /// * `to` - The type being depended upon
    /// * `rel_type` - The type of relationship
    pub fn add_dependency(
        &mut self,
        from: &str,
        to: &str,
        rel_type: RelationshipType,
    ) {
        let from_idx = self.add_node(from);
        let to_idx = self.add_node(to);
        self.graph.add_edge(from_idx, to_idx, rel_type);
    }

    /// Get all types that a given type depends on (outgoing edges)
    pub fn get_dependencies(&self, type_name: &str) -> Vec<String> {
        if let Some(&node_idx) = self.node_indices.get(type_name) {
            self.graph
                .neighbors(node_idx)
                .map(|idx| self.graph[idx].clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get all types that depend on a given type (incoming edges)
    pub fn get_dependents(&self, type_name: &str) -> Vec<String> {
        if let Some(&node_idx) = self.node_indices.get(type_name) {
            self.graph
                .neighbors_directed(node_idx, petgraph::Direction::Incoming)
                .map(|idx| self.graph[idx].clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get all type names in the graph
    pub fn get_all_types(&self) -> Vec<String> {
        self.node_indices.keys().cloned().collect()
    }

    /// Get types with no incoming edges (top-level types)
    pub fn get_root_types(&self) -> Vec<String> {
        self.graph
            .node_indices()
            .filter(|&idx| {
                self.graph
                    .neighbors_directed(idx, petgraph::Direction::Incoming)
                    .next()
                    .is_none()
            })
            .map(|idx| self.graph[idx].clone())
            .collect()
    }

    /// Get the in-degree (number of incoming edges) for a type
    pub fn get_in_degree(&self, type_name: &str) -> usize {
        if let Some(&node_idx) = self.node_indices.get(type_name) {
            self.graph
                .neighbors_directed(node_idx, petgraph::Direction::Incoming)
                .count()
        } else {
            0
        }
    }

    /// Perform a breadth-first traversal from a given type, collecting reachable types
    pub fn traverse_from(&self, start_type: &str) -> Vec<String> {
        use petgraph::visit::Bfs;

        if let Some(&start_idx) = self.node_indices.get(start_type) {
            let mut bfs = Bfs::new(&self.graph, start_idx);
            let mut reachable = Vec::new();

            while let Some(node_idx) = bfs.next(&self.graph) {
                reachable.push(self.graph[node_idx].clone());
            }

            reachable
        } else {
            Vec::new()
        }
    }

    /// Export the graph to a JSON-serializable format
    pub fn export(&self) -> GraphExport {
        let nodes: Vec<String> = self.graph.node_indices()
            .map(|idx| self.graph[idx].clone())
            .collect();

        let edges: Vec<GraphEdge> = self.graph.edge_indices()
            .filter_map(|edge_idx| {
                let (source, target) = self.graph.edge_endpoints(edge_idx)?;
                let source_name = self.graph[source].clone();
                let target_name = self.graph[target].clone();
                let rel_type = *self.graph.edge_weight(edge_idx)?;
                Some(GraphEdge {
                    source: source_name,
                    target: target_name,
                    relationship: rel_type,
                })
            })
            .collect();

        GraphExport {
            nodes,
            edges,
            categories: HashMap::new(), // Will be populated by categorization
        }
    }
}

impl Default for TypeGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Serializable export format for the type graph
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphExport {
    /// List of all type names
    pub nodes: Vec<String>,
    /// List of dependency edges
    pub edges: Vec<GraphEdge>,
    /// Category assignments for each type
    pub categories: HashMap<String, String>,
}

impl GraphExport {
    /// Export the graph in DOT format for visualization with Graphviz
    pub fn to_dot(&self) -> String {
        let mut output = String::from("digraph TypeDependencies {\n");
        output.push_str("  rankdir=LR;\n");
        output.push_str("  node [shape=box];\n\n");
        
        // Group nodes by category
        let mut categories_map: HashMap<&str, Vec<&str>> = HashMap::new();
        for (node, category) in &self.categories {
            categories_map
                .entry(category.as_str())
                .or_default()
                .push(node.as_str());
        }
        
        // Output nodes grouped by category in subgraphs
        for (i, (category, nodes)) in categories_map.iter().enumerate() {
            output.push_str(&format!("  subgraph cluster_{} {{\n", i));
            output.push_str(&format!("    label=\"{}\";\n", category));
            
            for node in nodes {
                let safe_node = node.replace('"', "\\\"");
                output.push_str(&format!("    \"{}\";\n", safe_node));
            }
            
            output.push_str("  }\n\n");
        }
        
        // Output edges
        for edge in &self.edges {
            let safe_source = edge.source.replace('"', "\\\"");
            let safe_target = edge.target.replace('"', "\\\"");
            let label = match edge.relationship {
                RelationshipType::Contains => "contains",
                RelationshipType::Extends => "extends",
                RelationshipType::Generic => "generic",
            };
            output.push_str(&format!(
                "  \"{}\" -> \"{}\" [label=\"{}\"];\n",
                safe_source, safe_target, label
            ));
        }
        
        output.push_str("}\n");
        output
    }
}

/// A single edge in the exported graph
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphEdge {
    /// Source type name
    pub source: String,
    /// Target type name
    pub target: String,
    /// Type of relationship
    pub relationship: RelationshipType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node_idempotent() {
        let mut graph = TypeGraph::new();
        let idx1 = graph.add_node("CustomObject");
        let idx2 = graph.add_node("CustomObject");
        assert_eq!(idx1, idx2);
    }

    #[test]
    fn test_add_dependency() {
        let mut graph = TypeGraph::new();
        graph.add_dependency("CustomObject", "CustomField", RelationshipType::Contains);
        
        let deps = graph.get_dependencies("CustomObject");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0], "CustomField");
    }

    #[test]
    fn test_get_dependents() {
        let mut graph = TypeGraph::new();
        graph.add_dependency("CustomObject", "CustomField", RelationshipType::Contains);
        
        let dependents = graph.get_dependents("CustomField");
        assert_eq!(dependents.len(), 1);
        assert_eq!(dependents[0], "CustomObject");
    }

    #[test]
    fn test_get_root_types() {
        let mut graph = TypeGraph::new();
        graph.add_node("CustomObject");
        graph.add_dependency("CustomObject", "CustomField", RelationshipType::Contains);
        
        let roots = graph.get_root_types();
        assert!(roots.contains(&"CustomObject".to_string()));
        assert!(!roots.contains(&"CustomField".to_string()));
    }

    #[test]
    fn test_traverse_from() {
        let mut graph = TypeGraph::new();
        graph.add_dependency("A", "B", RelationshipType::Contains);
        graph.add_dependency("B", "C", RelationshipType::Contains);
        
        let reachable = graph.traverse_from("A");
        assert_eq!(reachable.len(), 3);
        assert!(reachable.contains(&"A".to_string()));
        assert!(reachable.contains(&"B".to_string()));
        assert!(reachable.contains(&"C".to_string()));
    }
}
