import { parse } from 'node-html-parser';
import { appendRecord, downloadLocally, notUndefined, randomDelay } from './utils';

const pages = 3;
// const pageURL = (num: number) => `https://wallhaven.cc/random?seed=${Math.random()}?page=${num}`;
// const pageURL = (num: number) => `https://wallhaven.cc/search?q=${"my+melody"}&page=${num}`;
const pageURL = (num: number) => `https://wallhaven.cc/search?q=id%3A5&page=${num}`;

async function fetchDetail(url: string) {
  const detailResponse = await fetch(url);
  if (!detailResponse.ok) {
    console.error(detailResponse);
    throw new Error(detailResponse.statusText);
  };
  const detail = await detailResponse.text();
  return parse(detail);
}

async function fetchPage(num: number) {
  const response = await fetch(pageURL(num));
  const html = await response.text();
  const dom = parse(html);

  const urls = dom.querySelectorAll('.preview')
    .map((preview) => preview.getAttribute('href'))
    .filter(notUndefined);

  for await (const url of urls) {
    const detail = await fetchDetail(url);
    const tags = detail.querySelectorAll('li.tag').map((tag) => tag.textContent);
    const downloadURL = detail.querySelector('img#wallpaper').getAttribute('src');
    console.log(downloadURL);
    const { filename, mimetype } = await downloadLocally(downloadURL);

    await appendRecord({
      filename,
      mimetype,
      tags
    })

    console.log({ tags });
    await randomDelay();
  }
}

async function main() {
  for (let i = 1; i <= pages; i++) {
    await fetchPage(i);
  }
}

main();
