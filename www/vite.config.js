import { defineConfig } from 'vite'
import topLevelAwait from "vite-plugin-top-level-await";
import vue from "@vitejs/plugin-vue";
import wasm from "vite-plugin-wasm";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    topLevelAwait(),
    vue(),
    wasm()
  ],
  resolve: {
    alias: {
      '@': '/src'
    }
  }
})
