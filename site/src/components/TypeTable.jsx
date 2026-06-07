import { Card, Button, Input } from "reablocks";
import { useState, useMemo } from "react";

export default function TypeTable({
    types,
    onTypeSelect,
    showCategory = true,
}) {
    const [sortField, setSortField] = useState("connectionCount");
    const [sortDirection, setSortDirection] = useState("desc");
    const [filterText, setFilterText] = useState("");

    const sortedAndFiltered = useMemo(() => {
        let filtered = types;

        if (filterText) {
            const lower = filterText.toLowerCase();
            filtered = types.filter((t) =>
                t.name.toLowerCase().includes(lower),
            );
        }

        return [...filtered].sort((a, b) => {
            let aVal = a[sortField];
            let bVal = b[sortField];

            if (sortField === "category") {
                aVal = a.category?.name || "uncategorized";
                bVal = b.category?.name || "uncategorized";
            }

            if (sortField === "name") {
                return sortDirection === "asc"
                    ? aVal.localeCompare(bVal)
                    : bVal.localeCompare(aVal);
            }

            return sortDirection === "asc" ? aVal - bVal : bVal - aVal;
        });
    }, [types, sortField, sortDirection, filterText]);

    const handleSort = (field) => {
        if (sortField === field) {
            setSortDirection(sortDirection === "asc" ? "desc" : "asc");
        } else {
            setSortField(field);
            setSortDirection("desc");
        }
    };

    const SortIcon = ({ field }) => {
        if (sortField !== field) return null;
        return sortDirection === "asc" ? " ↑" : " ↓";
    };

    return (
        <div className="bg-black border border-white/10 p-6 rounded-xl shadow-2xl">
            <div className="mb-6 flex flex-col md:flex-row md:items-center justify-between gap-4">
                <Input
                    placeholder="Filter registry by name..."
                    value={filterText}
                    onChange={(e) => setFilterText(e.target.value)}
                    className="max-w-md bg-white/5 border-white/10 text-white placeholder:text-gray-500 rounded-lg py-2"
                />
                <div className="text-[10px] text-gray-500 uppercase tracking-widest font-bold">
                    Showing {sortedAndFiltered.length} of {types.length} types
                </div>
            </div>

            <div className="overflow-x-auto">
                <table className="w-full text-sm border-collapse">
                    <thead className="border-b border-white/10">
                        <tr className="text-left text-gray-400">
                            <th
                                className="py-4 px-4 font-semibold cursor-pointer hover:text-white transition-colors"
                                onClick={() => handleSort("name")}
                            >
                                Type Name
                                <SortIcon field="name" />
                            </th>
                            {showCategory && (
                                <th
                                    className="py-4 px-4 font-semibold cursor-pointer hover:text-white transition-colors"
                                    onClick={() => handleSort("category")}
                                >
                                    Category
                                    <SortIcon field="category" />
                                </th>
                            )}
                            <th
                                className="py-4 px-4 font-semibold cursor-pointer hover:text-white transition-colors text-right"
                                onClick={() => handleSort("connectionCount")}
                            >
                                Connections
                                <SortIcon field="connectionCount" />
                            </th>
                            <th className="py-4 px-4 font-semibold text-right">
                                Actions
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        {sortedAndFiltered.map((type) => (
                            <tr
                                key={type.name}
                                className="border-b border-white/5 hover:bg-white/5 transition-colors group"
                            >
                                <td className="py-4 px-4 font-medium text-white group-hover:text-blue-400 transition-colors">
                                    {type.name}
                                </td>
                                {showCategory && (
                                    <td className="py-4 px-4 text-gray-400">
                                        <span className="inline-flex items-center px-2 py-1 rounded-md text-xs font-mono bg-white/5 border border-white/10 text-gray-300">
                                            {type.category?.name ||
                                                "uncategorized"}
                                        </span>
                                    </td>
                                )}
                                <td className="py-4 px-4 text-right text-gray-400 font-mono">
                                    {type.connectionCount}
                                </td>
                                <td className="py-4 px-4 text-right">
                                    <Button
                                        size="small"
                                        variant="outline"
                                        className="border-white/20 text-white hover:bg-white hover:text-black transition-all rounded-md"
                                        onClick={() => onTypeSelect(type.name)}
                                    >
                                        Explore
                                    </Button>
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>

            {sortedAndFiltered.length === 0 && (
                <div className="text-center py-20 text-gray-600 border border-dashed border-white/5 rounded-xl m-4">
                    No types found matching "{filterText}"
                </div>
            )}
        </div>
    );
}
