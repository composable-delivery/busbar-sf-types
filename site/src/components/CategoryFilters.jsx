import { Button } from 'reablocks';

const categoryColors = [
  'bg-gray-500',
  'bg-amber-500',
  'bg-emerald-500',
  'bg-violet-500',
  'bg-pink-500',
  'bg-teal-500',
  'bg-orange-500',
  'bg-indigo-500',
  'bg-lime-500',
];

export default function CategoryFilters({ categories, selectedCategory, onCategorySelect }) {
  return (
    <div className="animate-slide-up">
      <div className="mb-4">
        <h3 className="text-lg font-semibold text-white">Filter by Category</h3>
        <p className="text-sm text-gray-400">Select a category to filter the treemap</p>
      </div>
      
      <div className="flex gap-3 overflow-x-auto pb-4 scrollbar-thin">
        <Button
          onClick={() => onCategorySelect(null)}
          variant={selectedCategory === null ? 'filled' : 'outline'}
          className={`flex-shrink-0 ${selectedCategory === null ? 'bg-blue-600' : ''}`}
        >
          All Categories
        </Button>
        
        {categories.map((category) => {
          const isSelected = selectedCategory === category.name;
          const colorClass = categoryColors[category.colorIndex] || categoryColors[0];
          
          return (
            <Button
              key={category.name}
              onClick={() => onCategorySelect(category.name)}
              variant={isSelected ? 'filled' : 'outline'}
              className={`flex-shrink-0 ${isSelected ? colorClass : ''}`}
            >
              <div className="flex items-center gap-2">
                <span className="font-medium">{category.name}</span>
                <span className="text-xs opacity-75">({category.count})</span>
              </div>
            </Button>
          );
        })}
      </div>
    </div>
  );
}
