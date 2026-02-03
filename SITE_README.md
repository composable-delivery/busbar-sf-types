# Salesforce Types Explorer - React Site

This directory contains the source code for the interactive GitHub Pages site built with React, reagraph, and reablocks.

## Technology Stack

- **React 19** - UI framework
- **Reagraph** - WebGL-based graph visualization for interactive dependency graphs
- **Reaviz** - Data visualization library (for potential future enhancements)
- **Reablocks** - UI component library (ThemeProvider)
- **Vite** - Build tool and dev server

## Development

### Prerequisites

- Node.js 16+
- npm or yarn

### Getting Started

```bash
# Install dependencies (from repo root)
npm install

# Start development server
npm run dev

# Visit http://localhost:5173/busbar-sf-types/
```

### Building

```bash
# Build for production (from repo root)
npm run build

# Output will be in ../docs/ directory
```

## Project Structure

```
site/
├── src/
│   ├── components/
│   │   ├── Header.jsx           # Site header
│   │   ├── Sidebar.jsx          # Search, filters, categories
│   │   ├── GraphVisualization.jsx  # Reagraph-based graph display
│   │   ├── TypeDetails.jsx      # Type information with tabs
│   │   └── SimpleTabs.jsx       # Custom tabs component
│   ├── App.jsx                  # Main application component
│   ├── main.jsx                 # React entry point
│   └── styles.css               # Global styles
├── public/
│   └── type-graph.json          # Graph data (copied from assets/)
└── index.html                   # HTML template
```

## Features

### Graph Visualization
- **Interactive WebGL rendering** via reagraph
- Click nodes to navigate to types
- Force-directed layout
- Overview mode showing top categories
- Type-specific mode showing dependencies

### Type Details
- **Tabbed interface** for Details and JSON Schema
- Shows dependencies grouped by relationship type (Contains, Extends, Generic)
- Lists types that depend on the selected type
- Category and statistics

### Search & Navigation
- Real-time search across all types
- Category-based browsing
- Relationship type filters (Contains, Extends, Generic)
- Click any dependency to navigate

### Layout
- **Graph always on top** - Appears before type details
- Sticky sidebar with statistics
- Responsive design
- Clean, modern UI

## Build Output

The build process:
1. Compiles React components to optimized JavaScript
2. Bundles dependencies
3. Outputs to `../docs/` directory for GitHub Pages
4. Copies `type-graph.json` from `public/` directory

## Future Enhancements

- **JSON Schema display** - Show actual schemas when generation is implemented
- **Graph zoom and pan** - Enhanced graph navigation
- **Data visualizations** - Charts using reaviz for statistics
- **Export features** - Download graphs or type information
