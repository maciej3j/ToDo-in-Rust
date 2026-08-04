import { build } from "esbuild";
import cssModulesPlugin from "esbuild-css-modules-plugin";

build({
  plugins: [cssModulesPlugin()],
  entryPoints: ["src/index.tsx"],
  bundle: true,
  outfile: "public/bundle.js",
  format: "esm",
  define: {
    "process.env.NODE_ENV": '"production"',
  },
  minify: true,
  sourcemap: true,
  loader: {
    ".js": "jsx",
    ".tsx": "tsx",
    ".ts": "ts",
    ".wasm": "binary",
    ".css": "css",
  },
}).catch(() => process.exit(1));
