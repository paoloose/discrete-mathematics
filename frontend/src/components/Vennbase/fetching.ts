export const BACKEND_ADDR = 'http://127.0.0.1:8000';

export async function vennfetch(endpoint: string, method: string = 'GET') {
  const response = await fetch(`${BACKEND_ADDR}${endpoint}`, {
    method,
    headers: {
      'Accept': 'application/json',
    }
  }).then(response => {
    if (response.status < 200 || response.status >= 300) {
      throw new Error(response.statusText);
    }
    return response;
  });
  return response;
}
