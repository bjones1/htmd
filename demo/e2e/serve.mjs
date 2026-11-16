import { createReadStream } from 'node:fs';
import { access, stat } from 'node:fs/promises';
import { createServer } from 'node:http';
import { dirname, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';
import { publicPath } from './public-path.mjs';

const dist = resolve(dirname(fileURLToPath(import.meta.url)), '../dist');
await access(resolve(dist, 'index.html')).catch(() => {
  throw new Error('Build the release demo first: trunk build --release');
});

const contentTypes = {
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.wasm': 'application/wasm',
  '.css': 'text/css; charset=utf-8',
  '.json': 'application/json',
  '.ico': 'image/x-icon',
};

createServer(async (request, response) => {
  try {
    const pathname = decodeURIComponent(new URL(request.url, 'http://localhost').pathname);
    if (!pathname.startsWith(publicPath)) {
      response.writeHead(404).end();
      return;
    }
    const filename = resolve(dist, pathname.slice(publicPath.length) || 'index.html');
    if (!filename.startsWith(dist + sep) || !(await stat(filename)).isFile()) {
      response.writeHead(404).end();
      return;
    }
    const extension = filename.slice(filename.lastIndexOf('.'));
    response.writeHead(200, {
      'Content-Type': contentTypes[extension] ?? 'application/octet-stream',
      'Cache-Control': 'no-store',
    });
    createReadStream(filename).pipe(response);
  } catch {
    response.writeHead(404).end();
  }
}).listen(4173, '127.0.0.1', () => {
  console.log(`Serving ${dist} at http://127.0.0.1:4173${publicPath}`);
});
