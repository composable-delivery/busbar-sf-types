import React, { useMemo } from 'react';
import { BarList } from 'reaviz';

function Dashboard({ graphData, onTypeSelect }) {
  const stats = useMemo(() => {
    if (!graphData) return null;

    // Category statistics
    const categoryStats = {};
    Object.entries(graphData.categories).forEach(([type, category]) => {
      if (!categoryStats[category]) {
        categoryStats[category] = { count: 0, types: [] };
      }
      categoryStats[category].count++;
      categoryStats[category].types.push(type);
    });

    // Relationship statistics
    const relationshipStats = {
      Contains: 0,
      Extends: 0,
      Generic: 0,
    };
    graphData.edges.forEach((edge) => {
      if (relationshipStats[edge.relationship] !== undefined) {
        relationshipStats[edge.relationship]++;
      }
    });

    // Top categories by type count
    const topCategories = Object.entries(categoryStats)
      .sort((a, b) => b[1].count - a[1].count)
      .slice(0, 15);

    // Most connected types
    const typeConnections = {};
    graphData.edges.forEach((edge) => {
      typeConnections[edge.source] = (typeConnections[edge.source] || 0) + 1;
      typeConnections[edge.target] = (typeConnections[edge.target] || 0) + 1;
    });
    const mostConnected = Object.entries(typeConnections)
      .sort((a, b) => b[1] - a[1])
      .slice(0, 10);

    // Format data for reaviz BarList
    const relationshipChartData = Object.entries(relationshipStats).map(([key, data]) => ({
      key,
      data
    }));

    const topCategoriesChartData = topCategories.map(([category, stats]) => ({
      key: category,
      data: stats.count,
      metadata: stats.types
    }));

    return {
      totalTypes: graphData.nodes.length,
      totalEdges: graphData.edges.length,
      totalCategories: Object.keys(categoryStats).length,
      categoryStats,
      relationshipStats,
      relationshipChartData,
      topCategories,
      topCategoriesChartData,
      mostConnected,
    };
  }, [graphData]);

  if (!stats) return null;

  return (
    <div className="dashboard">
      {/* Overview Cards */}
      <div className="stat-cards">
        <div className="stat-card">
          <div className="stat-value" style={{ color: '#0176d3' }}>
            {stats.totalTypes.toLocaleString()}
          </div>
          <div className="stat-label">Total Types</div>
        </div>
        <div className="stat-card">
          <div className="stat-value" style={{ color: '#2e844a' }}>
            {stats.totalEdges.toLocaleString()}
          </div>
          <div className="stat-label">Dependencies</div>
        </div>
        <div className="stat-card">
          <div className="stat-value" style={{ color: '#7b1fa2' }}>
            {stats.totalCategories}
          </div>
          <div className="stat-label">Categories</div>
        </div>
      </div>

      {/* Relationship Stats with Reaviz */}
      <div className="card">
        <h3>Relationship Distribution</h3>
        <div style={{ height: '200px', padding: '1rem 0' }}>
          <BarList
            data={stats.relationshipChartData}
          />
        </div>
      </div>

      {/* Most Connected Types */}
      <div className="card">
        <h3>Most Connected Types</h3>
        <div className="type-list">
          {stats.mostConnected.map(([type, connections]) => (
            <div
              key={type}
              className="type-list-item"
              onClick={() => onTypeSelect(type)}
            >
              <span className="type-name">{type}</span>
              <span className="connection-badge">
                {connections} connections
              </span>
            </div>
          ))}
        </div>
      </div>

      {/* Top Categories with Reaviz */}
      <div className="card">
        <h3>Top Categories by Type Count</h3>
        <div style={{ height: '400px', padding: '1rem 0' }}>
          <BarList
            data={stats.topCategoriesChartData}
            onItemClick={(item) => {
              if (item.metadata && item.metadata.length > 0) {
                onTypeSelect(item.metadata[0]);
              }
            }}
          />
        </div>
      </div>
    </div>
  );
}

export default Dashboard;
