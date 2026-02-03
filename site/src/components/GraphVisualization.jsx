import React, { useMemo } from 'react';
import { GraphCanvas } from 'reagraph';

function GraphVisualization({ graphData, selectedType, filters, onTypeSelect }) {
  // Convert graph data to reagraph format
  const { nodes, edges } = useMemo(() => {
    if (!graphData) return { nodes: [], edges: [] };

    if (!selectedType) {
      // Show overview graph with top categories
      const categoryStats = {};
      Object.entries(graphData.categories).forEach(([type, category]) => {
        if (!categoryStats[category]) {
          categoryStats[category] = { count: 0, types: [] };
        }
        categoryStats[category].count++;
        categoryStats[category].types.push(type);
      });

      const topCategories = Object.entries(categoryStats)
        .sort((a, b) => b[1].count - a[1].count)
        .slice(0, 20);

      const nodes = topCategories.map(([category, stats]) => ({
        id: category,
        label: `${category} (${stats.count})`,
        size: Math.max(15, Math.min(35, stats.count / 10)),
        fill: getCategoryColor(category),
      }));

      // Create some edges between related categories (simplified)
      const edges = [];
      
      return { nodes, edges };
    }

    // Show type-specific graph
    const dependencies = graphData.edges.filter((edge) => {
      if (edge.source !== selectedType) return false;
      const rel = edge.relationship.toLowerCase();
      return (
        (rel === 'contains' && filters.contains) ||
        (rel === 'extends' && filters.extends) ||
        (rel === 'generic' && filters.generic)
      );
    });

    const nodeSet = new Set([selectedType]);
    dependencies.forEach((dep) => nodeSet.add(dep.target));

    const nodes = Array.from(nodeSet).map((nodeId) => ({
      id: nodeId,
      label: nodeId,
      fill: nodeId === selectedType ? '#0176d3' : getCategoryColor(graphData.categories[nodeId] || 'unknown'),
      size: nodeId === selectedType ? 20 : 10,
    }));

    const edges = dependencies.map((dep, idx) => ({
      id: `${dep.source}-${dep.target}-${idx}`,
      source: dep.source,
      target: dep.target,
      label: dep.relationship,
      fill: getRelationshipColor(dep.relationship),
    }));

    return { nodes, edges };
  }, [graphData, selectedType, filters]);

  const handleNodeClick = (node) => {
    if (node && node.id) {
      onTypeSelect(node.id);
    }
  };

  return (
    <div className="graph-container">
      <h3>Dependency Graph</h3>
      <p className="graph-note">
        {selectedType
          ? `Showing dependencies for: ${selectedType}`
          : 'Overview of top categories - select a type to explore'}
      </p>
      <div style={{ width: '100%', height: '600px', border: '1px solid #ddd', borderRadius: '8px', background: '#fafafa' }}>
        <GraphCanvas
          nodes={nodes}
          edges={edges}
          layoutType="forceDirected2d"
          edgeArrowPosition="end"
          onNodeClick={handleNodeClick}
          draggable
          readonly={false}
        />
      </div>
    </div>
  );
}

// Helper functions
function getCategoryColor(category) {
  const colors = {
    common: '#0176d3',
    objects: '#2e844a',
    permissions: '#7b1fa2',
    flows: '#e65100',
    apex: '#d32f2f',
    lwc: '#00838f',
    ai: '#6a1b9a',
    ai_ml: '#6a1b9a',
    datacloud: '#00695c',
    data_cloud: '#00695c',
  };
  return colors[category] || '#546e7a';
}

function getRelationshipColor(relationship) {
  const colors = {
    Contains: '#1565c0',
    Extends: '#7b1fa2',
    Generic: '#e65100',
  };
  return colors[relationship] || '#546e7a';
}

export default GraphVisualization;
