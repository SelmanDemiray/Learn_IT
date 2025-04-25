module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
    "./*.tsx",
    "./components/**/*.tsx",
    "./pages/**/*.tsx",
    "./data/**/*.tsx",
    "./data/**/*.ts"
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        primary: {
          light: '#60a5fa',
          DEFAULT: '#2563eb',
          dark: '#1e40af'
        },
        accent: '#fbbf24',
        success: '#22c55e',
        info: '#38bdf8',
        warning: '#f59e42',
        danger: '#ef4444'
      },
      fontFamily: {
        display: ['"Poppins"', 'sans-serif'],
        body: ['"Inter"', 'sans-serif']
      },
      boxShadow: {
        'xl-blue': '0 10px 40px 0 rgba(37,99,235,0.15)'
      }
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
    require('@tailwindcss/forms')
  ],
}
