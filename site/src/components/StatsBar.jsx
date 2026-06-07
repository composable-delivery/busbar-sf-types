import { Card } from "reablocks";
import { useEffect, useState } from "react";

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
    const { totalTypes, totalDependencies, totalCategories } = stats;

    return (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 animate-scale-in">
            <Card className="bg-black text-center p-8 border border-white/10 hover:border-white/30 transition-all rounded-2xl shadow-2xl relative overflow-hidden group">
                <div className="absolute inset-0 bg-blue-500/5 translate-y-full group-hover:translate-y-0 transition-transform duration-500" />
                <div className="relative z-10">
                    <div className="text-5xl lg:text-7xl font-black text-white mb-2 tracking-tighter">
                        <AnimatedCounter value={totalTypes} />
                    </div>
                    <div className="text-xs text-gray-500 uppercase tracking-[0.2em] font-black">
                        Total Types
                    </div>
                </div>
            </Card>

            <Card className="bg-black text-center p-8 border border-white/10 hover:border-white/30 transition-all rounded-2xl shadow-2xl relative overflow-hidden group">
                <div className="absolute inset-0 bg-indigo-500/5 translate-y-full group-hover:translate-y-0 transition-transform duration-500" />
                <div className="relative z-10">
                    <div className="text-5xl lg:text-7xl font-black text-white mb-2 tracking-tighter">
                        <AnimatedCounter value={totalDependencies} />
                    </div>
                    <div className="text-xs text-gray-500 uppercase tracking-[0.2em] font-black">
                        Total Dependencies
                    </div>
                </div>
            </Card>

            <Card className="bg-black text-center p-8 border border-white/10 hover:border-white/30 transition-all rounded-2xl shadow-2xl relative overflow-hidden group">
                <div className="absolute inset-0 bg-purple-500/5 translate-y-full group-hover:translate-y-0 transition-transform duration-500" />
                <div className="relative z-10">
                    <div className="text-5xl lg:text-7xl font-black text-white mb-2 tracking-tighter">
                        <AnimatedCounter value={totalCategories} />
                    </div>
                    <div className="text-xs text-gray-500 uppercase tracking-[0.2em] font-black">
                        Categories Explorer
                    </div>
                </div>
            </Card>
        </div>
    );
}
