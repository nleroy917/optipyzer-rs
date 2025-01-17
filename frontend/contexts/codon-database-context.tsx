import React, { createContext, useContext, useEffect, useState } from 'react';
import { fetchDbAndInit, executeQuery } from '../lib/sqlite';
import { BindParams, Database, QueryExecResult } from 'sql.js';

const DB_URL = 'https://multimizer-public-files.s3.us-east-2.amazonaws.com/codon.db';

type CodonDatabaseContextType = {
  query: (query: string, params?: BindParams) => Promise<QueryExecResult | null>;
  loading: boolean;
  error: string | null;
  progress: number;
};

const CodonDatabaseContext = createContext<CodonDatabaseContextType | null>(null);

export const CodonDatabaseProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [db, setDb] = useState<Database | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    const initializeDatabase = async () => {
      setLoading(true);
      try {
        return await fetchDbAndInit(DB_URL, (progressFraction) => setProgress(progressFraction));
      } catch (err) {
        console.error(err);
        setError('There was an error reading the database. Please try again later.');
      } finally {
        setLoading(false);
      }
    };

    initializeDatabase().then((db) => {
      setDb(db || null);
    });
  }, []);

  const query = async (sql: string, params: BindParams = []) => {
    if (loading || error) throw new Error('Database is not ready yet.');
    if (!db) throw new Error('Database is not ready yet.');
    return executeQuery(db, sql, params);
  };

  return (
    <CodonDatabaseContext.Provider value={{ query, loading, error, progress }}>
      {children}
    </CodonDatabaseContext.Provider>
  );
};

export const useCodonDatabase = () => {
  const context = useContext(CodonDatabaseContext);
  if (!context) {
    throw new Error('useCodonDatabase must be used within a CodonDatabaseProvider.');
  }
  return context;
};
