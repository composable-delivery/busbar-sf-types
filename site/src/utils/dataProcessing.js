// Data processing utilities for Salesforce types

export function parseDotClusters(dotContent) {
  const clusters = [];
  const clusterRegex = /subgraph\s+cluster_(\d+)\s+{\s+label="([^"]+)";([^}]+)}/g;
  
  let match;
  while ((match = clusterRegex.exec(dotContent)) !== null) {
    const [, id, label, content] = match;
    const types = content.match(/"([^"]+)"/g)?.map(t => t.replace(/"/g, '')) || [];
    clusters.push({
      id: `cluster_${id}`,
      name: label,
      types: types
    });
  }
  
  return clusters;
}

export function buildTypeMetadata(graphData) {
  const metadata = new Map();
  
  // Initialize all nodes
  graphData.nodes.forEach(node => {
    metadata.set(node, {
      name: node,
      dependencies: [],
      dependents: [],
      category: null,
      connectionCount: 0,
    });
  });
  
  // Process edges
  graphData.edges.forEach(edge => {
    const sourceData = metadata.get(edge.source);
    const targetData = metadata.get(edge.target);
    
    if (sourceData) {
      sourceData.dependencies.push({
        target: edge.target,
        relationship: edge.relationship
      });
      sourceData.connectionCount++;
    }
    
    if (targetData) {
      targetData.dependents.push({
        source: edge.source,
        relationship: edge.relationship
      });
      targetData.connectionCount++;
    }
  });
  
  return metadata;
}

export function assignCategories(metadata, clusters) {
  const categoryMap = new Map();
  
  clusters.forEach((cluster, index) => {
    cluster.types.forEach(typeName => {
      categoryMap.set(typeName, {
        name: cluster.name,
        id: cluster.id,
        colorIndex: (index % 8) + 1
      });
    });
  });
  
  // Assign categories to metadata
  metadata.forEach((data, typeName) => {
    const category = categoryMap.get(typeName);
    if (category) {
      data.category = category;
    } else {
      data.category = {
        name: 'uncategorized',
        id: 'cluster_uncategorized',
        colorIndex: 0
      };
    }
  });
  
  return metadata;
}

export function buildTreemapData(metadata, selectedCategory = null) {
  const categoryGroups = new Map();
  
  metadata.forEach((data, typeName) => {
    const categoryName = data.category?.name || 'uncategorized';
    
    if (selectedCategory && categoryName !== selectedCategory) {
      return;
    }
    
    if (!categoryGroups.has(categoryName)) {
      categoryGroups.set(categoryName, {
        key: categoryName,
        data: categoryName,
        children: [],
        colorIndex: data.category?.colorIndex || 0
      });
    }
    
    categoryGroups.get(categoryName).children.push({
      key: typeName,
      data: typeName,
      value: Math.max(1, data.connectionCount),
      metadata: data
    });
  });
  
  return Array.from(categoryGroups.values());
}

export function getCategoryStats(metadata) {
  const stats = new Map();
  
  metadata.forEach((data) => {
    const categoryName = data.category?.name || 'uncategorized';
    
    if (!stats.has(categoryName)) {
      stats.set(categoryName, {
        name: categoryName,
        count: 0,
        totalConnections: 0,
        colorIndex: data.category?.colorIndex || 0
      });
    }
    
    const stat = stats.get(categoryName);
    stat.count++;
    stat.totalConnections += data.connectionCount;
  });
  
  return Array.from(stats.values()).sort((a, b) => b.totalConnections - a.totalConnections);
}

export function getTopConnectedTypes(metadata, limit = 10) {
  return Array.from(metadata.values())
    .sort((a, b) => b.connectionCount - a.connectionCount)
    .slice(0, limit);
}

export function buildLocalGraph(metadata, typeName, depth = 1) {
  const nodes = new Set([typeName]);
  const edges = [];
  const visited = new Set();
  
  function traverse(currentType, currentDepth) {
    if (currentDepth > depth || visited.has(currentType)) return;
    visited.add(currentType);
    
    const typeData = metadata.get(currentType);
    if (!typeData) return;
    
    // Add dependencies
    typeData.dependencies.forEach(dep => {
      nodes.add(dep.target);
      edges.push({
        source: currentType,
        target: dep.target,
        relationship: dep.relationship
      });
      
      if (currentDepth < depth) {
        traverse(dep.target, currentDepth + 1);
      }
    });
    
    // Add dependents
    typeData.dependents.forEach(dep => {
      nodes.add(dep.source);
      edges.push({
        source: dep.source,
        target: currentType,
        relationship: dep.relationship
      });
      
      if (currentDepth < depth) {
        traverse(dep.source, currentDepth + 1);
      }
    });
  }
  
  traverse(typeName, 0);
  
  return {
    nodes: Array.from(nodes),
    edges: edges
  };
}

export function searchTypes(metadata, query) {
  if (!query) return [];
  
  const lowerQuery = query.toLowerCase();
  return Array.from(metadata.values())
    .filter(data => data.name.toLowerCase().includes(lowerQuery))
    .sort((a, b) => {
      // Prioritize exact matches and prefix matches
      const aStarts = a.name.toLowerCase().startsWith(lowerQuery);
      const bStarts = b.name.toLowerCase().startsWith(lowerQuery);
      if (aStarts && !bStarts) return -1;
      if (!aStarts && bStarts) return 1;
      return b.connectionCount - a.connectionCount;
    })
    .slice(0, 50);
}

export function getRelationshipStats(graphData) {
  const stats = {
    Contains: 0,
    Extends: 0,
    Generic: 0
  };
  
  graphData.edges.forEach(edge => {
    if (stats.hasOwnProperty(edge.relationship)) {
      stats[edge.relationship]++;
    }
  });
  
  return stats;
}
