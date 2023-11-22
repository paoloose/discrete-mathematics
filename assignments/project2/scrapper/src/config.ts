import { mkdirSync, existsSync, writeFileSync } from 'fs';
import { join } from 'path';

export const OUTPUT_DIR = 'scrapped-data';
export const JSON_FILE = 'records.json';

mkdirSync(OUTPUT_DIR, { recursive: true });

if (!existsSync(join(OUTPUT_DIR, JSON_FILE))) {
  writeFileSync(join(OUTPUT_DIR, JSON_FILE), '[]');
}
