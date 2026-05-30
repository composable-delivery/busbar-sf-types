export const RELATIONSHIP_COLORS = {
    Contains: "#3b82f6",
    Extends: "#a855f7",
    Generic: "#f97316",
    AliasOf: "#14b8a6",
    UnionMember: "#ec4899",
    IntersectionMember: "#6366f1",
    GenericBase: "#f59e0b",
    GenericArg: "#fb923c",
    CollectionOf: "#22c55e",
    MapKey: "#eab308",
    MapValue: "#facc15",
    References: "#38bdf8",
    LookupRelationship: "#22d3ee",
    MasterDetailRelationship: "#0ea5e9",
    FormulaReference: "#34d399",
    ValidationReference: "#f472b6",
    RollupSummary: "#f59e0b",
    Unknown: "#6b7280",
};

export function getRelationshipColor(relationship) {
    if (!relationship) return RELATIONSHIP_COLORS.Unknown;
    return RELATIONSHIP_COLORS[relationship] || RELATIONSHIP_COLORS.Unknown;
}
