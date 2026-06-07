import { Input, Button } from "reablocks";

export default function Hero({ searchQuery, onSearchChange, onSearch }) {
    const handleKeyPress = (e) => {
        if (e.key === "Enter") {
            onSearch();
        }
    };

    return (
        <div className="relative bg-black text-white animate-fade-in border-b border-white/10">
            <div className="absolute inset-0 bg-[radial-gradient(circle_at_center,_var(--tw-gradient-stops))] from-blue-900/10 via-transparent to-transparent opacity-50"></div>

            <div className="relative max-w-7xl mx-auto px-6 py-16 lg:py-24">
                <div className="text-center space-y-6 animate-slide-up">
                    <h1 className="text-5xl lg:text-7xl font-bold tracking-tight text-white mb-2">
                        Salesforce Types
                    </h1>
                    <p className="text-xl lg:text-2xl text-gray-400 max-w-3xl mx-auto font-light tracking-wide">
                        Interactive visualization of Salesforce metadata type
                        dependencies
                    </p>

                    <div className="max-w-xl mx-auto mt-10">
                        <div className="flex gap-2 p-1 bg-white/5 rounded-xl border border-white/10 shadow-2xl">
                            <div className="flex-1">
                                <Input
                                    placeholder="Search types..."
                                    value={searchQuery}
                                    onChange={(e) =>
                                        onSearchChange(e.target.value)
                                    }
                                    onKeyPress={handleKeyPress}
                                    className="w-full bg-transparent border-none text-white focus:ring-0 placeholder:text-gray-500"
                                    size="large"
                                />
                            </div>
                            <Button
                                onClick={onSearch}
                                className="bg-white text-black hover:bg-gray-200 font-bold px-8 rounded-lg transition-all"
                            >
                                Search
                            </Button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}
