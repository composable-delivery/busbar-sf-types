# Type Dependency Graph

This directory contains the type dependency graph generated from Salesforce metadata TypeScript definitions.

## Files

- **type-graph.json** - Complete graph in JSON format with nodes, edges, and category assignments
- **type-graph.dot** - GraphViz DOT format for visualization

## Structure

### JSON Format

The JSON export contains:

```json
{
  "nodes": ["TypeName1", "TypeName2", ...],
  "edges": [
    {
      "source": "TypeA",
      "target": "TypeB",
      "relationship": "Contains"
    }
  ],
  "categories": {
    "TypeName": "category_name"
  }
}
```

### Relationship Types

- **Contains**: Type A has a field of Type B
- **Extends**: Type A extends Type B
- **Generic**: Type A is a generic instantiation of Type B

## Usage

### Visualizing the Graph

To generate a PNG visualization from the DOT file:

```bash
# Install Graphviz if not already installed
# macOS: brew install graphviz
# Ubuntu: apt-get install graphviz
# Windows: Download from https://graphviz.org/download/

# Generate PNG
dot -Tpng assets/type-graph.dot -o type-graph.png

# Or SVG for better quality
dot -Tsvg assets/type-graph.dot -o type-graph.svg
```

### Analyzing the Graph Programmatically

```rust
use serde_json;
use std::fs;

#[derive(Deserialize)]
struct GraphExport {
    nodes: Vec<String>,
    edges: Vec<GraphEdge>,
    categories: HashMap<String, String>,
}

#[derive(Deserialize)]
struct GraphEdge {
    source: String,
    target: String,
    relationship: String,
}

fn main() {
    let json = fs::read_to_string("assets/type-graph.json")?;
    let graph: GraphExport = serde_json::from_str(&json)?;
    
    // Find all dependencies of CustomObject
    let deps: Vec<_> = graph.edges.iter()
        .filter(|e| e.source == "CustomObject")
        .collect();
    
    println!("CustomObject depends on {} types", deps.len());
}
```

## Statistics

Current graph (as of latest generation):

- **Nodes**: 2682 types
- **Edges**: 4906 dependencies
- **Categories**: 81 categories
- **Shared Types**: 267 types used by multiple categories (automatically assigned to "common")
- **Root Types**: 2 types with no incoming dependencies

## Category Assignment

The graph-based categorization algorithm:

1. **Seeds** from explicit types defined in each category
2. **Propagates** categories through dependency traversal
3. **Detects shared types** used by multiple categories
4. **Assigns** shared types to the "common" module

This enables automatic categorization of new types based on their relationships rather than manual pattern matching.

## Regeneration

The graph is regenerated automatically when running:

```bash
cargo run -p sf-typegen --bin generate_from_typescript
```

The graph will be saved to `assets/type-graph.json` and `assets/type-graph.dot`.
