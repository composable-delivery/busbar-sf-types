import { Card, Button, Tabs, TabList, Tab, TabPanel } from 'reablocks';
import TypeNetwork from './TypeNetwork';
import { useMemo } from 'react';

export default function TypeDetailView({ typeData, graphData, onBack, onTypeSelect }) {
  const relationshipBadgeClass = (rel) => {
    const baseClass = 'inline-flex items-center px-3 py-1 rounded-full text-xs font-medium';
    switch (rel) {
      case 'Contains':
        return `${baseClass} bg-blue-500/20 text-blue-300`;
      case 'Extends':
        return `${baseClass} bg-purple-500/20 text-purple-300`;
      case 'Generic':
        return `${baseClass} bg-orange-500/20 text-orange-300`;
      default:
        return `${baseClass} bg-gray-500/20 text-gray-300`;
    }
  };

  const localGraph = useMemo(() => {
    if (!typeData || !graphData) return { nodes: [], edges: [] };
    
    const nodes = new Set([typeData.name]);
    const edges = [];
    
    // Add direct dependencies
    typeData.dependencies.forEach(dep => {
      nodes.add(dep.target);
      edges.push({
        source: typeData.name,
        target: dep.target,
        relationship: dep.relationship
      });
    });
    
    // Add direct dependents
    typeData.dependents.forEach(dep => {
      nodes.add(dep.source);
      edges.push({
        source: dep.source,
        target: typeData.name,
        relationship: dep.relationship
      });
    });
    
    return {
      nodes: Array.from(nodes),
      edges: edges
    };
  }, [typeData, graphData]);

  if (!typeData) {
    return (
      <Card className="bg-surface border border-surface-light p-12 text-center">
        <div className="text-gray-400">Type not found</div>
      </Card>
    );
  }

  return (
    <div className="space-y-6 animate-fade-in">
      {/* Breadcrumb Navigation */}
      <div className="flex items-center gap-2 text-sm">
        <Button variant="text" onClick={onBack} className="text-primary-400 hover:text-primary-300">
          ← Back to Overview
        </Button>
      </div>

      {/* Type Header */}
      <Card className="bg-surface border border-surface-light p-8">
        <div className="flex items-start justify-between flex-wrap gap-4">
          <div>
            <h1 className="text-4xl font-bold text-white mb-3">{typeData.name}</h1>
            <div className="flex items-center gap-3">
              <span className="inline-flex items-center px-3 py-1 rounded-lg text-sm font-medium bg-primary-500/20 text-primary-300">
                {typeData.category?.name || 'uncategorized'}
              </span>
              <span className="text-gray-400">
                {typeData.connectionCount} total connections
              </span>
            </div>
          </div>
          
          <div className="grid grid-cols-2 gap-4 text-center">
            <div className="bg-surface-dark p-4 rounded-lg">
              <div className="text-3xl font-bold text-primary-400">{typeData.dependencies.length}</div>
              <div className="text-xs text-gray-400 uppercase mt-1">Dependencies</div>
            </div>
            <div className="bg-surface-dark p-4 rounded-lg">
              <div className="text-3xl font-bold text-green-400">{typeData.dependents.length}</div>
              <div className="text-xs text-gray-400 uppercase mt-1">Dependents</div>
            </div>
          </div>
        </div>
      </Card>

      {/* Graph Visualization */}
      <TypeNetwork 
        graphData={localGraph} 
        selectedType={typeData.name}
        onTypeSelect={onTypeSelect}
      />

      {/* Detailed Relationships */}
      <Card className="bg-surface border border-surface-light p-6">
        <Tabs>
          <TabList>
            <Tab>Dependencies ({typeData.dependencies.length})</Tab>
            <Tab>Dependents ({typeData.dependents.length})</Tab>
          </TabList>

          <TabPanel>
            <div className="mt-4">
              {typeData.dependencies.length === 0 ? (
                <div className="text-center py-8 text-gray-400">
                  This type has no dependencies
                </div>
              ) : (
                <div className="space-y-2">
                  {typeData.dependencies.map((dep, index) => (
                    <div
                      key={index}
                      className="flex items-center justify-between p-3 bg-surface-dark hover:bg-surface-light/30 rounded-lg transition-colors cursor-pointer"
                      onClick={() => onTypeSelect(dep.target)}
                    >
                      <span className="font-medium text-white">{dep.target}</span>
                      <span className={relationshipBadgeClass(dep.relationship)}>
                        {dep.relationship}
                      </span>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </TabPanel>

          <TabPanel>
            <div className="mt-4">
              {typeData.dependents.length === 0 ? (
                <div className="text-center py-8 text-gray-400">
                  No types depend on this type
                </div>
              ) : (
                <div className="space-y-2">
                  {typeData.dependents.map((dep, index) => (
                    <div
                      key={index}
                      className="flex items-center justify-between p-3 bg-surface-dark hover:bg-surface-light/30 rounded-lg transition-colors cursor-pointer"
                      onClick={() => onTypeSelect(dep.source)}
                    >
                      <span className="font-medium text-white">{dep.source}</span>
                      <span className={relationshipBadgeClass(dep.relationship)}>
                        {dep.relationship}
                      </span>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </TabPanel>
        </Tabs>
      </Card>
    </div>
  );
}
