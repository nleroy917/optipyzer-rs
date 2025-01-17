import initSqlJs, { BindParams, Database } from 'sql.js';
import { getCachedDb, openIDB, setCachedDb } from '@/lib/indexeddb';

let sqliteInstance: Database | null = null;

function fetchWithProgress(url: string, onProgress: (fraction: number) => void): Promise<ArrayBuffer> {
  return fetch(url).then(async (response) => {
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
    const contentLength = parseInt(response.headers.get('Content-Length') || '0', 10);
    if (!response.body || !contentLength) {
      // Fallback if no content length or streaming body
      return response.arrayBuffer();
    }

    const reader = response.body.getReader();
    let received = 0;
    const chunks: Uint8Array[] = [];

    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      chunks.push(value);
      received += value.length;
      onProgress(received / contentLength);
    }

    // Merge into a single buffer
    const data = new Uint8Array(received);
    let pos = 0;
    for (const chunk of chunks) {
      data.set(chunk, pos);
      pos += chunk.length;
    }
    return data.buffer;
  });
}

export async function fetchDbAndInit(dbUrl: string, onProgress?: (fraction: number) => void): Promise<Database> {
  if (sqliteInstance) return sqliteInstance;

  const sqlPromise = initSqlJs({
    locateFile: (file: string) => `https://sql.js.org/dist/${file}`,
  });

  // 1) Try IndexedDB first
  const dbHandle = await openIDB();
  const cached = await getCachedDb(dbHandle, 'codonDb');
  let dbArrayBuffer: ArrayBuffer;

  if (cached) {
    console.log('Using cached database');
    // If found, use that
    dbArrayBuffer = cached;
    onProgress?.(1);
  } else {
    console.log('Fetching database');
    // 2) Otherwise fetch from network
    dbArrayBuffer = await fetchWithProgress(dbUrl, (fraction) => onProgress?.(fraction));
    // 3) Store it in IndexedDB for next time
    await setCachedDb(dbHandle, 'codonDb', dbArrayBuffer);
  }

  // Init SQL.js
  const [SQL] = await Promise.all([sqlPromise]);
  const db = new SQL.Database(new Uint8Array(dbArrayBuffer));
  sqliteInstance = db;
  return db;
}

export const executeQuery = async (db: Database, query: string, params: BindParams) => {
  if (!sqliteInstance) {
    throw new Error('Database not initialized. Call `fetchAndCacheDatabase` first.');
  }
  const result = db.exec(query, params);
  return result.length ? result[0] : null; // Return the first result set
};

// TODO: Add ETag support in the future for cache invalidation
// async function getETag(url: string): Promise<string | null> {
//   // Using HEAD request
//   const resp = await fetch(url, { method: 'HEAD' });
//   if (!resp.ok) {
//     throw new Error(`HEAD request failed with status ${resp.status}`);
//   }
//   return resp.headers.get('ETag');
// }
