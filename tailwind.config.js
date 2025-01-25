/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./index.html", "./src/**/*.rs", "./components/src/**/*.rs"],
    theme: {
        extend: {},
    },
    plugins: [
        require('daisyui'),
        require('@tailwindcss/forms')({
            strategy: 'base',
            strategy: 'class',
        }),
        require('@tailwindcss/typography'),
        require('@tailwindcss/container-queries'),
        require('tailwindcss-animate'),
        require('tailwindcss-opentype'),
    ],

}

