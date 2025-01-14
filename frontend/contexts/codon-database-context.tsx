'use client';

import React, { createContext, useContext, useEffect, useState } from 'react';
import { fetchAndCacheDatabase, executeQuery } from '../lib/sqlite';

const DB_URL = 'https://multimizer-public-files.s3.us-east-2.amazonaws.com/codon.db';

type CodonDatabaseContextType = {
  // @ts-expect-error - query is not defined yet
  query: (query: string, params?: any[]) => Promise<any[]>;
  loading: boolean;
  error: string | null;
};

const CodonDatabaseContext = createContext<CodonDatabaseContextType | null>(null);

export const CodonDatabaseProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const initializeDatabase = async () => {
      setLoading(true);
      try {
        await fetchAndCacheDatabase(DB_URL);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    initializeDatabase();
  }, []);

  const query = async (sql: string, params: any[] = []) => {
    if (loading || error) throw new Error('Database is not ready yet.');
    return executeQuery(sql, params);
  };

  return <CodonDatabaseContext.Provider value={{ query, loading, error }}>{children}</CodonDatabaseContext.Provider>;
};

export const useCodonDatabase = () => {
  const context = useContext(CodonDatabaseContext);
  if (!context) {
    throw new Error('useCodonDatabase must be used within a CodonDatabaseProvider.');
  }
  return context;
};
