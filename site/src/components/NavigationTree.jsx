import React, { useState, useMemo } from "react";

export default function NavigationTree({ metadata, onTypeSelect, activeType }) {
    const [expandedCategories, setExpandedCategories] = useState(new Set());

    const categories = useMemo(() => {
        const groups = new Map();

        metadata.forEach((data, typeName) => {
            const catName = data.category?.name || "uncategorized";
            if (!groups.has(catName)) {
                groups.set(catName, []);
            }
            groups.get(catName).push(data);
        });

        return Array.from(groups.entries())
            .map(([name, types]) => ({
                name,
                types: types.sort((a, b) => a.name.localeCompare(b.name)),
            }))
            .sort((a, b) => a.name.localeCompare(b.name));
    }, [metadata]);

    const toggleCategory = (name) => {
        const next = new Set(expandedCategories);
        if (next.has(name)) {
            next.delete(name);
        } else {
            next.add(name);
        }
        setExpandedCategories(next);
    };

    return (
        <div className="bg-black border-r border-white/10 h-full overflow-y-auto w-80 flex-shrink-0">
            <div className="p-6 border-b border-white/10">
                <h2 className="text-white font-black text-xl tracking-tight uppercase">
                    Metadata Explorer
                </h2>
                <p className="text-xs text-gray-500 mt-1 uppercase font-bold tracking-widest">
                    {metadata.size} Types Loaded
                </p>
            </div>

            <div className="py-4">
                {categories.map((cat) => (
                    <div key={cat.name} className="mb-1">
                        <button
                            onClick={() => toggleCategory(cat.name)}
                            className="w-full flex items-center justify-between px-6 py-2 hover:bg-white/5 transition-colors group"
                        >
                            <div className="flex items-center gap-3">
                                <span
                                    className={`text-xs transition-transform duration-200 ${expandedCategories.has(cat.name) ? "rotate-90" : ""}`}
                                >
                                    ▶
                                </span>
                                <span
                                    className={`text-sm font-bold uppercase tracking-wider ${expandedCategories.has(cat.name) ? "text-white" : "text-gray-500"}`}
                                >
                                    {cat.name}
                                </span>
                            </div>
                            <span className="text-[10px] bg-white/10 px-2 py-0.5 rounded text-gray-400 group-hover:text-white transition-colors">
                                {cat.types.length}
                            </span>
                        </button>

                        {expandedCategories.has(cat.name) && (
                            <div className="bg-white/5 py-1">
                                {cat.types.map((type) => (
                                    <button
                                        key={type.name}
                                        onClick={() => onTypeSelect(type.name)}
                                        className={`w-full text-left px-12 py-1.5 text-sm transition-all hover:text-white ${
                                            activeType === type.name
                                                ? "text-blue-400 font-bold border-l-2 border-blue-400 bg-blue-400/10"
                                                : "text-gray-400"
                                        }`}
                                    >
                                        {type.name}
                                    </button>
                                ))}
                            </div>
                        )}
                    </div>
                ))}
            </div>
        </div>
    );
}
