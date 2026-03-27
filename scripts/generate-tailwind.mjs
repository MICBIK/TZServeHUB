import { compile } from 'tailwindcss';
import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const ROOT = path.resolve(__dirname, '..');
const SRC_DIR = path.join(ROOT, 'src');
const OUTPUT_DIR = path.join(SRC_DIR, 'generated');
const OUTPUT_FILE = path.join(OUTPUT_DIR, 'tailwind.css');

const TEXT_TOKEN = /[A-Za-z0-9-_:./[\]%#(),]+/g;

async function walk(dir) {
  const entries = await fs.readdir(dir, { withFileTypes: true });
  const files = [];

  for (const entry of entries) {
    const fullPath = path.join(dir, entry.name);

    if (entry.isDirectory()) {
      if (entry.name === 'generated') {
        continue;
      }
      files.push(...await walk(fullPath));
      continue;
    }

    if (/\.(css|html|ts|tsx)$/.test(entry.name)) {
      files.push(fullPath);
    }
  }

  return files;
}

function extractCandidates(content) {
  return content.match(TEXT_TOKEN) ?? [];
}

async function loadStylesheet(id, from) {
  const resolved =
    id === 'tailwindcss'
      ? path.join(ROOT, 'node_modules', 'tailwindcss', 'index.css')
      : path.resolve(path.dirname(from), id);

  return {
    path: resolved,
    base: path.dirname(resolved),
    content: await fs.readFile(resolved, 'utf8'),
  };
}

async function main() {
  const files = await walk(SRC_DIR);
  const candidates = new Set([
    'dark',
    'light',
  ]);

  for (const file of files) {
    const content = await fs.readFile(file, 'utf8');
    for (const candidate of extractCandidates(content)) {
      candidates.add(candidate);
    }
  }

  const compiled = await compile('@import "tailwindcss";', {
    base: ROOT,
    from: path.join(SRC_DIR, 'index.css'),
    loadStylesheet,
  });

  const css = compiled.build([...candidates]);

  await fs.mkdir(OUTPUT_DIR, { recursive: true });
  await fs.writeFile(OUTPUT_FILE, `${css}\n`, 'utf8');
  console.log(`Generated ${path.relative(ROOT, OUTPUT_FILE)} with ${candidates.size} candidates`);
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
