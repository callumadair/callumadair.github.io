import tailwindcss from "@tailwindcss/vite";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
	compatibilityDate: "2025-07-15",
	modules: [
		"@nuxt/test-utils",
		"@nuxt/ui",
		"@nuxt/icon"
	],
	css: ["./app/assets/css/main.css"],
	vite: {
		plugins: [tailwindcss()],
	},
	devtools: {
		enabled: true,
		vscode: {
			codeServer: "ms-code-server",
			host: "0.0.0.0",
			port: 3090
		}
	},
	ui: {
		theme: {
			colors: ["primary", "secondary", "tertiary", "quarternary", "quinternary", "light-monochrome", "medium-monochrome", "dark-monochrome", "success", "error", "info", "warning", "neutral", "neutralHighlight"]
		}
	}
});