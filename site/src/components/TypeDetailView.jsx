import { Card, Button, Tabs, TabList, Tab, TabPanel } from "reablocks";
import TypeNetwork from "./TypeNetwork";
import { useMemo } from "react";

export default function TypeDetailView({
    typeData,
    graphData,
    onBack,
    onTypeSelect,
}) {
    const relationshipBadgeClass = (rel) => {
        const baseClass =
            "inline-flex items-center px-3 py-1 rounded-full text-xs font-medium";
        switch (rel) {
            case "Contains":
                return `${baseClass} bg-blue-500/20 text-blue-300`;
            case "Extends":
                return `${baseClass} bg-purple-500/20 text-purple-300`;
            case "Generic":
                return `${baseClass} bg-orange-500/20 text-orange-300`;
            default:
                return `${baseClass} bg-gray-500/20 text-gray-300`;
        }
    };

    const localGraph = useMemo(() => {
        if (!typeData || !graphData) return { nodes: [], edges: [] };

        const nodes = new Set([typeData.name]);
        const edges = [];

        // Add direct dependencies
        typeData.dependencies.forEach((dep) => {
            nodes.add(dep.target);
            edges.push({
                source: typeData.name,
                target: dep.target,
                relationship: dep.relationship,
            });
        });

        // Add direct dependents
        typeData.dependents.forEach((dep) => {
            nodes.add(dep.source);
            edges.push({
                source: dep.source,
                target: typeData.name,
                relationship: dep.relationship,
            });
        });

        return {
            nodes: Array.from(nodes),
            edges: edges,
        };
    }, [typeData, graphData]);

    if (!typeData) {
        return (
            <Card className="bg-slate-800 border border-slate-700 p-12 text-center">
                <div className="text-gray-400">Type not found</div>
            </Card>
        );
    }

    return (
        <div className="space-y-8 animate-fade-in max-w-6xl mx-auto">
            {/* Navigation */}
            <div className="flex items-center">
                <Button
                    variant="text"
                    onClick={onBack}
                    className="text-gray-400 hover:text-white transition-colors p-0 flex items-center gap-2 group"
                >
                    <span className="group-hover:-translate-x-1 transition-transform">
                        ←
                    </span>
                    <span>Home</span>
                </Button>
            </div>

            {/* Type Header */}
            <div className="bg-black border border-white/10 p-10 rounded-2xl shadow-2xl relative overflow-hidden">
                <div className="absolute top-0 right-0 p-8 opacity-10 pointer-events-none">
                    <div className="text-8xl font-black text-white">
                        {typeData.name[0]}
                    </div>
                </div>

                <div className="relative z-10 flex flex-col md:flex-row md:items-end justify-between gap-8">
                    <div className="space-y-4">
                        <div className="inline-flex items-center px-3 py-1 rounded-full text-xs font-bold tracking-widest uppercase bg-blue-600/20 text-blue-400 border border-blue-600/30">
                            {typeData.category?.name || "uncategorized"}
                        </div>
                        <h1 className="text-5xl lg:text-7xl font-black text-white tracking-tight">
                            {typeData.name}
                        </h1>
                        <div className="text-gray-400 font-medium text-lg">
                            {typeData.connectionCount} total connections
                            discovered
                        </div>
                    </div>

                    <div className="flex gap-6">
                        <div className="bg-white/5 border border-white/10 p-6 rounded-xl min-w-[140px]">
                            <div className="text-4xl font-bold text-white mb-1">
                                {typeData.dependencies.length}
                            </div>
                            <div className="text-[10px] text-gray-500 uppercase tracking-widest font-bold">
                                Dependencies
                            </div>
                        </div>
                        <div className="bg-white/5 border border-white/10 p-6 rounded-xl min-w-[140px]">
                            <div className="text-4xl font-bold text-white mb-1">
                                {typeData.dependents.length}
                            </div>
                            <div className="text-[10px] text-gray-500 uppercase tracking-widest font-bold">
                                Dependents
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            {/* Graph Section */}
            <div className="h-[700px]">
                <TypeNetwork
                    graphData={localGraph}
                    selectedType={typeData.name}
                    onTypeSelect={(nodeId) => {
                        if (nodeId !== typeData.name) {
                            onTypeSelect(nodeId);
                        }
                    }}
                />
            </div>

            {/* Connection Lists */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
                <Card className="bg-black border border-white/10 p-8 rounded-2xl shadow-xl">
                    <h3 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
                        <span className="w-2 h-2 rounded-full bg-blue-500"></span>
                        Dependencies ({typeData.dependencies.length})
                    </h3>
                    {typeData.dependencies.length === 0 ? (
                        <div className="py-12 text-center text-gray-600 border border-dashed border-white/10 rounded-xl">
                            This type has no defined dependencies
                        </div>
                    ) : (
                        <div className="grid gap-2">
                            {typeData.dependencies.map((dep, index) => (
                                <div
                                    key={index}
                                    className="flex items-center justify-between p-4 bg-white/5 hover:bg-white/10 border border-white/5 rounded-xl transition-all cursor-pointer group"
                                    onClick={() => onTypeSelect(dep.target)}
                                >
                                    <span className="font-medium text-gray-200 group-hover:text-blue-400 transition-colors">
                                        {dep.target}
                                    </span>
                                    <span
                                        className={relationshipBadgeClass(
                                            dep.relationship,
                                        )}
                                    >
                                        {dep.relationship}
                                    </span>
                                </div>
                            ))}
                        </div>
                    )}
                </Card>

                <Card className="bg-black border border-white/10 p-8 rounded-2xl shadow-xl">
                    <h3 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
                        <span className="w-2 h-2 rounded-full bg-green-500"></span>
                        Dependents ({typeData.dependents.length})
                    </h3>
                    {typeData.dependents.length === 0 ? (
                        <div className="py-12 text-center text-gray-600 border border-dashed border-white/10 rounded-xl">
                            No types depend on this type
                        </div>
                    ) : (
                        <div className="grid gap-2">
                            {typeData.dependents.map((dep, index) => (
                                <div
                                    key={index}
                                    className="flex items-center justify-between p-4 bg-white/5 hover:bg-white/10 border border-white/5 rounded-xl transition-all cursor-pointer group"
                                    onClick={() => onTypeSelect(dep.source)}
                                >
                                    <span className="font-medium text-gray-200 group-hover:text-green-400 transition-colors">
                                        {dep.source}
                                    </span>
                                    <span
                                        className={relationshipBadgeClass(
                                            dep.relationship,
                                        )}
                                    >
                                        {dep.relationship}
                                    </span>
                                </div>
                            ))}
                        </div>
                    )}
                </Card>
            </div>
        </div>
    );
}
