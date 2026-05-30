import { useState, useMemo } from "react";
import { List } from "reablocks";

export default function CategoryTree({ metadata, clusters, onTypeSelect }) {
    const [expandedCategories, setExpandedCategories] = useState(new Set());

    // Build tree structure from categories and types
    const categoryData = useMemo(() => {
        // Group types by category
        const categoryGroups = new Map();

        metadata.forEach((data, typeName) => {
            const categoryName = data.category?.name || "uncategorized";
            if (!categoryGroups.has(categoryName)) {
                categoryGroups.set(categoryName, []);
            }
            categoryGroups.get(categoryName).push({
                name: typeName,
                connections: data.connectionCount,
            });
        });

        // Sort categories alphabetically
        const sortedCategories = Array.from(categoryGroups.entries()).sort(
            ([a], [b]) => a.localeCompare(b),
        );

        return sortedCategories.map(([categoryName, types]) => {
            // Sort types by connection count descending
            const sortedTypes = types.sort(
                (a, b) => b.connections - a.connections,
            );

            return {
                name: categoryName,
                types: sortedTypes,
            };
        });
    }, [metadata, clusters]);

    const toggleCategory = (categoryName) => {
        const newExpanded = new Set(expandedCategories);
        if (newExpanded.has(categoryName)) {
            newExpanded.delete(categoryName);
        } else {
            newExpanded.add(categoryName);
        }
        setExpandedCategories(newExpanded);
    };

    return (
        <div className="h-full overflow-auto bg-black border-r border-white/10">
            <div className="p-4 border-b border-white/10">
                <h2 className="text-sm font-bold text-white uppercase tracking-widest">
                    Type Explorer
                </h2>
                <p className="text-xs text-gray-500 mt-1">
                    {metadata.size} types across {categoryData.length}{" "}
                    categories
                </p>
            </div>
            <div className="p-2">
                {categoryData.map((category) => {
                    const isExpanded = expandedCategories.has(category.name);
                    return (
                        <div key={category.name} className="mb-2">
                            <button
                                onClick={() => toggleCategory(category.name)}
                                className="w-full text-left px-3 py-2 rounded-lg hover:bg-white/5 transition-colors flex items-center justify-between"
                            >
                                <span className="flex items-center gap-2">
                                    <span className="text-gray-500">
                                        {isExpanded ? "▼" : "▶"}
                                    </span>
                                    <span className="font-bold uppercase text-xs tracking-wider text-white">
                                        {category.name}
                                    </span>
                                </span>
                                <span className="text-gray-500 text-xs">
                                    ({category.types.length})
                                </span>
                            </button>
                            {isExpanded && (
                                <div className="ml-6 mt-1 space-y-1">
                                    {category.types.map((type) => (
                                        <button
                                            key={type.name}
                                            onClick={() =>
                                                onTypeSelect(type.name)
                                            }
                                            className="w-full text-left px-3 py-1.5 rounded-lg hover:bg-white/10 transition-colors flex items-center justify-between text-sm"
                                        >
                                            <span className="text-gray-200">
                                                {type.name}
                                            </span>
                                            <span className="text-gray-500 text-xs">
                                                {type.connections}
                                            </span>
                                        </button>
                                    ))}
                                </div>
                            )}
                        </div>
                    );
                })}
            </div>
        </div>
    );
}
