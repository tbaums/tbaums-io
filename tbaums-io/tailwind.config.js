/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      colors: {
        'light-pink': '#FFC0CB',
        'hot-pink': '#FF69B4',
        'fuchsia': '#FF00FF',
        'lavender': '#E6E6FA',
        'orchid': '#DA70D6',
        'medium-purple': '#9370DB',
        'dark-violet': '#9400D3',
        'deep-pink': '#FF1493',
        'pale-violet-red': '#DB7093',
        'black-violet': '#1A001A',
        'plum': '#DDA0DD',
      },
    },
  },
  plugins: [],
};
