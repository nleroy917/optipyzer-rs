'use client';

import { useState } from 'react';
import { motion } from 'framer-motion';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

import { parseFastaSequencesFromString } from 'multimizerjs';
import { OptimizationPlayground } from '@/components/optimizer/playground';
import { cn, convertMapToObject } from '@/lib/utils';
import { Textarea } from '@/components/ui/textarea';
import { Button } from '@/components/ui/button';
import { useCodonDatabase } from '@/contexts/codon-database-context';
import { QueryExecResult } from 'sql.js';
import Layout from '@/components/layout';

export default function Home() {
  const [query, setQuery] = useState('');
  const [sql, setSql] = useState('');
  const [rows, setRows] = useState<QueryExecResult | null>(null);

  const { query: queryCodonDb, progress, loading, error } = useCodonDatabase();

  const parsedFastaSequences = parseFastaSequencesFromString(query);
  const parsedFastaSequencesResult = convertMapToObject(parsedFastaSequences.result as Map<string, string>);

  return (
    <Layout title="Multipizer">
      <div className="items-center justify-items-center min-h-screen p-8 font-[family-name:var(--font-geist-sans)]">
        <div className="flex flex-col gap-4">
          <div>
            <h1 className="text-4xl font-bold mb-2">Multipizer</h1>
            <p className="mb-4">Multispecies codon optimization engine written in Rust.</p>
          </div>
          <OptimizationPlayground query={query} setQuery={setQuery} />
          {query.length > 0 && (
            <Card>
              <CardHeader className="p-0 border-b">
                <div className="px-6 py-4">
                  <CardTitle>Optimized</CardTitle>
                </div>
              </CardHeader>
              <CardContent className="p-4">
                <div className="break-all flex flex-col gap-2">
                  {Object.keys(parsedFastaSequencesResult).map((key) => {
                    return (
                      <div key={key}>
                        <strong>{key}</strong>
                        <br />
                        <p className="break-all">{parsedFastaSequencesResult[key]}</p>
                      </div>
                    );
                  })}
                </div>
              </CardContent>
            </Card>
          )}
          <Card>
            <CardHeader className="p-0 border-b">
              <div className="px-6 py-4 flex items-center justify-between">
                <CardTitle>SQL playground</CardTitle>
                {loading ? (
                  <span className="text-xs font-bold rounded-full px-2 py-1 bg-amber-100 text-amber-800 flex items-center gap-2">
                    <span className="relative flex h-3 w-3">
                      <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-amber-400 opacity-75" />
                      <span className="relative inline-flex rounded-full h-3 w-3 bg-amber-500" />
                    </span>
                    Loading
                  </span>
                ) : (
                  <span className="text-xs font-bold rounded-full px-2 py-1 bg-emerald-100 text-emerald-800 flex items-center gap-2">
                    <span className="relative flex h-3 w-3">
                      <span className="absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75" />
                      <span className="relative inline-flex rounded-full h-3 w-3 bg-emerald-500" />
                    </span>
                    Connected
                  </span>
                )}
                {error && <p className="text-red-500">{error}</p>}
              </div>
              <div className="w-full relative">
                <motion.div
                  style={{
                    width: `${progress * 100}%`,
                  }}
                  className={cn('absolute top-0 left-0 w-full h-0.5 bg-amber-500', progress === 1 && 'bg-emerald-500')}
                />
              </div>
            </CardHeader>
            <CardContent className="p-4">
              <Textarea value={sql} onChange={(e) => setSql(e.target.value)} rows={10} className="mb-2" />
              <Button
                size="lg"
                onClick={() =>
                  queryCodonDb(sql).then((result) => {
                    if (result) {
                      setRows(result);
                    }
                  })
                }
              >
                Execute
              </Button>
            </CardContent>
          </Card>
          {rows && (
            <pre className="text-sm text-gray-500">
              <code>{JSON.stringify(rows, null, 2)}</code>
            </pre>
          )}
        </div>
      </div>
    </Layout>
  );
}
