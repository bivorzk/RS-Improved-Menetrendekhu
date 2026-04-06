module.exports = {
  content: [
    "./src/**/*.{rs,html}",
    "./index.html",
  ],
  theme: {
    extend: {
      colors: {
        transit: {
          50: '#eff8ff',
          100: '#dbeafe',
          200: '#bfdbfe',
          500: '#0ea5e9',
          700: '#0369a1',
          900: '#0f172a',
        },
      },
      boxShadow: {
        soft: '0 24px 80px rgba(15, 23, 42, 0.08)',
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
      },
    },
  },
  plugins: [],
};
