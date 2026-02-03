// Global state
let graphData = null;
let selectedType = null;
let simulation = null;
let svg = null;
let g = null;
let zoom = null;
let links = null;
let nodes = null;

// Relationship type filters
const filters = {
    contains: true,
    extends: true,
    generic: true
};

// Load the type graph data
async function loadGraphData() {
    try {
        const response = await fetch('../assets/type-graph.json');
        graphData = await response.json();
        initializeApp();
    } catch (error) {
        console.error('Error loading graph data:', error);
        document.getElementById('typeDetails').innerHTML = `
            <h2>Error Loading Data</h2>
            <p>Could not load the type graph data. Please ensure the site is properly deployed.</p>
            <p>Error: ${error.message}</p>
        `;
    }
}

// Initialize the application
function initializeApp() {
    // Update statistics
    document.getElementById('totalTypes').textContent = graphData.nodes.length.toLocaleString();
    document.getElementById('totalEdges').textContent = graphData.edges.length.toLocaleString();
    
    const uniqueCategories = new Set(Object.values(graphData.categories));
    document.getElementById('totalCategories').textContent = uniqueCategories.size.toLocaleString();
    
    // Set generated date
    const now = new Date().toLocaleDateString('en-US', { 
        year: 'numeric', 
        month: 'long', 
        day: 'numeric' 
    });
    document.getElementById('generatedDate').textContent = `Updated: ${now}`;
    
    // Setup search
    setupSearch();
    
    // Setup filters
    setupFilters();
    
    // Setup categories
    setupCategories();
    
    // Initialize graph visualization
    initializeGraph();
}

// Setup search functionality
function setupSearch() {
    const searchInput = document.getElementById('searchInput');
    let searchTimeout;
    
    searchInput.addEventListener('input', (e) => {
        clearTimeout(searchTimeout);
        searchTimeout = setTimeout(() => {
            const query = e.target.value.trim().toLowerCase();
            if (query.length > 0) {
                searchTypes(query);
            } else {
                // Clear search results
                if (selectedType) {
                    showTypeDetails(selectedType);
                } else {
                    showWelcomeMessage();
                }
            }
        }, 300);
    });
}

// Search for types
function searchTypes(query) {
    const results = graphData.nodes.filter(node => 
        node.toLowerCase().includes(query)
    );
    
    displaySearchResults(results, query);
}

// Display search results
function displaySearchResults(results, query) {
    const detailsDiv = document.getElementById('typeDetails');
    
    if (results.length === 0) {
        detailsDiv.innerHTML = `
            <h2>No Results</h2>
            <p>No types found matching "<strong>${query}</strong>"</p>
        `;
        return;
    }
    
    const resultsHTML = results.slice(0, 50).map(type => `
        <div class="dependency-item" onclick="selectType('${type}')">
            ${type}
            <span class="relationship-badge contains">${graphData.categories[type] || 'unknown'}</span>
        </div>
    `).join('');
    
    detailsDiv.innerHTML = `
        <h2>Search Results</h2>
        <p>Found ${results.length} types matching "<strong>${query}</strong>"</p>
        ${results.length > 50 ? '<p><em>Showing first 50 results</em></p>' : ''}
        <div class="dependency-list" style="margin-top: 1rem;">
            ${resultsHTML}
        </div>
    `;
}

// Setup relationship filters
function setupFilters() {
    document.getElementById('filterContains').addEventListener('change', (e) => {
        filters.contains = e.target.checked;
        updateGraphVisualization();
    });
    
    document.getElementById('filterExtends').addEventListener('change', (e) => {
        filters.extends = e.target.checked;
        updateGraphVisualization();
    });
    
    document.getElementById('filterGeneric').addEventListener('change', (e) => {
        filters.generic = e.target.checked;
        updateGraphVisualization();
    });
}

