import React from 'react';

function Header({ viewMode, onViewModeChange }) {
  return (
    <header className="header">
      <div className="container header-content">
        <div>
          <h1>🔍 Salesforce Types Explorer</h1>
          <p>Interactive visualization of Salesforce metadata type dependencies</p>
        </div>
        <div className="view-mode-buttons">
          <button
            className={`view-mode-btn ${viewMode === 'dashboard' ? 'active' : ''}`}
            onClick={() => onViewModeChange('dashboard')}
          >
            📊 Dashboard
          </button>
          <button
            className={`view-mode-btn ${viewMode === 'graph' ? 'active' : ''}`}
            onClick={() => onViewModeChange('graph')}
          >
            🔗 Graph
          </button>
        </div>
      </div>
    </header>
  );
}

export default Header;
