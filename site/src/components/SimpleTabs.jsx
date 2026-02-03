import React from 'react';

function SimpleTabs({ children, value, onChange }) {
  const tabs = React.Children.toArray(children);
  
  return (
    <div className="simple-tabs">
      <div className="tabs-header">
        {tabs.map((tab) => (
          <button
            key={tab.props.value}
            className={`tab-button ${value === tab.props.value ? 'active' : ''}`}
            onClick={() => onChange(tab.props.value)}
          >
            {tab.props.label}
          </button>
        ))}
      </div>
      <div className="tabs-content">
        {tabs.find((tab) => tab.props.value === value)?.props.children}
      </div>
    </div>
  );
}

function SimpleTab({ children }) {
  return <>{children}</>;
}

export { SimpleTabs, SimpleTab };
