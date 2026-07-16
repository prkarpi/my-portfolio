/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html"
  ],
  theme: {
    extend: {
      fontSize: {
        // 1. Core Metadata / Skill Tags (Originally 12px -> Now 15px)
        'xs': '0.9375rem',  

        // 2. Job bullets and matrix descriptions (Originally 14px -> Now 18px)
        'sm': '1.125rem',   

        // 3. Main Job Descriptions & Text Paragraphs (Originally 16px -> Now 20px)
        'base': '1.25rem',  

        // 4. Academic subheaders & "Alex Vassiliev" Logo (Originally 18px -> Now 22px)
        'lg': '1.375rem',   

        // 5. Main Hero Paragraph Desktop View (Originally 20px -> Now 26px)
        'xl': '1.625rem',   

        // 6. Selected Job Title Headers (Originally 30px -> Now 38px)
        '3xl': '2.375rem',  

        // 7. Hero Title Mobile State (Originally 48px -> Now 60px)
        '5xl': '3.75rem',   
      },
    },
  },
  plugins: [],
}