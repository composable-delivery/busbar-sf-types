import { Treemap } from 'reaviz';
import { Card } from 'reablocks';

const categoryColors = [
  '#64748b', // uncategorized
  '#f59e0b', // category 1
  '#10b981', // category 2
  '#8b5cf6', // category 3
  '#ec4899', // category 4
  '#14b8a6', // category 5
  '#f97316', // category 6
  '#6366f1', // category 7
  '#84cc16', // category 8
];

export default function TypeTreemap({ data, onTypeSelect }) {
  const getCategoryColor = (colorIndex) => {
    return categoryColors[colorIndex] || categoryColors[0];
  };

  const formatData = (treemapData) => {
    return {
      key: 'root',
      data: 'Salesforce Types',
      children: treemapData.map(category => ({
        ...category,
        children: category.children.map(child => ({
          ...child,
          fill: getCategoryColor(category.colorIndex)
        }))
      }))
    };
  };

  const handleCellClick = (event) => {
    console.log('[v0] Treemap cell clicked:', event);
    if (event.data && event.data.metadata) {
      onTypeSelect(event.data.metadata.name);
    }
  };

  if (!data || data.length === 0) {
    return (
      <Card className="bg-slate-800 border border-slate-700 p-12 text-center">
        <div className="text-gray-400">No data available for treemap</div>
      </Card>
    );
  }

  const formattedData = formatData(data);

  return (
    <Card className="bg-slate-800 border border-slate-700 p-6 animate-scale-in">
      <div className="mb-4">
        <h2 className="text-2xl font-bold text-white">Type Hierarchy</h2>
        <p className="text-gray-400 mt-1">Click any cell to explore that type</p>
      </div>
      
      <div className="h-[600px] rounded-lg overflow-hidden border border-slate-700">
        <Treemap
          data={formattedData}
          onClick={handleCellClick}
          colorScheme={categoryColors}
          label={{
            visible: true,
            fill: 'white',
            fontWeight: 600,
          }}
        />
      </div>

      <div className="mt-6 grid grid-cols-2 md:grid-cols-4 lg:grid-cols-8 gap-3">
        {data.slice(0, 8).map((category, index) => (
          <div key={category.key} className="flex items-center gap-2">
            <div 
              className="w-4 h-4 rounded flex-shrink-0" 
              style={{ backgroundColor: getCategoryColor(category.colorIndex) }}
            />
            <span className="text-sm text-gray-300 truncate">{category.key}</span>
          </div>
        ))}
      </div>
    </Card>
  );
}
