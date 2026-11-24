import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  // a page under a path (GitHub Pages: /chimera-pt/) builds with CHIMERA_BASE=/chimera-pt/
  base: process.env.CHIMERA_BASE ?? "/",
  plugins: [react()],
  worker: { format: "es" },
  // three.js loads only when 3D is first ticked: pre-bundle it, or the dev server re-optimizes
  // then and the lazy import fails ("504 Outdated Optimize Dep", a black page)
  optimizeDeps: {
    include: [
      "three",
      "three/addons/controls/OrbitControls.js",
      "three/addons/lines/LineMaterial.js",
      "three/addons/lines/LineSegments2.js",
      "three/addons/lines/LineSegmentsGeometry.js",
      "three/addons/postprocessing/EffectComposer.js",
      "three/addons/postprocessing/OutputPass.js",
      "three/addons/postprocessing/RenderPass.js",
      "three/addons/postprocessing/UnrealBloomPass.js",
    ],
  },
  build: { target: "es2022", sourcemap: true },
  server: { host: "127.0.0.1", port: 5317, strictPort: true, allowedHosts: true },
  preview: { host: "127.0.0.1", port: 5318, strictPort: true, allowedHosts: true },
});
