import react from "@astrojs/react";
import { defineConfig } from 'astro/config';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

// https://astro.build/config
export default defineConfig({
  site: 'https://paoloose.site',
  base: '/discrete-mathematics',
  integrations: [react()],
  vite: {
    plugins: [wasm(), topLevelAwait()]
  }
});
