import { GraphCanvas } from "reagraph";
import { Card } from "reablocks";
import { useMemo } from "react";

export default function TypeNetwork({ graphData, selectedType, onTypeSelect }) {
    const graphNodes = useMemo(() => {
        if (!graphData || !graphData.nodes) return [];

        return graphData.nodes.map((node) => ({
            id: node,
            label: node,
            fill: node === selectedType ? "#3b82f6" : "#64748b",
            size: node === selectedType ? 20 : 10,
        }));
    }, [graphData, selectedType]);

    const graphEdges = useMemo(() => {
        if (!graphData || !graphData.edges) return [];

        return graphData.edges.map((edge, index) => ({
            id: `edge-${index}`,
            source: edge.source,
            target: edge.target,
            label: edge.relationship,
        }));
    }, [graphData]);

    const handleNodeClick = (node) => {
        console.log("[v0] Graph node clicked:", node);
        onTypeSelect(node.id);
    };

    if (!graphData || graphNodes.length === 0) {
        return (
            <Card className="bg-slate-800 border border-slate-700 p-12 text-center">
                <div className="text-gray-400">No graph data available</div>
            </Card>
        );
    }

    return (
        <Card className="bg-slate-800 border border-slate-700 p-6">
            <div className="mb-4">
                <h3 className="text-xl font-bold text-white">
                    Dependency Network
                </h3>
                <p className="text-sm text-gray-400 mt-1">
                    {selectedType
                        ? `Showing connections for ${selectedType}`
                        : "Click a node to explore its connections"}
                </p>
            </div>

            <div className="h-[600px] rounded-lg overflow-hidden border border-slate-700 bg-slate-900">
                <GraphCanvas
                    nodes={graphNodes}
                    edges={graphEdges}
                    onNodeClick={handleNodeClick}
                    layoutType="forceDirected2d"
                    edgeInterpolation="curved"
                    theme={{
                        canvas: {
                            background: "#000000",
                        },
                        node: {
                            fill: "#ffffff",
                            activeFill: "#3b82f6",
                            opacity: 1,
                            selectedOpacity: 1,
                            label: {
                                color: "#ffffff",
                                stroke: "#000000",
                                distance: 5,
                                fontSize: 12,
                                show: true,
                                activeFill: "#3b82f6",
                            },
                        },
                        edge: {
                            fill: "#333333",
                            activeFill: "#3b82f6",
                            opacity: 0.4,
                            selectedOpacity: 1,
                            label: {
                                color: "#999999",
                                stroke: "#000000",
                                fontSize: 10,
                                show: true,
                                activeFill: "#3b82f6",
                            },
                        },
                        lasso: {
                            border: "1px solid #3b82f6",
                            background: "rgba(59, 130, 246, 0.2)",
                            show: true,
                        },
                        arrow: {
                            fill: "#333333",
                            activeFill: "#3b82f6",
                            show: true,
                        },
                        ring: {
                            fill: "#3b82f6",
                            activeFill: "#60a5fa",
                        },
                    }}
                />
            </div>
        </Card>
    );
}
