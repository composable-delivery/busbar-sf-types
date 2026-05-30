import { useMemo } from "react";
import { Card } from "reablocks";
import { getRelationshipStats, getTopConnectedTypes } from "../utils/dataProcessing";
import { RELATIONSHIP_COLORS } from "../utils/graphStyles";

export default function GraphInsights({ graphData, metadata, onTypeSelect }) {
    const relationshipStats = useMemo(() => {
        if (!graphData) return {};
        return getRelationshipStats(graphData);
    }, [graphData]);

    const topTypes = useMemo(() => {
        if (!metadata) return [];
        return getTopConnectedTypes(metadata, 12);
    }, [metadata]);

    return (
        <div className="space-y-8">
            <Card className="bg-black border border-white/10 p-6 rounded-3xl">
                <h3 className="text-lg font-bold text-white mb-4">
                    Relationship Breakdown
                </h3>
                <div className="space-y-3">
                    {Object.entries(relationshipStats).map(([rel, count]) => (
                        <div
                            key={rel}
                            className="flex items-center justify-between text-sm"
                        >
                            <div className="flex items-center gap-3">
                                <span
                                    className="w-3 h-3 rounded-full"
                                    style={{
                                        background:
                                            RELATIONSHIP_COLORS[rel] ||
                                            RELATIONSHIP_COLORS.Unknown,
                                    }}
                                />
                                <span className="text-gray-300">
                                    {rel}
                                </span>
                            </div>
                            <span className="text-gray-500">
                                {count.toLocaleString()}
                            </span>
                        </div>
                    ))}
                </div>
            </Card>

            <Card className="bg-black border border-white/10 p-6 rounded-3xl">
                <h3 className="text-lg font-bold text-white mb-4">
                    Most Connected Types
                </h3>
                <div className="space-y-2">
                    {topTypes.map((item) => (
                        <button
                            key={item.name}
                            className="w-full text-left flex items-center justify-between px-3 py-2 rounded-xl bg-white/5 hover:bg-white/10 transition"
                            onClick={() => onTypeSelect(item.name)}
                        >
                            <span className="text-gray-200 text-sm font-medium">
                                {item.name}
                            </span>
                            <span className="text-xs text-gray-500">
                                {item.connectionCount}
                            </span>
                        </button>
                    ))}
                </div>
            </Card>

            <Card className="bg-black border border-white/10 p-6 rounded-3xl">
                <h3 className="text-lg font-bold text-white mb-4">
                    Legend
                </h3>
                <div className="grid grid-cols-2 gap-3">
                    {Object.entries(RELATIONSHIP_COLORS).map(([rel, color]) => (
                        <div key={rel} className="flex items-center gap-2">
                            <span
                                className="w-3 h-3 rounded-full"
                                style={{ background: color }}
                            />
                            <span className="text-xs text-gray-400">
                                {rel}
                            </span>
                        </div>
                    ))}
                </div>
            </Card>
        </div>
    );
}