// Setup categories sidebar
function setupCategories() {
    const categoriesContent = document.getElementById('categoriesContent');
    
    // Get unique categories and count types in each
    const categoryMap = {};
    Object.entries(graphData.categories).forEach(([type, category]) => {
        if (!categoryMap[category]) {
            categoryMap[category] = [];
        }
        categoryMap[category].push(type);
    });
    
    // Sort categories by name
    const sortedCategories = Object.keys(categoryMap).sort();
    
    const categoriesHTML = sortedCategories.map(category => {
        const count = categoryMap[category].length;
        return `
            <div class="category-item" onclick="selectCategory('${category}')">
                ${category} <span style="opacity: 0.6;">(${count})</span>
            </div>
        `;
    }).join('');
    
    categoriesContent.innerHTML = categoriesHTML;
}

// Select a category
function selectCategory(category) {
    const types = Object.entries(graphData.categories)
        .filter(([_, cat]) => cat === category)
        .map(([type, _]) => type);
    
    displayCategoryTypes(category, types);
}

// Display types in a category
function displayCategoryTypes(category, types) {
    const detailsDiv = document.getElementById('typeDetails');
    
    const typesHTML = types.slice(0, 100).map(type => `
        <div class="dependency-item" onclick="selectType('${type}')">
            ${type}
        </div>
    `).join('');
    
    detailsDiv.innerHTML = `
        <h2>Category: ${category}</h2>
        <p>${types.length} types in this category</p>
        ${types.length > 100 ? '<p><em>Showing first 100 types</em></p>' : ''}
        <div class="dependency-list" style="margin-top: 1rem;">
            ${typesHTML}
        </div>
    `;
}

// Select and display a type
function selectType(typeName) {
    selectedType = typeName;
    showTypeDetails(typeName);
    highlightTypeInGraph(typeName);
    
    // Clear search
    document.getElementById('searchInput').value = '';
}

// Show type details
function showTypeDetails(typeName) {
    const dependencies = graphData.edges.filter(edge => edge.source === typeName);
    const dependents = graphData.edges.filter(edge => edge.target === typeName);
    const category = graphData.categories[typeName] || 'unknown';
    
    // Group dependencies by relationship type
    const depsByRelationship = {
        Contains: dependencies.filter(d => d.relationship === 'Contains'),
        Extends: dependencies.filter(d => d.relationship === 'Extends'),
        Generic: dependencies.filter(d => d.relationship === 'Generic')
    };
    
    const dependenciesHTML = Object.entries(depsByRelationship)
        .filter(([_, deps]) => deps.length > 0)
        .map(([rel, deps]) => `
            <h3>${rel} Dependencies (${deps.length})</h3>
            <ul class="dependency-list">
                ${deps.map(dep => `
                    <li class="dependency-item" onclick="selectType('${dep.target}')">
                        ${dep.target}
                        <span class="relationship-badge ${dep.relationship.toLowerCase()}">${dep.relationship}</span>
                    </li>
                `).join('')}
            </ul>
        `).join('');
    
    const dependentsHTML = dependents.length > 0 ? `
        <h3>Used By (${dependents.length})</h3>
        <ul class="dependency-list">
            ${dependents.slice(0, 20).map(dep => `
                <li class="dependency-item" onclick="selectType('${dep.source}')">
                    ${dep.source}
                    <span class="relationship-badge ${dep.relationship.toLowerCase()}">${dep.relationship}</span>
                </li>
            `).join('')}
            ${dependents.length > 20 ? `<li style="padding: 0.5rem; color: var(--text-secondary);"><em>... and ${dependents.length - 20} more</em></li>` : ''}
        </ul>
    ` : '';
    
    document.getElementById('typeDetails').innerHTML = `
        <h2>${typeName}</h2>
        <div class="info-box">
            <strong>Category:</strong> ${category}<br>
            <strong>Dependencies:</strong> ${dependencies.length}<br>
            <strong>Used by:</strong> ${dependents.length} types
        </div>
        ${dependenciesHTML}
        ${dependentsHTML}
    `;
}

