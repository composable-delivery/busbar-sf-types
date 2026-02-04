import { Card, Button, Input } from 'reablocks';
import { useState, useMemo } from 'react';

export default function TypeTable({ types, onTypeSelect, showCategory = true }) {
  const [sortField, setSortField] = useState('connectionCount');
  const [sortDirection, setSortDirection] = useState('desc');
  const [filterText, setFilterText] = useState('');

  const sortedAndFiltered = useMemo(() => {
    let filtered = types;
    
    if (filterText) {
      const lower = filterText.toLowerCase();
      filtered = types.filter(t => t.name.toLowerCase().includes(lower));
    }

    return [...filtered].sort((a, b) => {
      let aVal = a[sortField];
      let bVal = b[sortField];
      
      if (sortField === 'category') {
        aVal = a.category?.name || 'uncategorized';
        bVal = b.category?.name || 'uncategorized';
      }
      
      if (sortField === 'name') {
        return sortDirection === 'asc' 
          ? aVal.localeCompare(bVal)
          : bVal.localeCompare(aVal);
      }
      
      return sortDirection === 'asc' ? aVal - bVal : bVal - aVal;
    });
  }, [types, sortField, sortDirection, filterText]);

  const handleSort = (field) => {
    if (sortField === field) {
      setSortDirection(sortDirection === 'asc' ? 'desc' : 'asc');
    } else {
      setSortField(field);
      setSortDirection('desc');
    }
  };

  const SortIcon = ({ field }) => {
    if (sortField !== field) return null;
    return sortDirection === 'asc' ? ' ↑' : ' ↓';
  };

  return (
    <Card className="bg-slate-800 border border-slate-700 p-6">
      <div className="mb-4">
        <h3 className="text-xl font-bold text-white mb-3">Type Explorer</h3>
        <Input
          placeholder="Filter types..."
          value={filterText}
          onChange={(e) => setFilterText(e.target.value)}
          className="max-w-md"
        />
      </div>

      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead className="border-b border-slate-700">
            <tr className="text-left text-gray-400">
              <th 
                className="py-3 px-4 font-semibold cursor-pointer hover:text-white transition-colors"
                onClick={() => handleSort('name')}
              >
                Type Name<SortIcon field="name" />
              </th>
              {showCategory && (
                <th 
                  className="py-3 px-4 font-semibold cursor-pointer hover:text-white transition-colors"
                  onClick={() => handleSort('category')}
                >
                  Category<SortIcon field="category" />
                </th>
              )}
              <th 
                className="py-3 px-4 font-semibold cursor-pointer hover:text-white transition-colors text-right"
                onClick={() => handleSort('connectionCount')}
              >
                Connections<SortIcon field="connectionCount" />
              </th>
              <th className="py-3 px-4 font-semibold text-right">
                Actions
              </th>
            </tr>
          </thead>
          <tbody>
            {sortedAndFiltered.map((type) => (
              <tr 
                key={type.name}
                className="border-b border-slate-700 hover:bg-slate-700/50 transition-colors"
              >
                <td className="py-3 px-4 font-medium text-white">
                  {type.name}
                </td>
                {showCategory && (
                  <td className="py-3 px-4 text-gray-300">
                    <span className="inline-flex items-center px-2 py-1 rounded text-xs bg-slate-700">
                      {type.category?.name || 'uncategorized'}
                    </span>
                  </td>
                )}
                <td className="py-3 px-4 text-right text-gray-300">
                  {type.connectionCount}
                </td>
                <td className="py-3 px-4 text-right">
                  <Button
                    size="small"
                    variant="outline"
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
        <div className="text-center py-8 text-gray-400">
          No types found matching "{filterText}"
        </div>
      )}

      <div className="mt-4 text-sm text-gray-400">
        Showing {sortedAndFiltered.length} of {types.length} types
      </div>
    </Card>
  );
}
