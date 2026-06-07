import React from 'react';
import ReactDOM from 'react-dom/client';
import { ThemeProvider } from 'reablocks';
import App from './App';
import './styles.css';

const customTheme = {
  colors: {
    primary: '#3b82f6',
    secondary: '#64748b',
    surface: '#1e293b',
    background: '#0f172a',
  }
};

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <ThemeProvider theme={customTheme}>
      <App />
    </ThemeProvider>
  </React.StrictMode>
);
