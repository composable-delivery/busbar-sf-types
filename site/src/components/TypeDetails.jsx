import React, { useMemo, useState } from 'react';
import { SimpleTabs, SimpleTab } from './SimpleTabs';

function TypeDetails({ graphData, selectedType, searchQuery, onTypeSelect }) {
  const [activeTab, setActiveTab] = useState('details');

  // Search functionality
  const searchResults = useMemo(() => {
    if (!graphData || !searchQuery || searchQuery.trim().length === 0) {
      return [];
    }
    const query = searchQuery.toLowerCase();
    return graphData.nodes.filter((node) => node.toLowerCase().includes(query));
  }, [graphData, searchQuery]);

  // Type details
  const typeInfo = useMemo(() => {
    if (!graphData || !selectedType) return null;

    const dependencies = graphData.edges.filter((edge) => edge.source === selectedType);
    const dependents = graphData.edges.filter((edge) => edge.target === selectedType);
    const category = graphData.categories[selectedType] || 'unknown';

    // Group dependencies by relationship type
    const depsByRelationship = {
      Contains: dependencies.filter((d) => d.relationship === 'Contains'),
      Extends: dependencies.filter((d) => d.relationship === 'Extends'),
      Generic: dependencies.filter((d) => d.relationship === 'Generic'),
    };

    return {
      category,
      dependencies,
      dependents,
      depsByRelationship,
    };
  }, [graphData, selectedType]);

  if (searchQuery && searchQuery.trim().length > 0) {
    return (
      <div className="type-details">
        <h2>Search Results</h2>
        <p>
          Found {searchResults.length} types matching "<strong>{searchQuery}</strong>"
        </p>
        {searchResults.length > 50 && <p><em>Showing first 50 results</em></p>}
        <div className="dependency-list">
          {searchResults.slice(0, 50).map((type) => (
            <div key={type} className="dependency-item" onClick={() => onTypeSelect(type)}>
              {type}
              <span className="relationship-badge contains">
                {graphData.categories[type] || 'unknown'}
              </span>
            </div>
          ))}
        </div>
      </div>
    );
  }

  if (!selectedType) {
    return (
      <div className="type-details">
        <h2>Welcome to Salesforce Types Explorer</h2>
        <p>Select a type from the search results, graph, or browse by category to explore dependencies.</p>
        <div className="info-box">
          <h3>About</h3>
          <p>This interactive tool visualizes the dependency graph of Salesforce metadata types. The graph shows:</p>
          <ul>
            <li>
              <strong>Contains:</strong> Type A has a field of Type B
            </li>
            <li>
              <strong>Extends:</strong> Type A extends/inherits from Type B
            </li>
            <li>
              <strong>Generic:</strong> Type A is a generic instantiation of Type B
            </li>
          </ul>
        </div>
      </div>
    );
  }

  return (
    <div className="type-details">
      <h2>{selectedType}</h2>
      
      <SimpleTabs value={activeTab} onChange={setActiveTab}>
        <SimpleTab value="details" label="Details">
          <div className="tab-content">
            <div className="info-box">
              <strong>Category:</strong> {typeInfo.category}
              <br />
              <strong>Dependencies:</strong> {typeInfo.dependencies.length}
              <br />
              <strong>Used by:</strong> {typeInfo.dependents.length} types
            </div>

            {Object.entries(typeInfo.depsByRelationship)
              .filter(([_, deps]) => deps.length > 0)
              .map(([rel, deps]) => (
                <div key={rel}>
                  <h3>
                    {rel} Dependencies ({deps.length})
                  </h3>
                  <ul className="dependency-list">
                    {deps.map((dep) => (
                      <li key={dep.target} className="dependency-item" onClick={() => onTypeSelect(dep.target)}>
                        {dep.target}
                        <span className={`relationship-badge ${dep.relationship.toLowerCase()}`}>
                          {dep.relationship}
                        </span>
                      </li>
                    ))}
                  </ul>
                </div>
              ))}

            {typeInfo.dependents.length > 0 && (
              <div>
                <h3>Used By ({typeInfo.dependents.length})</h3>
                <ul className="dependency-list">
                  {typeInfo.dependents.slice(0, 20).map((dep) => (
                    <li key={dep.source} className="dependency-item" onClick={() => onTypeSelect(dep.source)}>
                      {dep.source}
                      <span className={`relationship-badge ${dep.relationship.toLowerCase()}`}>
                        {dep.relationship}
                      </span>
                    </li>
                  ))}
                  {typeInfo.dependents.length > 20 && (
                    <li style={{ padding: '0.5rem', color: 'var(--text-secondary)' }}>
                      <em>... and {typeInfo.dependents.length - 20} more</em>
                    </li>
                  )}
                </ul>
              </div>
            )}
          </div>
        </SimpleTab>

        <SimpleTab value="schema" label="JSON Schema">
          <div className="tab-content">
            <div className="info-box">
              <h3>JSON Schema</h3>
              <p>
                JSON Schema generation for this type is currently in development. Once available, the schema will be
                displayed here showing the structure, field types, and validation rules for <strong>{selectedType}</strong>.
              </p>
              <p style={{ marginTop: '1rem', fontSize: '0.9rem', color: '#706e6b' }}>
                See the{' '}
                <a href="https://github.com/composable-delivery/busbar-sf-types#json-schemas" target="_blank" rel="noopener noreferrer">
                  README
                </a>{' '}
                for information about enabling JSON Schema support.
              </p>
            </div>
          </div>
        </SimpleTab>
      </SimpleTabs>
    </div>
  );
}

export default TypeDetails;
