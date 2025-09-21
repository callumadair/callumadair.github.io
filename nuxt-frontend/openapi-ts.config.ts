import { defineConfig } from "@hey-api/openapi-ts";

export default defineConfig({
	input: "http://localhost.:5000/api/openapi.json",
	output: "src/client",
});
