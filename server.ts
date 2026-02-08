import { serve } from 'bun';

const port = 8080;
const PUBLIC_DIR = './public';

serve({
  port,
  async fetch(req) {
    const url = new URL(req.url);
    let path = url.pathname;

    // Default to index.html for root path
    if (path === '/') {
      path = '/index.html';
    }

    // Try to serve the file from public directory
    try {
      const file = Bun.file(`${PUBLIC_DIR}${path}`);
      return new Response(file);
    } catch {
      return new Response('Not Found', { status: 404 });
    }
  },
});

console.log(`✓ Server running on http://localhost:${port}`);
console.log(`✓ Serving files from: ${PUBLIC_DIR}`);
