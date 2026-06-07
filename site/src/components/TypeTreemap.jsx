import { TreeMap } from "reaviz";

const categoryColors = [
    "#222222", // uncategorized (Darker Gray)
    "#2563eb", // category 1 (Blue)
    "#16a34a", // category 2 (Green)
    "#9333ea", // category 3 (Purple)
    "#dc2626", // category 4 (Red)
    "#0891b2", // category 5 (Cyan)
    "#ea580c", // category 6 (Orange)
    "#4f46e5", // category 7 (Indigo)
    "#ca8a04", // category 8 (Yellow)
];

export default function TypeTreeMap({ data, onTypeSelect }) {
    const handleCellClick = (event) => {
        if (event.data && event.data.metadata) {
            onTypeSelect(event.data.metadata.name);
        }
    };

    if (!data || data.length === 0) {
        return (
            <div className="flex items-center justify-center h-full text-gray-700 italic border border-dashed border-white/10 rounded-2xl">
                No distribution data available
            </div>
        );
    }

    return (
        <div className="w-full h-full relative">
            <TreeMap
                data={data}
                onClick={handleCellClick}
                colorScheme={categoryColors}
                padding={2}
                border={{
                    width: 1,
                    color: "#000000",
                }}
                label={{
                    visible: true,
                    fontFamily: "Inter, system-ui, sans-serif",
                    fontSize: 11,
                    fill: "#ffffff",
                    fontWeight: 700,
                }}
                rect={{
                    rx: 4,
                    ry: 4,
                    padding: 2,
                    stroke: "rgba(0,0,0,0.5)",
                    strokeWidth: 1,
                }}
            />
        </div>
    );
}
