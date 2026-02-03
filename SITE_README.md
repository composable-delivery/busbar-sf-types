# Salesforce Types Explorer - React Site

This directory contains the completely redesigned interactive GitHub Pages site built with React, reagraph, reaviz, reablocks, and Tailwind CSS.

## Technology Stack

- **React 19.2** - UI framework
- **Reagraph 4.30** - WebGL-based graph visualization for interactive dependency graphs
- **Reaviz 16.1** - Data visualization library for treemap visualization
- **Reablocks 9.2** - UI component library (Cards, Buttons, Tabs, etc.)
- **Tailwind CSS 3.4** - Utility-first CSS framework with custom dark theme
- **Vite 7.3** - Build tool and dev server

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
│   │   ├── Hero.jsx                # Landing page hero with search
│   │   ├── StatsBar.jsx            # Animated statistics counters
│   │   ├── TypeTreemap.jsx         # Reaviz treemap visualization
│   │   ├── CategoryFilters.jsx     # Horizontal category filter pills
│   │   ├── TypeTable.jsx           # Sortable, filterable type table
│   │   ├── TypeNetwork.jsx         # Reagraph network visualization
│   │   └── TypeDetailView.jsx      # Complete type detail page
│   ├── utils/
│   │   └── dataProcessing.js       # Data utilities and transformations
│   ├── App.jsx                     # Main application component
│   ├── main.jsx                    # React entry point with theme
│   └── styles.css                  # Tailwind CSS imports only
├── public/
│   ├── type-graph.json             # Graph data
│   └── type-graph.dot              # DOT file with clusters
├── tailwind.config.js              # Tailwind configuration
├── postcss.config.js               # PostCSS configuration
├── vite.config.js                  # Vite build configuration
└── index.html                      # HTML template
```

## Features

### Landing Page (No Type Selected)
- **Visually Impactful Hero** - Large heading, description, and global search
- **Animated Statistics Bar** - 6 stat cards with animated counters showing totals
- **Category Filters** - Horizontal scrolling pills from DOT file clusters
- **Interactive Treemap** - Reaviz treemap sized by connection count, colored by category
- **Type Explorer Table** - Sortable, filterable table of all 2,682+ types

### Type Detail View (Type Selected)
- **Breadcrumb Navigation** - Easy return to overview
- **Type Header Card** - Name, category badge, connection statistics
- **Network Visualization** - Reagraph showing local dependency graph
- **Tabbed Relationships** - Separate tabs for dependencies and dependents
- **Clickable Navigation** - Click any related type to navigate

### Data Processing
- **DOT File Parsing** - Extracts cluster categories from type-graph.dot
- **Metadata Enrichment** - Builds comprehensive type metadata with relationships
- **Treemap Data Generation** - Hierarchical data for visualization
- **Search & Filter** - Fast client-side search and category filtering

### Design
- **Pure Tailwind CSS** - No custom CSS classes, all utility-based
- **Dark Theme** - Optimized for data visualization with blue accents
- **Animations** - Fade-in, slide-up, scale-in for smooth UX
- **Responsive** - Mobile-friendly layouts

## Build Output

The build process:
1. Compiles React components to optimized JavaScript
2. Bundles dependencies
3. Outputs to `../docs/` directory for GitHub Pages
4. Copies `type-graph.json` from `public/` directory

## Design System

### Colors
- **Primary**: Blue (#3b82f6) - Main accent color
- **Surface**: Slate (#1e293b) - Card backgrounds
- **Background**: Dark Slate (#0f172a) - Page background
- **Categories**: 8 distinct colors for visual differentiation

### Typography
- **Font**: Inter (Google Fonts)
- **Weights**: 400 (regular), 500 (medium), 600 (semibold), 700 (bold), 800 (extrabold)

### Animations
- **fade-in**: 0.5s ease-in
- **slide-up**: 0.6s ease-out
- **scale-in**: 0.4s ease-out
- **Animated counters**: 2s duration with easing

## Key Changes from Previous Version

### Removed
- ❌ Custom CSS (600+ lines of .css classes)
- ❌ Old component structure (Header, Sidebar, Dashboard)
- ❌ Dashboard-first navigation pattern

### Added
- ✅ Tailwind CSS with dark theme
- ✅ Treemap visualization for overview
- ✅ Category filtering from DOT clusters
- ✅ Animated statistics
- ✅ Data processing utilities
- ✅ Visually impactful landing page

### Improved
- 🔄 Better UX for navigating 500+ types
- 🔄 Clearer distinction between overview and detail views
- 🔄 Proper use of reagraph, reaviz, and reablocks
- 🔄 Faster, more intuitive navigation