// Show welcome message
function showWelcomeMessage() {
    document.getElementById('typeDetails').innerHTML = `
        <h2>Welcome to Salesforce Types Explorer</h2>
        <p>Select a type from the search results or click on a node in the graph to explore dependencies.</p>
        <div class="info-box">
            <h3>About</h3>
            <p>This interactive tool visualizes the dependency graph of Salesforce metadata types. The graph shows:</p>
            <ul>
                <li><strong>Contains:</strong> Type A has a field of Type B</li>
                <li><strong>Extends:</strong> Type A extends/inherits from Type B</li>
                <li><strong>Generic:</strong> Type A is a generic instantiation of Type B</li>
            </ul>
        </div>
    `;
}

// Initialize D3.js graph visualization
function initializeGraph() {
    const container = document.getElementById('graph');
    const width = container.clientWidth;
    const height = 600;
    
    // Create SVG
    svg = d3.select('#graph')
        .attr('width', width)
        .attr('height', height);
    
    // Setup zoom behavior
    zoom = d3.zoom()
        .scaleExtent([0.1, 10])
        .on('zoom', (event) => {
            g.attr('transform', event.transform);
        });
    
    svg.call(zoom);
    
    // Create container group
    g = svg.append('g');
    
    // Setup graph controls
    document.getElementById('resetZoom').addEventListener('click', resetZoom);
    document.getElementById('centerGraph').addEventListener('click', centerGraph);
    document.getElementById('fitToScreen').addEventListener('click', fitToScreen);
    
    // Render initial graph with a subset of data
    renderGraph();
}

// Render the graph visualization
function renderGraph() {
    // For performance, show only types with significant connections
    const nodeSet = new Set();
    const filteredEdges = [];
    
    // Filter edges based on relationship filters
    graphData.edges.forEach(edge => {
        const rel = edge.relationship.toLowerCase();
        if ((rel === 'contains' && filters.contains) ||
            (rel === 'extends' && filters.extends) ||
            (rel === 'generic' && filters.generic)) {
            filteredEdges.push(edge);
            nodeSet.add(edge.source);
            nodeSet.add(edge.target);
        }
    });
    
    // Limit to most connected types for performance (top 200 by connection count)
    const connectionCounts = {};
    filteredEdges.forEach(edge => {
        connectionCounts[edge.source] = (connectionCounts[edge.source] || 0) + 1;
        connectionCounts[edge.target] = (connectionCounts[edge.target] || 0) + 1;
    });
    
    const topNodes = Object.entries(connectionCounts)
        .sort((a, b) => b[1] - a[1])
        .slice(0, 200)
        .map(([node, _]) => node);
    
    const topNodeSet = new Set(topNodes);
    const displayEdges = filteredEdges.filter(edge => 
        topNodeSet.has(edge.source) && topNodeSet.has(edge.target)
    );
    
    // Prepare data
    const nodesData = topNodes.map(node => ({
        id: node,
        category: graphData.categories[node] || 'unknown'
    }));
    
    // Clear previous graph
    g.selectAll('*').remove();
    
    // Create force simulation
    simulation = d3.forceSimulation(nodesData)
        .force('link', d3.forceLink(displayEdges)
            .id(d => d.id)
            .distance(100))
        .force('charge', d3.forceManyBody().strength(-300))
        .force('center', d3.forceCenter(svg.attr('width') / 2, svg.attr('height') / 2))
        .force('collision', d3.forceCollide().radius(30));
    
    // Create links
    links = g.append('g')
        .selectAll('line')
        .data(displayEdges)
        .join('line')
        .attr('class', d => `link ${d.relationship.toLowerCase()}`)
        .attr('stroke-width', 1.5);
    
    // Create nodes
    nodes = g.append('g')
        .selectAll('g')
        .data(nodesData)
        .join('g')
        .attr('class', 'node')
        .call(d3.drag()
            .on('start', dragstarted)
            .on('drag', dragged)
            .on('end', dragended));
    
    nodes.append('circle')
        .attr('r', 8)
        .attr('fill', d => getCategoryColor(d.category));
    
    nodes.append('text')
        .attr('dy', 20)
        .text(d => d.id.length > 15 ? d.id.substring(0, 15) + '...' : d.id)
        .style('font-size', '10px')
        .style('fill', '#333');
    
    // Add hover effects
    nodes.on('mouseover', function(event, d) {
        showTooltip(event, d);
        d3.select(this).select('circle').attr('r', 12);
    })
    .on('mouseout', function() {
        hideTooltip();
        d3.select(this).select('circle').attr('r', 8);
    })
    .on('click', function(event, d) {
        selectType(d.id);
    });
    
    // Update positions on each tick
    simulation.on('tick', () => {
        links
            .attr('x1', d => d.source.x)
            .attr('y1', d => d.source.y)
            .attr('x2', d => d.target.x)
            .attr('y2', d => d.target.y);
        
        nodes.attr('transform', d => `translate(${d.x},${d.y})`);
    });
}

