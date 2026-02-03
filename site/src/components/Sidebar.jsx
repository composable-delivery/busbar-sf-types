import React, { useMemo } from 'react';

function Sidebar({ graphData, searchQuery, setSearchQuery, filters, onFilterChange, onTypeSelect, viewMode }) {
  // Calculate statistics
  const stats = useMemo(() => {
    if (!graphData) return { totalTypes: 0, totalEdges: 0, totalCategories: 0 };
    const uniqueCategories = new Set(Object.values(graphData.categories));
    return {
      totalTypes: graphData.nodes.length,
      totalEdges: graphData.edges.length,
      totalCategories: uniqueCategories.size,
    };
  }, [graphData]);

  // Group types by category
  const categoryMap = useMemo(() => {
    if (!graphData) return {};
    const map = {};
    Object.entries(graphData.categories).forEach(([type, category]) => {
      if (!map[category]) {
        map[category] = [];
      }
      map[category].push(type);
    });
    return map;
  }, [graphData]);

  const sortedCategories = useMemo(() => {
    return Object.keys(categoryMap).sort();
  }, [categoryMap]);

  const handleCategoryClick = (category) => {
    const types = categoryMap[category];
    if (types.length > 0) {
      onTypeSelect(types[0]); // Select first type in category
    }
  };

  return (
    <aside className="sidebar">
      <div className="sidebar-section">
        <input
          type="text"
          placeholder="Search types..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="search-input"
        />
      </div>

      <div className="sidebar-section">
        <h3>Statistics</h3>
        <div className="stat-item">
          <span className="stat-label">Total Types:</span>
          <span className="stat-value">{stats.totalTypes.toLocaleString()}</span>
        </div>
        <div className="stat-item">
          <span className="stat-label">Dependencies:</span>
          <span className="stat-value">{stats.totalEdges.toLocaleString()}</span>
        </div>
        <div className="stat-item">
          <span className="stat-label">Categories:</span>
          <span className="stat-value">{stats.totalCategories.toLocaleString()}</span>
        </div>
      </div>

      {viewMode === 'graph' && (
        <div className="sidebar-section">
          <h3>Relationship Filters</h3>
          <label className="filter-label">
            <input
              type="checkbox"
              checked={filters.contains}
              onChange={(e) => onFilterChange('contains', e.target.checked)}
            />
            <span className="relationship-badge contains">Contains</span>
          </label>
          <label className="filter-label">
            <input
              type="checkbox"
              checked={filters.extends}
              onChange={(e) => onFilterChange('extends', e.target.checked)}
            />
            <span className="relationship-badge extends">Extends</span>
          </label>
          <label className="filter-label">
            <input
              type="checkbox"
              checked={filters.generic}
              onChange={(e) => onFilterChange('generic', e.target.checked)}
            />
            <span className="relationship-badge generic">Generic</span>
          </label>
        </div>
      )}

      <div className="sidebar-section">
        <h3>Categories</h3>
        <div className="categories-list">
          {sortedCategories.map((category) => (
            <div
              key={category}
              className="category-item"
              onClick={() => handleCategoryClick(category)}
            >
              {category} <span style={{ opacity: 0.6 }}>({categoryMap[category].length})</span>
            </div>
          ))}
        </div>
      </div>
    </aside>
  );
}

export default Sidebar;
