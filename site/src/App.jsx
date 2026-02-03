import { useState, useEffect } from 'react';
import Header from './components/Header';
import Sidebar from './components/Sidebar';
import GraphVisualization from './components/GraphVisualization';
import TypeDetails from './components/TypeDetails';
import Dashboard from './components/Dashboard';

function App() {
  const [graphData, setGraphData] = useState(null);
  const [selectedType, setSelectedType] = useState(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [viewMode, setViewMode] = useState('dashboard'); // 'dashboard' or 'graph'
  const [filters, setFilters] = useState({
    contains: true,
    extends: true,
    generic: true,
  });
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  // Load graph data
  useEffect(() => {
    fetch('/busbar-sf-types/type-graph.json')
      .then(response => response.json())
      .then(data => {
        setGraphData(data);
        setLoading(false);
      })
      .catch(err => {
        console.error('Error loading graph data:', err);
        setError(err.message);
        setLoading(false);
      });
  }, []);

  const handleTypeSelect = (typeName) => {
    setSelectedType(typeName);
    setSearchQuery('');
    setViewMode('graph');
  };

  const handleFilterChange = (filterName, value) => {
    setFilters(prev => ({ ...prev, [filterName]: value }));
  };

  if (loading) {
    return (
      <div style={{ padding: '2rem', textAlign: 'center' }}>
        <h2>Loading Salesforce Types Explorer...</h2>
      </div>
    );
  }

  if (error) {
    return (
      <div style={{ padding: '2rem' }}>
        <h2>Error Loading Data</h2>
        <p>Could not load the type graph data: {error}</p>
      </div>
    );
  }

  return (
    <div className="app">
      <Header viewMode={viewMode} onViewModeChange={setViewMode} />
      <main className="main-container">
        <div className="layout">
          <Sidebar
            graphData={graphData}
            searchQuery={searchQuery}
            setSearchQuery={setSearchQuery}
            filters={filters}
            onFilterChange={handleFilterChange}
            onTypeSelect={handleTypeSelect}
            viewMode={viewMode}
          />
          <div className="content">
            {viewMode === 'dashboard' ? (
              <Dashboard
                graphData={graphData}
                onTypeSelect={handleTypeSelect}
              />
            ) : (
              <>
                <GraphVisualization
                  graphData={graphData}
                  selectedType={selectedType}
                  filters={filters}
                  onTypeSelect={handleTypeSelect}
                />
                <TypeDetails
                  graphData={graphData}
                  selectedType={selectedType}
                  searchQuery={searchQuery}
                  onTypeSelect={handleTypeSelect}
                />
              </>
            )}
          </div>
        </div>
      </main>
      <footer className="footer">
        <div className="container">
          <p>
            Generated from{' '}
            <a href="https://github.com/composable-delivery/busbar-sf-types" target="_blank" rel="noopener noreferrer">
              busbar-sf-types
            </a>{' '}
            • Data:{' '}
            <a href="/busbar-sf-types/type-graph.json" target="_blank" rel="noopener noreferrer">
              type-graph.json
            </a>
          </p>
        </div>
      </footer>
    </div>
  );
}

export default App;
