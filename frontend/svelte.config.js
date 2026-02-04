import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter({
			pages: '../backend/dist',
			assets: '../backend/dist',
			fallback: 'index.html'
		}),
		alias: {
			// "@/*": "./path/to/lib/*",
		}
	}
};

export default config;
