import { join } from 'path';
import { OUTPUT_DIR } from './config';

export function randomDelay() {
  return new Promise((resolve) => {
    setTimeout(resolve, (Math.random() + 0.5) * 2000);
  });
}

export function notUndefined<T>(value: T | undefined): value is T {
  return value !== undefined;
}

type Record = {
  filename: string;
  tags: string[];
  mimetype: string;
};

export async function appendRecord(record: Record) {
  const records = Bun.file(join(OUTPUT_DIR, 'records.json'));
  const data = await (records.exists() ? records.json() : []) as Record[];
  data.push(record);
  const writer = records.writer();
  writer.write(JSON.stringify(data, null, 2));
  writer.flush();
}

export async function downloadLocally(imageurl: string) {
  const response = await fetch(imageurl);

  if (!response.ok) {
    console.log(response);
    throw new Error('Failed to fetch image');
  }

  const mimetype = response.headers.get('content-type');

  if (!!!mimetype.includes('image')) {
    throw new Error('Not a png image');
  }

  const filename = imageurl.split('/').pop();
  const blob = await response.blob();

  await Bun.write(join(OUTPUT_DIR, filename), blob);

  return {
    filename,
    mimetype
  };
}
