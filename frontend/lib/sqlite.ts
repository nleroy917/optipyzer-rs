import initSqlJs, { Database } from 'sql.js';

const DB_CACHE_NAME = 'codonDatabase';
const DB_CACHE_KEY = 'sqlite-file';

let sqliteInstance: Database | null = null;

const saveToIndexedDB = async (key: string, value: Uint8Array) => {
  const db = await indexedDB.open(DB_CACHE_NAME, 1);
  db.onupgradeneeded = (event: IDBVersionChangeEvent) => {
    // @ts-expect-error - createObjectStore is not defined on IDBRequest
    event?.target?.result.createObjectStore(DB_CACHE_NAME);
  };
  return new Promise<void>((resolve, reject) => {
    const transaction = db.result.transaction(DB_CACHE_NAME, 'readwrite');
    const store = transaction.objectStore(DB_CACHE_NAME);
    const request = store.put(value, key);
    request.onsuccess = () => resolve();
    request.onerror = () => reject(request.error);
  });
};

const loadFromIndexedDB = async (key: string): Promise<Uint8Array | null> => {
  const db = await indexedDB.open(DB_CACHE_NAME, 1);
  return new Promise((resolve, reject) => {
    const transaction = db.result.transaction(DB_CACHE_NAME, 'readonly');
    const store = transaction.objectStore(DB_CACHE_NAME);
    const request = store.get(key);
    request.onsuccess = () => resolve(request.result || null);
    request.onerror = () => reject(request.error);
  });
};

export const fetchAndCacheDatabase = async (dbUrl: string): Promise<Database> => {
  if (sqliteInstance) return sqliteInstance; // Use cached instance if available

  const SQL = await initSqlJs({
    locateFile: (file: string) => `https://cdnjs.cloudflare.com/ajax/libs/sql.js/1.8.0/${file}`,
  });

  // check IndexedDB for cached database file
  let dbFile: Uint8Array | null = await loadFromIndexedDB(DB_CACHE_KEY);

  // if not cached, fetch from S3 and save to IndexedDB
  if (!dbFile) {
    const response = await fetch(dbUrl);
    if (!response.ok) {
      throw new Error(`Failed to fetch database from ${dbUrl}`);
    }
    const arrayBuffer = await response.arrayBuffer();
    dbFile = new Uint8Array(arrayBuffer);

    // Cache the database file for future use
    await saveToIndexedDB(DB_CACHE_KEY, dbFile);
  } else {
    // pass
  }

  // Create SQLite instance
  sqliteInstance = new SQL.Database(dbFile);
  return sqliteInstance;
};

// @ts-expect-error - params is not defined yet
export const executeQuery = async (query: string, params: any[] = []) => {
  if (!sqliteInstance) {
    throw new Error('Database not initialized. Call `fetchAndCacheDatabase` first.');
  }
  // Execute query and return results
  const result = sqliteInstance.exec(query, params);
  return result.length ? result[0] : null; // Return the first result set
};
