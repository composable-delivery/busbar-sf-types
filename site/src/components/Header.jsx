import React from 'react';
import { Button } from 'reablocks';

function Header({ viewMode, onViewModeChange }) {
  return (
    <header className="header">
      <div className="container header-content">
        <div>
          <h1>🔍 Salesforce Types Explorer</h1>
          <p>Interactive visualization of Salesforce metadata type dependencies</p>
        </div>
        <div className="view-mode-buttons">
          <Button
            variant={viewMode === 'dashboard' ? 'filled' : 'outline'}
            color={viewMode === 'dashboard' ? 'primary' : 'default'}
            onClick={() => onViewModeChange('dashboard')}
          >
            📊 Dashboard
          </Button>
          <Button
            variant={viewMode === 'graph' ? 'filled' : 'outline'}
            color={viewMode === 'graph' ? 'primary' : 'default'}
            onClick={() => onViewModeChange('graph')}
          >
            🔗 Graph
          </Button>
        </div>
      </div>
    </header>
  );
}

export default Header;
