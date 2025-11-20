import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import Icons from 'unplugin-icons/vite';
import { enhancedImages } from '@sveltejs/enhanced-img';

export default defineConfig({
    css: {
        preprocessorOptions: {
            scss: {
                api: 'modern-compiler' // or "modern"
            }
        }
    },
    plugins: [
        sveltekit(),
        Icons({
            compiler: 'svelte'
        }),
        enhancedImages(),
    ],
    server: {
        proxy: {
            '/api': {
                target: 'http://0.0.0.0:2020',
                changeOrigin: true,
                rewrite: path => path.replace(/^\/api/, '')
            }

        },
    }
});
