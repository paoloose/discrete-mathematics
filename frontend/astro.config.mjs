import preact from '@astrojs/preact';
import { defineConfig } from 'astro/config';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';


// https://astro.build/config
export default defineConfig({
  site: 'https://paoloose.site',
  base: '/discrete-mathematics',
  integrations: [preact()],
  vite: {
    plugins: [wasm(), topLevelAwait()]
  }
});
