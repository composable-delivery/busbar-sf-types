import { Card } from 'reablocks';
import { useEffect, useState } from 'react';

function AnimatedCounter({ value, duration = 2000 }) {
  const [count, setCount] = useState(0);

  useEffect(() => {
    const startTime = Date.now();
    const endValue = parseInt(value);
    
    const animate = () => {
      const now = Date.now();
      const progress = Math.min((now - startTime) / duration, 1);
      const currentCount = Math.floor(progress * endValue);
      
      setCount(currentCount);
      
      if (progress < 1) {
        requestAnimationFrame(animate);
      }
    };
    
    animate();
  }, [value, duration]);

  return count.toLocaleString();
}

export default function StatsBar({ stats }) {
  const { totalTypes, totalDependencies, totalCategories, relationshipStats } = stats;

  return (
    <div className="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-6 gap-4 animate-scale-in">
      <Card className="bg-slate-800 text-center p-6 border border-slate-700 hover:border-blue-500 transition-all">
        <div className="text-4xl lg:text-5xl font-bold text-blue-400 mb-2">
          <AnimatedCounter value={totalTypes} />
        </div>
        <div className="text-sm text-gray-400 uppercase tracking-wide">Total Types</div>
      </Card>

      <Card className="bg-slate-800 text-center p-6 border border-slate-700 hover:border-green-500 transition-all">
        <div className="text-4xl lg:text-5xl font-bold text-green-400 mb-2">
          <AnimatedCounter value={totalDependencies} />
        </div>
        <div className="text-sm text-gray-400 uppercase tracking-wide">Dependencies</div>
      </Card>

      <Card className="bg-slate-800 text-center p-6 border border-slate-700 hover:border-purple-500 transition-all">
        <div className="text-4xl lg:text-5xl font-bold text-purple-400 mb-2">
          <AnimatedCounter value={totalCategories} />
        </div>
        <div className="text-sm text-gray-400 uppercase tracking-wide">Categories</div>
      </Card>

      <Card className="bg-slate-800 text-center p-6 border border-slate-700 hover:border-blue-500 transition-all">
        <div className="text-4xl lg:text-5xl font-bold text-blue-400 mb-2">
          <AnimatedCounter value={relationshipStats.Contains || 0} />
        </div>
        <div className="text-sm text-gray-400 uppercase tracking-wide">Contains</div>
      </Card>

      <Card className="bg-slate-800 text-center p-6 border border-slate-700 hover:border-purple-500 transition-all">
        <div className="text-4xl lg:text-5xl font-bold text-purple-400 mb-2">
          <AnimatedCounter value={relationshipStats.Extends || 0} />
        </div>
        <div className="text-sm text-gray-400 uppercase tracking-wide">Extends</div>
      </Card>

      <Card className="bg-slate-800 text-center p-6 border border-slate-700 hover:border-orange-500 transition-all">
        <div className="text-4xl lg:text-5xl font-bold text-orange-400 mb-2">
          <AnimatedCounter value={relationshipStats.Generic || 0} />
        </div>
        <div className="text-sm text-gray-400 uppercase tracking-wide">Generic</div>
      </Card>
    </div>
  );
}
