import initSqlJs, { BindParams, Database } from 'sql.js';

let sqliteInstance: Database | null = null;

export const fetchDbAndInit = async (dbUrl: string): Promise<Database> => {
  if (sqliteInstance) return sqliteInstance; // Use cached instance if available

  const sqlPromise = initSqlJs({
    locateFile: (file: string) => `https://sql.js.org/dist/${file}`,
  });

  const dataPromise = fetch(dbUrl).then((res) => res.arrayBuffer());
  const [SQL, buf] = await Promise.all([sqlPromise, dataPromise]);
  const db = new SQL.Database(new Uint8Array(buf));

  sqliteInstance = db;

  return db;
};

export const executeQuery = async (db: Database, query: string, params: BindParams) => {
  if (!sqliteInstance) {
    throw new Error('Database not initialized. Call `fetchAndCacheDatabase` first.');
  }
  // Execute query and return results
  const result = sqliteInstance.exec(query, params);
  return result.length ? result[0] : null; // Return the first result set
};
