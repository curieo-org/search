import type { Config } from 'tailwindcss'
import { colors } from './src/styles/colors'

const config: Config = {
  content: [
    './src/pages/**/*.{js,ts,jsx,tsx,mdx}',
    './src/components/**/*.{js,ts,jsx,tsx,mdx}',
    './src/app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      colors: {
        ...colors,
      },
      width: {
        '90': '22.5rem',
      },
      borderWidth: {
        '6': '6px',
      },
      borderRadius: {
        '2.5xl': '20px',
      },
      fontSize: {
        '2xs': ['10px', '14px'],
      },
      opacity: {
        2: '0.02',
        3: '0.03',
        4: '0.04',
        25: '0.25',
        75: '0.75',
      },
      dropShadow: {
        xs: ['0 4px 10px rgba(0, 0, 0, 0.25)', '0 0 2px rgba(109, 58, 230, 0.7)'],
      },
      backgroundImage: {
        'gradient-dark': 'linear-gradient(to top, rgba(0, 0, 0, 0.1), rgba(47, 47, 47, 0.1))',
        'gradient-source-card': 'linear-gradient(to bottom, rgba(31, 31, 31, 0.05), rgba(255, 255, 255, 0.02))',
      },
      animation: {
        'spin-once': 'spin 1s linear',
        'spin-once-reverse': 'spin-reverse 1s linear',
        'spin-slow': 'spin 2s linear infinite',
      },

      keyframes: {
        'spin-reverse': {
          from: { transform: 'rotate(360deg)' },
          to: { transform: 'rotate(0deg)' },
        },
      },
    },
  },
  plugins: [],
  darkMode: 'class',
}

export default config
