// Global state
let graphData = null;
let selectedType = null;
let canvas = null;
let ctx = null;

// Relationship type filters
const filters = {
    contains: true,
    extends: true,
    generic: true
};

// Load the type graph data
async function loadGraphData() {
    try {
        const response = await fetch('type-graph.json');
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
    
    // Initialize canvas
    canvas = document.getElementById('graph');
    ctx = canvas.getContext('2d');
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
        if (selectedType) {
            drawGraph(selectedType);
        }
    });
    
    document.getElementById('filterExtends').addEventListener('change', (e) => {
        filters.extends = e.target.checked;
        if (selectedType) {
            drawGraph(selectedType);
        }
    });
    
    document.getElementById('filterGeneric').addEventListener('change', (e) => {
        filters.generic = e.target.checked;
        if (selectedType) {
            drawGraph(selectedType);
        }
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
    drawGraph(typeName);
    
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
        <p>Select a type from the search results or browse by category to explore dependencies.</p>
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

// Draw graph visualization using Canvas
function drawGraph(typeName) {
    if (!ctx) return;
    
    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    // Get dependencies for the selected type
    const dependencies = graphData.edges.filter(edge => {
        if (edge.source !== typeName) return false;
        const rel = edge.relationship.toLowerCase();
        return (rel === 'contains' && filters.contains) ||
               (rel === 'extends' && filters.extends) ||
               (rel === 'generic' && filters.generic);
    });
    
    if (dependencies.length === 0) {
        ctx.fillStyle = '#706e6b';
        ctx.font = '16px sans-serif';
        ctx.textAlign = 'center';
        ctx.fillText('No dependencies to display', canvas.width / 2, canvas.height / 2);
        return;
    }
    
    // Calculate layout
    const centerX = canvas.width / 2;
    const centerY = canvas.height / 2;
    const radius = Math.min(canvas.width, canvas.height) * 0.3;
    
    // Draw center node (selected type)
    ctx.fillStyle = '#0176d3';
    ctx.beginPath();
    ctx.arc(centerX, centerY, 30, 0, 2 * Math.PI);
    ctx.fill();
    
    ctx.fillStyle = '#fff';
    ctx.font = 'bold 14px sans-serif';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillText(typeName.length > 12 ? typeName.substring(0, 12) + '...' : typeName, centerX, centerY);
    
    // Draw dependencies in a circle
    const angleStep = (2 * Math.PI) / dependencies.length;
    dependencies.forEach((dep, index) => {
        const angle = index * angleStep - Math.PI / 2;
        const x = centerX + radius * Math.cos(angle);
        const y = centerY + radius * Math.sin(angle);
        
        // Draw line
        const lineColor = getRelationshipColor(dep.relationship);
        ctx.strokeStyle = lineColor;
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.moveTo(centerX, centerY);
        ctx.lineTo(x, y);
        ctx.stroke();
        
        // Draw node
        ctx.fillStyle = getCategoryColor(graphData.categories[dep.target] || 'unknown');
        ctx.beginPath();
        ctx.arc(x, y, 20, 0, 2 * Math.PI);
        ctx.fill();
        
        // Draw label
        ctx.fillStyle = '#181818';
        ctx.font = '11px sans-serif';
        ctx.textAlign = 'center';
        ctx.textBaseline = 'top';
        const label = dep.target.length > 10 ? dep.target.substring(0, 10) + '...' : dep.target;
        ctx.fillText(label, x, y + 25);
    });
    
    // Draw legend
    drawLegend();
}

// Draw legend for relationship types
function drawLegend() {
    const legendX = 10;
    const legendY = canvas.height - 80;
    
    ctx.font = '12px sans-serif';
    ctx.textAlign = 'left';
    ctx.textBaseline = 'middle';
    
    const relationships = [
        { name: 'Contains', color: '#1565c0' },
        { name: 'Extends', color: '#7b1fa2' },
        { name: 'Generic', color: '#e65100' }
    ];
    
    relationships.forEach((rel, index) => {
        const y = legendY + index * 20;
        
        // Draw line
        ctx.strokeStyle = rel.color;
        ctx.lineWidth = 3;
        ctx.beginPath();
        ctx.moveTo(legendX, y);
        ctx.lineTo(legendX + 30, y);
        ctx.stroke();
        
        // Draw label
        ctx.fillStyle = '#181818';
        ctx.fillText(rel.name, legendX + 40, y);
    });
}

// Get color for relationship type
function getRelationshipColor(relationship) {
    const colors = {
        'Contains': '#1565c0',
        'Extends': '#7b1fa2',
        'Generic': '#e65100'
    };
    return colors[relationship] || '#546e7a';
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
        'ai_ml': '#6a1b9a',
        'datacloud': '#00695c',
        'data_cloud': '#00695c'
    };
    
    return colors[category] || '#546e7a';
}

// Initialize on page load
window.addEventListener('DOMContentLoaded', loadGraphData);
