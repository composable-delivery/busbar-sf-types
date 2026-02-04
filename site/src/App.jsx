import { useState, useEffect, useMemo } from "react";
import Hero from "./components/Hero";
import StatsBar from "./components/StatsBar";
import TypeTreemap from "./components/TypeTreemap";
import CategoryFilters from "./components/CategoryFilters";
import TypeTable from "./components/TypeTable";
import TypeDetailView from "./components/TypeDetailView";
import NavigationTree from "./components/NavigationTree";
import {
    buildTypeMetadata,
    assignCategories,
    buildTreemapData,
    getCategoryStats,
    getRelationshipStats,
    parseDotClusters,
    searchTypes,
} from "./utils/dataProcessing";

function App() {
    const [graphData, setGraphData] = useState(null);
    const [dotClusters, setDotClusters] = useState([]);
    const [metadata, setMetadata] = useState(null);
    const [selectedType, setSelectedType] = useState(null);
    const [selectedCategory, setSelectedCategory] = useState(null);
    const [searchQuery, setSearchQuery] = useState("");
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);

    // Load graph data and dot file
    useEffect(() => {
        const loadData = async () => {
            try {
                // Load graph JSON
                const graphResponse = await fetch(
                    `${import.meta.env.BASE_URL}type-graph.json`,
                );
                const graphJson = await graphResponse.json();
                setGraphData(graphJson);

                // Load DOT file for clusters
                const dotResponse = await fetch(
                    `${import.meta.env.BASE_URL}type-graph.dot`,
                );
                const dotText = await dotResponse.text();
                const clusters = parseDotClusters(dotText);
                setDotClusters(clusters);

                console.log("[v0] Loaded graph data:", {
                    nodes: graphJson.nodes.length,
                    edges: graphJson.edges.length,
                    clusters: clusters.length,
                });

                // Build metadata
                const typeMetadata = buildTypeMetadata(graphJson);
                const enrichedMetadata = assignCategories(
                    typeMetadata,
                    clusters,
                );
                setMetadata(enrichedMetadata);

                setLoading(false);
            } catch (err) {
                console.error("[v0] Error loading data:", err);
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
                relationshipStats: {},
            };
        }

        const categoryStats = getCategoryStats(metadata);
        const relationshipStats = getRelationshipStats(graphData);

        return {
            totalTypes: graphData.nodes.length,
            totalDependencies: graphData.edges.length,
            totalCategories: categoryStats.length,
            relationshipStats,
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
        console.log("[v0] Type selected:", typeName);
        setSelectedType(typeName);
        window.scrollTo({ top: 0, behavior: "smooth" });
    };

    const handleBackToOverview = () => {
        setSelectedType(null);
        setSearchQuery("");
    };

    if (loading) {
        return (
            <div className="min-h-screen bg-slate-900 flex items-center justify-center">
                <div className="text-center">
                    <div className="text-4xl font-bold text-blue-400 mb-4 animate-pulse">
                        Loading Salesforce Types Explorer...
                    </div>
                    <div className="text-gray-400">
                        Processing {stats.totalTypes || "2,682"} types
                    </div>
                </div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="min-h-screen bg-slate-900 flex items-center justify-center">
                <div className="max-w-2xl bg-slate-800 border border-red-500 p-8 rounded-lg">
                    <h2 className="text-2xl font-bold text-red-400 mb-4">
                        Error Loading Data
                    </h2>
                    <p className="text-gray-300">
                        Could not load the type graph data: {error}
                    </p>
                </div>
            </div>
        );
    }

    return (
        <div className="flex h-screen bg-black text-white overflow-hidden font-sans selection:bg-blue-500/30">
            {/* Sidebar Navigation */}
            {metadata && (
                <NavigationTree
                    metadata={metadata}
                    onTypeSelect={handleTypeSelect}
                    activeType={selectedType}
                />
            )}

            {/* Main Content Area */}
            <main className="flex-1 overflow-y-auto overflow-x-hidden relative scroll-smooth bg-black">
                {/* Visual context gradients */}
                <div className="absolute top-0 right-0 w-[500px] h-[500px] bg-blue-600/5 blur-[120px] rounded-full pointer-events-none" />
                <div className="absolute bottom-0 left-0 w-[400px] h-[400px] bg-indigo-600/5 blur-[120px] rounded-full pointer-events-none" />

                <div className="relative z-10 p-8 lg:p-12 min-h-full">
                    {selectedType ? (
                        <TypeDetailView
                            typeData={metadata.get(selectedType)}
                            onBack={handleBackToOverview}
                            onTypeSelect={handleTypeSelect}
                        />
                    ) : (
                        <div className="max-w-7xl mx-auto space-y-16 animate-fade-in">
                            <Hero
                                searchQuery={searchQuery}
                                onSearchChange={setSearchQuery}
                                onSearch={handleSearch}
                            />

                            <StatsBar stats={stats} />

                            <div className="grid grid-cols-1 3xl:grid-cols-2 gap-16 items-start">
                                <section className="space-y-8 min-w-0">
                                    <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-6 px-2">
                                        <h2 className="text-4xl font-black text-white uppercase tracking-tighter italic">
                                            Distribution
                                        </h2>
                                        <CategoryFilters
                                            categories={categoryStats}
                                            selectedCategory={selectedCategory}
                                            onCategorySelect={
                                                setSelectedCategory
                                            }
                                        />
                                    </div>
                                    <div className="bg-black border border-white/10 rounded-[2rem] p-8 h-[700px] shadow-[0_0_50px_-12px_rgba(59,130,246,0.15)] relative overflow-hidden">
                                        <TypeTreemap
                                            data={treemapData}
                                            onTypeSelect={handleTypeSelect}
                                        />
                                    </div>
                                </section>

                                <section className="space-y-8 min-w-0">
                                    <h2 className="text-4xl font-black text-white uppercase tracking-tighter italic px-2">
                                        Type Registry
                                    </h2>
                                    <div className="bg-black border border-white/10 rounded-[2rem] overflow-hidden shadow-2xl">
                                        <TypeTable
                                            types={allTypes.filter(
                                                (t) =>
                                                    !selectedCategory ||
                                                    t.category?.name ===
                                                        selectedCategory,
                                            )}
                                            onTypeSelect={handleTypeSelect}
                                        />
                                    </div>
                                </section>
                            </div>
                        </div>
                    )}
                </div>
            </main>
        </div>
    );
}

export default App;
