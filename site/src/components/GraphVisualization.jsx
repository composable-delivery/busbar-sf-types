import React, { useMemo } from 'react';
import { GraphCanvas } from 'reagraph';

function GraphVisualization({ graphData, selectedType, filters, onTypeSelect }) {
  // Convert graph data to reagraph format with multi-level DAG
  const { nodes, edges } = useMemo(() => {
    if (!graphData) return { nodes: [], edges: [] };

    if (!selectedType) {
      // Show a broader overview with multiple types from top categories
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
        .slice(0, 8);

      // For each top category, add a few representative types
      const nodeSet = new Set();
      const edgeSet = [];
      
      topCategories.forEach(([category, stats]) => {
        // Add up to 5 types from each category
        const typesToShow = stats.types.slice(0, 5);
        typesToShow.forEach(type => nodeSet.add(type));
        
        // Find edges between these types
        graphData.edges.forEach(edge => {
          if (typesToShow.includes(edge.source) && typesToShow.includes(edge.target)) {
            edgeSet.push(edge);
          }
        });
      });

      const nodes = Array.from(nodeSet).map((nodeId) => ({
        id: nodeId,
        label: nodeId,
        fill: getCategoryColor(graphData.categories[nodeId] || 'unknown'),
        size: 12,
      }));

      const edges = edgeSet.map((dep, idx) => ({
        id: `${dep.source}-${dep.target}-${idx}`,
        source: dep.source,
        target: dep.target,
        label: dep.relationship,
        fill: getRelationshipColor(dep.relationship),
      }));

      return { nodes, edges };
    }

    // Show multi-level DAG centered on selected type
    const nodeSet = new Set([selectedType]);
    const edgeSet = [];
    
    // Level 1: Direct dependencies (outgoing)
    const level1Deps = graphData.edges.filter((edge) => {
      if (edge.source !== selectedType) return false;
      const rel = edge.relationship.toLowerCase();
      return (
        (rel === 'contains' && filters.contains) ||
        (rel === 'extends' && filters.extends) ||
        (rel === 'generic' && filters.generic)
      );
    });
    
    level1Deps.forEach(edge => {
      nodeSet.add(edge.target);
      edgeSet.push(edge);
    });

    // Level 2: Dependencies of dependencies (go one level deeper)
    const level1Targets = level1Deps.map(e => e.target);
    const level2Deps = graphData.edges.filter((edge) => {
      if (!level1Targets.includes(edge.source)) return false;
      const rel = edge.relationship.toLowerCase();
      return (
        (rel === 'contains' && filters.contains) ||
        (rel === 'extends' && filters.extends) ||
        (rel === 'generic' && filters.generic)
      );
    });
    
    // Limit level 2 to avoid overcrowding
    level2Deps.slice(0, 20).forEach(edge => {
      nodeSet.add(edge.target);
      edgeSet.push(edge);
    });

    // Also show what depends on the selected type (incoming edges)
    const dependents = graphData.edges.filter((edge) => {
      if (edge.target !== selectedType) return false;
      const rel = edge.relationship.toLowerCase();
      return (
        (rel === 'contains' && filters.contains) ||
        (rel === 'extends' && filters.extends) ||
        (rel === 'generic' && filters.generic)
      );
    });
    
    // Limit dependents to avoid overcrowding
    dependents.slice(0, 10).forEach(edge => {
      nodeSet.add(edge.source);
      edgeSet.push(edge);
    });

    const nodes = Array.from(nodeSet).map((nodeId) => {
      let size = 10;
      let fill = getCategoryColor(graphData.categories[nodeId] || 'unknown');
      
      // Make selected type larger and distinct
      if (nodeId === selectedType) {
        size = 25;
        fill = '#0176d3';
      } else if (level1Targets.includes(nodeId)) {
        // Level 1 dependencies slightly larger
        size = 15;
      }
      
      return {
        id: nodeId,
        label: nodeId,
        fill,
        size,
      };
    });

    const edges = edgeSet.map((dep, idx) => ({
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
          ? `Multi-level dependency tree for: ${selectedType}`
          : 'Overview of type relationships - click any type to explore its dependency tree'}
      </p>
      <div style={{ width: '100%', height: '700px', border: '1px solid #ddd', borderRadius: '8px', background: '#fafafa' }}>
        <GraphCanvas
          nodes={nodes}
          edges={edges}
          layoutType="forceDirected2d"
          edgeArrowPosition="end"
          onNodeClick={handleNodeClick}
          draggable
          readonly={false}
          labelType="all"
        />
      </div>
      {selectedType && (
        <div className="info-box" style={{ marginTop: '1rem' }}>
          <strong>Tip:</strong> The graph shows multiple levels of dependencies. The selected type (larger blue node) is at the center, 
          with its direct dependencies and their dependencies radiating outward. Nodes pointing to the selected type depend on it.
        </div>
      )}
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
    settings: '#5c6bc0',
    automation: '#f57c00',
    reports: '#c62828',
    messaging: '#00acc1',
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
