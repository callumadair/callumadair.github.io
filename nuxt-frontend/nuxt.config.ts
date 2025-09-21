import tailwindcss from "@tailwindcss/vite";
import Aura from "@primevue/themes/aura";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
	compatibilityDate: "2025-07-15",
	devtools: { enabled: true },
	modules: ["@nuxt/test-utils", "@primevue/nuxt-module"],
	css: ["~/assets/css/main.css"],
	vite: {
		plugins: [tailwindcss()],
	},
	primevue: {
		options: {
			theme: {
				preset: Aura,
			},
		},
	},
});
