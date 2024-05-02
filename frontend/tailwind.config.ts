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
      borderWidth: {
        '6': '6px',
      },
      fontSize: {
        '2xs': ['10px', '14px'],
      },
      animation: {
        'spin-slow': 'spin 2s linear infinite',
      },
    },
  },
  plugins: [],
  darkMode: 'class',
}

export default config
