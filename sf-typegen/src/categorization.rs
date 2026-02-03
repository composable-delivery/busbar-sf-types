//! Graph-based type categorization for automated module organization.
//!
//! This module implements automatic categorization of types based on their
//! dependency relationships in the type graph.

use crate::categories::CATEGORIES;
use crate::graph::TypeGraph;
use std::collections::{HashMap, HashSet};

/// Result of graph-based categorization
#[derive(Debug, Clone)]
pub struct CategoryAssignment {
    /// Map from type name to category name
    pub assignments: HashMap<String, String>,
    /// Types that are shared by multiple categories (candidates for "common")
    pub shared_types: HashSet<String>,
    /// Types with no incoming edges (potential new root types)
    pub root_types: Vec<String>,
}

impl CategoryAssignment {
    /// Create a new empty category assignment
    pub fn new() -> Self {
        Self {
            assignments: HashMap::new(),
            shared_types: HashSet::new(),
            root_types: Vec::new(),
        }
    }

    /// Get the category for a given type
    pub fn get_category(&self, type_name: &str) -> Option<&str> {
        self.assignments.get(type_name).map(|s| s.as_str())
    }

    /// Check if a type is shared across multiple categories
    pub fn is_shared(&self, type_name: &str) -> bool {
        self.shared_types.contains(type_name)
    }
}

impl Default for CategoryAssignment {
    fn default() -> Self {
        Self::new()
    }
}

/// Perform graph-based categorization of types
///
/// # Algorithm
/// 1. Seed roots: Use explicit_types from each category as seeds
/// 2. Propagate: For each seed, traverse the graph to find reachable types
/// 3. Detect shared: Types reachable from multiple categories are marked as shared
/// 4. Assign categories: Non-shared types get their single category, shared types go to "common"
pub fn categorize_by_graph(graph: &TypeGraph) -> CategoryAssignment {
    let mut assignment = CategoryAssignment::new();

    // Track which categories can reach each type
    let mut type_to_categories: HashMap<String, HashSet<&'static str>> = HashMap::new();

    // Phase 1: Seed roots from explicit_types in categories
    for category in CATEGORIES {
        for &explicit_type in category.explicit_types {
            // Mark this type as belonging to this category
            type_to_categories
                .entry(explicit_type.to_string())
                .or_default()
                .insert(category.name);

            // Traverse from this seed to find reachable types
            let reachable = graph.traverse_from(explicit_type);

            for reachable_type in reachable {
                // Skip the seed itself (already marked)
                if reachable_type == explicit_type {
                    continue;
                }

                // Mark this type as reachable from this category
                type_to_categories
                    .entry(reachable_type)
                    .or_default()
                    .insert(category.name);
            }
        }
    }

    // Phase 2: Detect shared types and assign categories
    for (type_name, categories) in &type_to_categories {
        if categories.len() > 1 {
            // Type is shared across multiple categories
            assignment.shared_types.insert(type_name.clone());
            assignment
                .assignments
                .insert(type_name.clone(), "common".to_string());
        } else if let Some(&category_name) = categories.iter().next() {
            // Type belongs to exactly one category
            assignment
                .assignments
                .insert(type_name.clone(), category_name.to_string());
        }
    }

    // Phase 3: Identify root types (potential uncategorized top-level types)
    assignment.root_types = graph
        .get_root_types()
        .into_iter()
        .filter(|type_name| {
            // Only include types that aren't already categorized
            !assignment.assignments.contains_key(type_name)
        })
        .collect();

    assignment
}

/// Get statistics about category assignments
pub fn get_category_statistics(assignment: &CategoryAssignment) -> HashMap<String, usize> {
    let mut stats: HashMap<String, usize> = HashMap::new();

    for category in assignment.assignments.values() {
        *stats.entry(category.clone()).or_insert(0) += 1;
    }

    stats
}

/// Print a report of category assignments
pub fn print_category_report(assignment: &CategoryAssignment, graph: &TypeGraph) {
    println!("\n📊 Graph-Based Categorization Report:");
    println!("   Total types analyzed: {}", graph.get_all_types().len());
    println!("   Types categorized: {}", assignment.assignments.len());
    println!(
        "   Shared types (-> common): {}",
        assignment.shared_types.len()
    );
    println!(
        "   Uncategorized root types: {}",
        assignment.root_types.len()
    );

    // Show statistics by category
    let stats = get_category_statistics(assignment);
    if !stats.is_empty() {
        println!("\n   By category (graph-based):");
        let mut sorted_stats: Vec<_> = stats.iter().collect();
        sorted_stats.sort_by(|a, b| b.1.cmp(a.1));

        for (category, count) in sorted_stats.iter().take(20) {
            println!("   - {}: {} types", category, count);
        }

        if sorted_stats.len() > 20 {
            println!("   ... and {} more categories", sorted_stats.len() - 20);
        }
    }

    // Show some example shared types
    if !assignment.shared_types.is_empty() {
        println!("\n   Example shared types:");
        for (i, type_name) in assignment.shared_types.iter().take(10).enumerate() {
            println!("   {}. {}", i + 1, type_name);
        }
        if assignment.shared_types.len() > 10 {
            println!("   ... and {} more", assignment.shared_types.len() - 10);
        }
    }

    // Show some uncategorized root types
    if !assignment.root_types.is_empty() {
        println!("\n   ⚠️  Uncategorized root types (need review):");
        for (i, type_name) in assignment.root_types.iter().take(10).enumerate() {
            println!("   {}. {}", i + 1, type_name);
        }
        if assignment.root_types.len() > 10 {
            println!("   ... and {} more", assignment.root_types.len() - 10);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{RelationshipType, TypeGraph};

    #[test]
    fn test_categorize_simple() {
        let mut graph = TypeGraph::new();

        // Create a simple graph: CustomObject -> CustomField
        graph.add_dependency("CustomObject", "CustomField", RelationshipType::Contains);

        let assignment = categorize_by_graph(&graph);

        // Both should be categorized under "objects" since CustomObject is an explicit type
        assert!(assignment.assignments.contains_key("CustomObject"));
        assert!(assignment.assignments.contains_key("CustomField"));
    }

    #[test]
    fn test_shared_types() {
        let mut graph = TypeGraph::new();

        // Create a diamond: A -> C, B -> C
        // If A and B are in different categories, C should be shared
        graph.add_dependency("CustomObject", "SharedType", RelationshipType::Contains);
        graph.add_dependency("Layout", "SharedType", RelationshipType::Contains);

        let assignment = categorize_by_graph(&graph);

        // SharedType should be marked as shared
        assert!(assignment.is_shared("SharedType"));
        assert_eq!(assignment.get_category("SharedType"), Some("common"));
    }
}
