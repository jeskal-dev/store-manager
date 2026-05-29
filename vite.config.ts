import tailwindcss from "@tailwindcss/vite";
import { devtools } from "@tanstack/devtools-vite";
import { tanstackStart } from "@tanstack/react-start/plugin/vite";
import viteReact from "@vitejs/plugin-react";
import { defineConfig } from "vite";

type TanStackStartInputConfig = NonNullable<Parameters<typeof tanstackStart>[0]>;
type SpaOptions = NonNullable<TanStackStartInputConfig["spa"]>;
type SpaPrerenderOptions = NonNullable<SpaOptions["prerender"]>;
type RegularPrerenderOptions = NonNullable<SpaOptions["prerender"]>;

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// @ts-expect-error process is a nodejs global
const useSsrPrerenderString: string = process.env.USE_SSR_PRERENDER_MODE?.toLowerCase() ?? "false";
const useSsrPrerenderMode: boolean =
  useSsrPrerenderString === "true" || useSsrPrerenderString === "1";

const sharedPrerenderOptions: SpaPrerenderOptions & RegularPrerenderOptions = {
  enabled: true,
  autoSubfolderIndex: true,
};

const regularPrerenderOptions: RegularPrerenderOptions = {
  ...sharedPrerenderOptions,
  // Whether to extract links from the HTML and prerender them also
  // See: https://tanstack.com/start/latest/docs/framework/react/guide/static-prerendering#crawling-links
  crawlLinks: true,
  // Number of times to retry a failed prerender job
  retryCount: 3,
  // Delay between retries in milliseconds
  retryDelay: 1000,
};

const spaWithPrerenderOptions: SpaOptions = {
  prerender: {
    ...sharedPrerenderOptions,
    // Change the root output path for SPA prerendering from /_shell.html to /index.html
    outputPath: "/index.html",
    crawlLinks: false,
    retryCount: 0,
  },
};

// https://vite.dev/config/
export default defineConfig(async () => ({
  resolve: { tsconfigPaths: true },
  plugins: [
    devtools(),
    tailwindcss(),
    tanstackStart({
      spa: (!useSsrPrerenderMode ? spaWithPrerenderOptions : undefined) satisfies
        | SpaOptions
        | undefined,
      prerender: (useSsrPrerenderMode ? regularPrerenderOptions : undefined) satisfies
        | RegularPrerenderOptions
        | undefined,
    }),
    viteReact(),
  ],
  
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
