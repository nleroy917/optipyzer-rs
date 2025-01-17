const CODON_DB_CACHE = 'codonDbCache';
const CODON_OBJECT_STORE = 'codonStore';

export async function openIDB(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(CODON_DB_CACHE, 1);

    request.onupgradeneeded = (e) => {
      const db = (e.target as IDBOpenDBRequest).result;
      // Create an object store to hold our DB file
      if (!db.objectStoreNames.contains(CODON_OBJECT_STORE)) {
        db.createObjectStore(CODON_OBJECT_STORE);
      }
    };

    request.onsuccess = (e) => {
      const db = (e.target as IDBOpenDBRequest).result;
      resolve(db);
    };

    request.onerror = (e) => {
      reject((e.target as IDBOpenDBRequest).error);
    };
  });
}

export async function getCachedDb(db: IDBDatabase, key: string): Promise<ArrayBuffer | null> {
  return new Promise((resolve, reject) => {
    const tx = db.transaction(CODON_OBJECT_STORE, 'readonly');
    const store = tx.objectStore(CODON_OBJECT_STORE);
    const request = store.get(key);

    request.onsuccess = () => {
      resolve(request.result ?? null);
    };
    request.onerror = () => {
      reject(request.error);
    };
  });
}

export async function setCachedDb(db: IDBDatabase, key: string, data: ArrayBuffer): Promise<void> {
  return new Promise((resolve, reject) => {
    const tx = db.transaction(CODON_OBJECT_STORE, 'readwrite');
    const store = tx.objectStore(CODON_OBJECT_STORE);
    const request = store.put(data, key);

    request.onsuccess = () => {
      resolve();
    };
    request.onerror = () => {
      reject(request.error);
    };
  });
}
