const plugin = require("tailwindcss/plugin");

module.exports = {
  darkMode: "class",
  content: ["./public/index.html", "./src/**/*.svelte"],
  theme: {
    extend: {},
  },
  plugins: [
    require("daisyui"),
    function ({ addVariant }) {
      addVariant("children", "& > *");
    },
    plugin(
      function ({ matchUtilities, theme }) {
        matchUtilities(
          {
            contain: (value) => ({ contain: value }),
          },
          {
            values: theme("contain", {}),
          }
        );
      },
      {
        theme: {
          contain: {
            none: "none",
            strict: "strict",
            content: "content",
            size: "size",
            layout: "layout",
            style: "style",
            paint: "paint",
            unset: "unset",
          },
        },
      }
    ),
  ],
};