// Update graph visualization based on filters
function updateGraphVisualization() {
    renderGraph();
}

// Highlight a type in the graph
function highlightTypeInGraph(typeName) {
    if (!nodes) return;
    
    nodes.classed('selected', d => d.id === typeName);
    
    // Highlight related links
    if (links) {
        links.classed('highlighted', d => 
            d.source.id === typeName || d.target.id === typeName
        );
    }
}

// Get color for a category
function getCategoryColor(category) {
    const colors = {
        'common': '#0176d3',
        'objects': '#2e844a',
        'permissions': '#7b1fa2',
        'flows': '#e65100',
        'apex': '#d32f2f',
        'lwc': '#00838f',
        'ai': '#6a1b9a',
        'datacloud': '#00695c'
    };
    
    return colors[category] || '#546e7a';
}

// Drag functions
function dragstarted(event, d) {
    if (!event.active) simulation.alphaTarget(0.3).restart();
    d.fx = d.x;
    d.fy = d.y;
}

function dragged(event, d) {
    d.fx = event.x;
    d.fy = event.y;
}

function dragended(event, d) {
    if (!event.active) simulation.alphaTarget(0);
    d.fx = null;
    d.fy = null;
}

// Tooltip functions
function showTooltip(event, d) {
    const tooltip = document.getElementById('tooltip');
    const dependencies = graphData.edges.filter(e => e.source === d.id).length;
    const dependents = graphData.edges.filter(e => e.target === d.id).length;
    
    tooltip.innerHTML = `
        <strong>${d.id}</strong><br>
        Category: ${d.category}<br>
        Dependencies: ${dependencies}<br>
        Used by: ${dependents} types
    `;
    
    tooltip.style.left = (event.pageX + 10) + 'px';
    tooltip.style.top = (event.pageY - 10) + 'px';
    tooltip.classList.add('visible');
}

function hideTooltip() {
    document.getElementById('tooltip').classList.remove('visible');
}

// Zoom controls
function resetZoom() {
    svg.transition().duration(750).call(zoom.transform, d3.zoomIdentity);
}

function centerGraph() {
    const bounds = g.node().getBBox();
    const parent = svg.node().getBoundingClientRect();
    const fullWidth = parent.width;
    const fullHeight = parent.height;
    const width = bounds.width;
    const height = bounds.height;
    const midX = bounds.x + width / 2;
    const midY = bounds.y + height / 2;
    
    const translate = [fullWidth / 2 - midX, fullHeight / 2 - midY];
    
    svg.transition().duration(750).call(
        zoom.transform,
        d3.zoomIdentity.translate(translate[0], translate[1])
    );
}

function fitToScreen() {
    const bounds = g.node().getBBox();
    const parent = svg.node().getBoundingClientRect();
    const fullWidth = parent.width;
    const fullHeight = parent.height;
    const width = bounds.width;
    const height = bounds.height;
    const midX = bounds.x + width / 2;
    const midY = bounds.y + height / 2;
    
    const scale = 0.9 / Math.max(width / fullWidth, height / fullHeight);
    const translate = [fullWidth / 2 - scale * midX, fullHeight / 2 - scale * midY];
    
    svg.transition().duration(750).call(
        zoom.transform,
        d3.zoomIdentity.translate(translate[0], translate[1]).scale(scale)
    );
}

// Initialize on page load
window.addEventListener('DOMContentLoaded', loadGraphData);
