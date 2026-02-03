import { GraphCanvas } from 'reagraph';
import { Card } from 'reablocks';
import { useMemo } from 'react';

export default function TypeNetwork({ graphData, selectedType, onTypeSelect }) {
  const graphNodes = useMemo(() => {
    if (!graphData || !graphData.nodes) return [];
    
    return graphData.nodes.map((node) => ({
      id: node,
      label: node,
      fill: node === selectedType ? '#3b82f6' : '#64748b',
      size: node === selectedType ? 20 : 10,
    }));
  }, [graphData, selectedType]);

  const graphEdges = useMemo(() => {
    if (!graphData || !graphData.edges) return [];
    
    return graphData.edges.map((edge, index) => ({
      id: `edge-${index}`,
      source: edge.source,
      target: edge.target,
      label: edge.relationship,
    }));
  }, [graphData]);

  const handleNodeClick = (node) => {
    console.log('[v0] Graph node clicked:', node);
    onTypeSelect(node.id);
  };

  if (!graphData || graphNodes.length === 0) {
    return (
      <Card className="bg-surface border border-surface-light p-12 text-center">
        <div className="text-gray-400">No graph data available</div>
      </Card>
    );
  }

  return (
    <Card className="bg-surface border border-surface-light p-6">
      <div className="mb-4">
        <h3 className="text-xl font-bold text-white">Dependency Network</h3>
        <p className="text-sm text-gray-400 mt-1">
          {selectedType ? `Showing connections for ${selectedType}` : 'Click a node to explore its connections'}
        </p>
      </div>
      
      <div className="h-[600px] rounded-lg overflow-hidden border border-surface-light bg-surface-dark">
        <GraphCanvas
          nodes={graphNodes}
          edges={graphEdges}
          onNodeClick={handleNodeClick}
          layoutType="forceDirected2d"
          edgeInterpolation="curved"
          theme={{
            canvas: {
              background: '#0f172a'
            },
            node: {
              fill: '#64748b',
              activeFill: '#3b82f6',
              label: {
                color: '#ffffff',
                fontSize: 10
              }
            },
            edge: {
              fill: '#475569',
              activeFill: '#3b82f6',
              label: {
                color: '#94a3b8',
                fontSize: 8
              }
            }
          }}
        />
      </div>
    </Card>
  );
}
