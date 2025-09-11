import { defineConfig, } from '@rsbuild/core';
import { pluginReact } from '@rsbuild/plugin-react';
import { pluginSvgr } from '@rsbuild/plugin-svgr';

const host = process.env.TAURI_DEV_HOST;
export default defineConfig({
    html: {
        template: './public/index.html',
    },
    output: {
        assetPrefix: './',
    },
    server: {
        host: host || false,
        port: 1420,
        strictPort: true,
    },
    plugins: [
        pluginReact(),
        pluginSvgr({ mixedImport: true }),
    ],
    resolve: {
        alias: {
            '@': './src/',
        }
    },
});
