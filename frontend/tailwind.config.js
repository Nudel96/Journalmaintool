/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        bg: {
          DEFAULT: '#0d1b2a',
          2: '#2e2e2e',
        },
        brand: '#1b9aaa',
        accent: '#7cfc00',
        success: '#7cfc00',
        danger: '#ef4444',
        text: {
          DEFAULT: '#ffffff',
          muted: '#cfd6dd',
        },
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'Segoe UI', 'Roboto', 'Ubuntu', 'Helvetica Neue', 'Arial', 'Noto Sans', 'sans-serif'],
      },
      borderRadius: {
        card: '24px',
        button: '12px',
        input: '14px',
        pill: '999px',
      },
      boxShadow: {
        card: '0 10px 40px rgba(0,0,0,.55)',
        ring: '0 0 0 4px rgba(27, 154, 170, .45)',
      },
    },
  },
  plugins: [],
}

