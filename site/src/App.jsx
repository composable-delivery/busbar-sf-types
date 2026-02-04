import { useState, useEffect, useMemo } from 'react';
import Hero from './components/Hero';
import StatsBar from './components/StatsBar';
import TypeTreemap from './components/TypeTreemap';
import CategoryFilters from './components/CategoryFilters';
import TypeTable from './components/TypeTable';
import TypeDetailView from './components/TypeDetailView';
import {
  buildTypeMetadata,
  assignCategories,
  buildTreemapData,
  getCategoryStats,
  getRelationshipStats,
  parseDotClusters,
  searchTypes
} from './utils/dataProcessing';

function App() {
  const [graphData, setGraphData] = useState(null);
  const [dotClusters, setDotClusters] = useState([]);
  const [metadata, setMetadata] = useState(null);
  const [selectedType, setSelectedType] = useState(null);
  const [selectedCategory, setSelectedCategory] = useState(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  // Load graph data and dot file
  useEffect(() => {
    const loadData = async () => {
      try {
        // Load graph JSON
        const graphResponse = await fetch('/busbar-sf-types/type-graph.json');
        const graphJson = await graphResponse.json();
        setGraphData(graphJson);

        // Load DOT file for clusters
        const dotResponse = await fetch('/busbar-sf-types/type-graph.dot');
        const dotText = await dotResponse.text();
        const clusters = parseDotClusters(dotText);
        setDotClusters(clusters);

        console.log('[v0] Loaded graph data:', {
          nodes: graphJson.nodes.length,
          edges: graphJson.edges.length,
          clusters: clusters.length
        });

        // Build metadata
        const typeMetadata = buildTypeMetadata(graphJson);
        const enrichedMetadata = assignCategories(typeMetadata, clusters);
        setMetadata(enrichedMetadata);

        setLoading(false);
      } catch (err) {
        console.error('[v0] Error loading data:', err);
        setError(err.message);
        setLoading(false);
      }
    };

    loadData();
  }, []);

  // Calculate stats
  const stats = useMemo(() => {
    if (!graphData || !metadata) {
      return {
        totalTypes: 0,
        totalDependencies: 0,
        totalCategories: 0,
        relationshipStats: {}
      };
    }

    const categoryStats = getCategoryStats(metadata);
    const relationshipStats = getRelationshipStats(graphData);

    return {
      totalTypes: graphData.nodes.length,
      totalDependencies: graphData.edges.length,
      totalCategories: categoryStats.length,
      relationshipStats
    };
  }, [graphData, metadata]);

  // Build treemap data
  const treemapData = useMemo(() => {
    if (!metadata) return [];
    return buildTreemapData(metadata, selectedCategory);
  }, [metadata, selectedCategory]);

  // Get category stats for filters
  const categoryStats = useMemo(() => {
    if (!metadata) return [];
    return getCategoryStats(metadata);
  }, [metadata]);

  // Get all types for table
  const allTypes = useMemo(() => {
    if (!metadata) return [];
    return Array.from(metadata.values());
  }, [metadata]);

  // Handle search
  const handleSearch = () => {
    if (!searchQuery || !metadata) return;
    
    const results = searchTypes(metadata, searchQuery);
    if (results.length > 0) {
      setSelectedType(results[0].name);
    }
  };

  // Handle type selection
  const handleTypeSelect = (typeName) => {
    console.log('[v0] Type selected:', typeName);
    setSelectedType(typeName);
    window.scrollTo({ top: 0, behavior: 'smooth' });
  };

  const handleBackToOverview = () => {
    setSelectedType(null);
    setSearchQuery('');
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-slate-900 flex items-center justify-center">
        <div className="text-center">
          <div className="text-4xl font-bold text-blue-400 mb-4 animate-pulse">
            Loading Salesforce Types Explorer...
          </div>
          <div className="text-gray-400">Processing {stats.totalTypes || '2,682'} types</div>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-screen bg-slate-900 flex items-center justify-center">
        <div className="max-w-2xl bg-slate-800 border border-red-500 p-8 rounded-lg">
          <h2 className="text-2xl font-bold text-red-400 mb-4">Error Loading Data</h2>
          <p className="text-gray-300">Could not load the type graph data: {error}</p>
        </div>
      </div>
    );
  }

  const selectedTypeData = selectedType ? metadata.get(selectedType) : null;

  return (
    <div className="min-h-screen bg-slate-900">
      {/* Hero Section */}
      {!selectedType && (
        <Hero
          searchQuery={searchQuery}
          onSearchChange={setSearchQuery}
          onSearch={handleSearch}
        />
      )}

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-6 py-12 space-y-12">
        {!selectedType ? (
          <>
            {/* Stats Bar */}
            <StatsBar stats={stats} />

            {/* Category Filters */}
            <CategoryFilters
              categories={categoryStats}
              selectedCategory={selectedCategory}
              onCategorySelect={setSelectedCategory}
            />

            {/* Treemap Visualization */}
            <TypeTreemap
              data={treemapData}
              onTypeSelect={handleTypeSelect}
            />

            {/* Type Table */}
            <TypeTable
              types={allTypes}
              onTypeSelect={handleTypeSelect}
              showCategory={true}
            />
          </>
        ) : (
          <TypeDetailView
            typeData={selectedTypeData}
            graphData={graphData}
            onBack={handleBackToOverview}
            onTypeSelect={handleTypeSelect}
          />
        )}
      </main>

      {/* Footer */}
      <footer className="bg-slate-800 border-t border-slate-700 mt-20">
        <div className="max-w-7xl mx-auto px-6 py-8 text-center text-gray-400">
          <p>
            Generated from{' '}
            <a 
              href="https://github.com/composable-delivery/busbar-sf-types" 
              target="_blank" 
              rel="noopener noreferrer"
              className="text-blue-400 hover:text-blue-300 transition-colors"
            >
              busbar-sf-types
            </a>
            {' • '}
            Data:{' '}
            <a 
              href="/busbar-sf-types/type-graph.json" 
              target="_blank" 
              rel="noopener noreferrer"
              className="text-blue-400 hover:text-blue-300 transition-colors"
            >
              type-graph.json
            </a>
          </p>
        </div>
      </footer>
    </div>
  );
}

export default App;
