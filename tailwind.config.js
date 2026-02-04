/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./site/index.html",
    "./site/src/**/*.{js,ts,jsx,tsx}",
    "./node_modules/reablocks/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        primary: '#3b82f6',
        secondary: '#64748b',
        surface: '#1e293b',
        background: '#0f172a',
        category: {
          1: '#3b82f6',
          2: '#8b5cf6',
          3: '#ec4899',
          4: '#f59e0b',
          5: '#10b981',
          6: '#06b6d4',
          7: '#f97316',
          8: '#a855f7',
        },
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
      },
      animation: {
        'fade-in': 'fadeIn 0.5s ease-in',
        'slide-up': 'slideUp 0.6s ease-out',
        'scale-in': 'scaleIn 0.4s ease-out',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(20px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        scaleIn: {
          '0%': { transform: 'scale(0.95)', opacity: '0' },
          '100%': { transform: 'scale(1)', opacity: '1' },
        },
      },
    },
  },
  plugins: [],
}
