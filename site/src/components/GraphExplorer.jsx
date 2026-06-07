import { useMemo, useState } from "react";
import { GraphCanvas } from "reagraph";
import { Card, Button, Select, Input } from "reablocks";
import { buildLocalGraph } from "../utils/dataProcessing";
import { getRelationshipColor } from "../utils/graphStyles";

const layoutOptions = [
    { value: "forceDirected2d", label: "Force (2D)" },
    { value: "radialOut", label: "Radial" },
    { value: "dagre", label: "DAG (Dagre)" },
    { value: "tree", label: "Tree" },
];

const HUB_EXCLUSIONS = new Set(["ApiSchemaTypes", "Metadata"]);

export default function GraphExplorer({
    graphData,
    metadata,
    selectedType,
    onTypeSelect,
}) {
    const [layoutType, setLayoutType] = useState("forceDirected2d");
    const [mode, setMode] = useState("full");
    const [depth, setDepth] = useState(2);
    const [maxNodes, setMaxNodes] = useState(300);
    const [hideHubTypes, setHideHubTypes] = useState(true);
    const [relationshipFilter, setRelationshipFilter] = useState(null);

    const relationshipOptions = useMemo(() => {
        if (!graphData) return [];
        const set = new Set();
        graphData.edges.forEach((edge) => set.add(edge.relationship || "Unknown"));
        return Array.from(set).sort();
    }, [graphData]);

    const filterSet = useMemo(() => {
        if (!relationshipFilter) return null;
        return new Set(relationshipFilter);
    }, [relationshipFilter]);

    const filteredGraph = useMemo(() => {
        if (!graphData) return { nodes: [], edges: [] };

        const includeEdge = (edge) => {
            if (!filterSet) return true;
            return filterSet.has(edge.relationship || "Unknown");
        };

        if (mode === "focused" && selectedType && metadata) {
            const local = buildLocalGraph(metadata, selectedType, depth);
            const edges = local.edges.filter(includeEdge);
            const nodes = new Set();
            edges.forEach((edge) => {
                nodes.add(edge.source);
                nodes.add(edge.target);
            });
            nodes.add(selectedType);
            return { nodes: Array.from(nodes), edges };
        }

        let nodes = new Set(graphData.nodes);
        if (maxNodes > 0 && metadata && nodes.size > maxNodes) {
            const ranked = Array.from(metadata.values())
                .sort((a, b) => b.connectionCount - a.connectionCount)
                .slice(0, maxNodes)
                .map((item) => item.name);
            nodes = new Set(ranked);
        }

        if (hideHubTypes) {
            HUB_EXCLUSIONS.forEach((name) => nodes.delete(name));
        }

        const edges = graphData.edges.filter(
            (edge) =>
                includeEdge(edge) && nodes.has(edge.source) && nodes.has(edge.target),
        );

        return { nodes: Array.from(nodes), edges };
    }, [graphData, metadata, selectedType, mode, depth, filterSet, maxNodes, hideHubTypes]);

    const graphNodes = useMemo(() => {
        if (!filteredGraph.nodes) return [];
        return filteredGraph.nodes.map((node) => {
            const nodeMeta = metadata?.get(node);
            const size = nodeMeta
                ? Math.min(28, 6 + Math.sqrt(nodeMeta.connectionCount || 1))
                : 8;
            return {
                id: node,
                label: node,
                size,
                fill: node === selectedType ? "#3b82f6" : "#64748b",
            };
        });
    }, [filteredGraph, metadata, selectedType]);

    const graphEdges = useMemo(() => {
        if (!filteredGraph.edges) return [];
        return filteredGraph.edges.map((edge, index) => ({
            id: `edge-${edge.source}-${edge.target}-${index}`,
            source: edge.source,
            target: edge.target,
            label: edge.relationship,
            fill: getRelationshipColor(edge.relationship),
        }));
    }, [filteredGraph]);

    const toggleRelationship = (rel) => {
        if (!relationshipFilter) {
            setRelationshipFilter([rel]);
            return;
        }

        if (relationshipFilter.includes(rel)) {
            const next = relationshipFilter.filter((r) => r !== rel);
            setRelationshipFilter(next.length ? next : null);
        } else {
            setRelationshipFilter([...relationshipFilter, rel]);
        }
    };

    const selectAllRelationships = () => {
        setRelationshipFilter(null);
    };

    return (
        <Card className="bg-black border border-white/10 p-8 rounded-4xl shadow-2xl">
            <div className="flex flex-col xl:flex-row xl:items-center justify-between gap-6">
                <div>
                    <h2 className="text-3xl font-black text-white uppercase tracking-tight">
                        Graph Explorer
                    </h2>
                    <p className="text-gray-500 text-sm mt-1">
                        Explore the full DAG or a focused neighborhood using
                        reagraph layouts.
                    </p>
                </div>
                <div className="flex flex-wrap gap-3">
                    <Select
                        size="small"
                        value={layoutType}
                        onChange={(value) => setLayoutType(value)}
                        options={layoutOptions}
                        className="min-w-44"
                    />
                    <div className="flex gap-2">
                        <Button
                            size="small"
                            variant={mode === "full" ? "filled" : "outline"}
                            onClick={() => setMode("full")}
                        >
                            Full DAG
                        </Button>
                        <Button
                            size="small"
                            variant={mode === "focused" ? "filled" : "outline"}
                            onClick={() => setMode("focused")}
                        >
                            Focused
                        </Button>
                    </div>
                    {mode === "focused" && (
                        <div className="flex items-center gap-2">
                            <span className="text-xs text-gray-500">
                                Depth
                            </span>
                            <Input
                                type="number"
                                size="small"
                                value={depth}
                                min={1}
                                max={4}
                                onChange={(e) =>
                                    setDepth(
                                        Math.max(
                                            1,
                                            Math.min(4, Number(e.target.value)),
                                        ),
                                    )
                                }
                                className="w-20 bg-white/5 border border-white/10 text-white"
                            />
                        </div>
                    )}
                    {mode === "full" && (
                        <div className="flex items-center gap-2">
                            <span className="text-xs text-gray-500">
                                Max Nodes
                            </span>
                            <Input
                                type="number"
                                size="small"
                                value={maxNodes}
                                min={0}
                                max={3000}
                                onChange={(e) =>
                                    setMaxNodes(
                                        Math.max(
                                            0,
                                            Math.min(3000, Number(e.target.value)),
                                        ),
                                    )
                                }
                                className="w-24 bg-white/5 border border-white/10 text-white"
                            />
                        </div>
                    )}
                    {mode === "full" && (
                        <Button
                            size="small"
                            variant={hideHubTypes ? "filled" : "outline"}
                            onClick={() => setHideHubTypes((prev) => !prev)}
                        >
                            Hide Hub Types
                        </Button>
                    )}
                </div>
            </div>

            <div className="mt-6 flex flex-wrap gap-2">
                <Button
                    size="small"
                    variant={relationshipFilter ? "outline" : "filled"}
                    onClick={selectAllRelationships}
                >
                    All Relationships
                </Button>
                {relationshipOptions.map((rel) => {
                    const isActive = !relationshipFilter
                        ? true
                        : relationshipFilter.includes(rel);
                    return (
                        <Button
                            key={rel}
                            size="small"
                            variant={isActive ? "filled" : "outline"}
                            onClick={() => toggleRelationship(rel)}
                        >
                            {rel}
                        </Button>
                    );
                })}
            </div>

            <div className="mt-6 h-180 rounded-2xl overflow-hidden border border-white/10 bg-black">
                <GraphCanvas
                    nodes={graphNodes}
                    edges={graphEdges}
                    layoutType={layoutType}
                    animated={false}
                    labelType="none"
                    defaultNodeSize={6}
                    minNodeSize={4}
                    maxNodeSize={14}
                    onNodeClick={(node) => node?.id && onTypeSelect(node.id)}
                    edgeInterpolation="curved"
                    edgeArrowPosition="end"
                    theme={{
                        canvas: { background: "#000000" },
                        node: {
                            fill: "#ffffff",
                            activeFill: "#3b82f6",
                            opacity: 1,
                            selectedOpacity: 1,
                            label: {
                                color: "#ffffff",
                                stroke: "#000000",
                                distance: 5,
                                fontSize: 11,
                                show: true,
                                activeFill: "#3b82f6",
                            },
                        },
                        edge: {
                            fill: "#2d2d2d",
                            activeFill: "#3b82f6",
                            opacity: 0.4,
                            selectedOpacity: 1,
                            label: {
                                color: "#999999",
                                stroke: "#000000",
                                fontSize: 10,
                                show: false,
                                activeFill: "#3b82f6",
                            },
                        },
                        ring: {
                            fill: "#3b82f6",
                            activeFill: "#60a5fa",
                        },
                        arrow: {
                            fill: "#2d2d2d",
                            activeFill: "#3b82f6",
                            show: true,
                        },
                        lasso: {
                            border: "1px solid #3b82f6",
                            background: "rgba(59, 130, 246, 0.2)",
                            show: true,
                        },
                    }}
                />
            </div>
        </Card>
    );
}
