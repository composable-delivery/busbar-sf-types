import { Input, Button } from 'reablocks';

export default function Hero({ searchQuery, onSearchChange, onSearch }) {
  const handleKeyPress = (e) => {
    if (e.key === 'Enter') {
      onSearch();
    }
  };

  return (
    <div className="relative bg-gradient-to-br from-blue-600 via-blue-500 to-blue-700 text-white animate-fade-in">
      <div className="absolute inset-0 bg-[url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNjAiIGhlaWdodD0iNjAiIHZpZXdCb3g9IjAgMCA2MCA2MCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48ZyBmaWxsPSJub25lIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiPjxnIGZpbGw9IiNmZmYiIGZpbGwtb3BhY2l0eT0iMC4wNSI+PHBhdGggZD0iTTM2IDE zNGg4djhoLTh6TTI4IDM0aDh2OGgtOHpNMjggMjZoOHY4aC04ek0yMCAyNmg4djhoLTh6TTIwIDE4aDh2OGgtOHoiLz48L2c+PC9nPjwvc3ZnPg==')] opacity-30"></div>
      
      <div className="relative max-w-7xl mx-auto px-6 py-16 lg:py-24">
        <div className="text-center space-y-6 animate-slide-up">
          <h1 className="text-5xl lg:text-7xl font-bold tracking-tight">
            Salesforce Types Explorer
          </h1>
          <p className="text-xl lg:text-2xl text-blue-100 max-w-3xl mx-auto font-medium">
            Interactive visualization of Salesforce metadata type dependencies
          </p>
          
          <div className="max-w-2xl mx-auto mt-8">
            <div className="flex gap-3">
              <div className="flex-1">
                <Input
                  placeholder="Search for a type..."
                  value={searchQuery}
                  onChange={(e) => onSearchChange(e.target.value)}
                  onKeyPress={handleKeyPress}
                  className="w-full"
                  size="large"
                />
              </div>
              <Button
                onClick={onSearch}
                className="bg-white text-blue-600 hover:bg-blue-50 font-semibold px-8"
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
