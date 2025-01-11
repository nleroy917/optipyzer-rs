'use client';

import { useState } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';

import { parseFastaSequencesFromString } from 'multimizerjs';
import { OptimizationPlayground } from '@/components/optimizer/playground';
import { convertMapToObject } from '@/lib/utils';

export default function Home() {
  const [query, setQuery] = useState('');
  const parsedFastaSequences = parseFastaSequencesFromString(query);
  const parsedFastaSequencesResult = convertMapToObject(parsedFastaSequences.result as Map<string, string>);

  return (
    <div className="items-center justify-items-center min-h-screen p-8 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-4">
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
      </main>
    </div>
  );
}
