'use client';

import { useState } from 'react';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';

import { optimize } from 'multimizerjs';
import { removeAllNewLines } from '@/lib/utils';

export default function Home() {
  const [query, setQuery] = useState('');
  return (
    <div className="items-center justify-items-center min-h-screen p-8 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-4 max-w-6xl mx-auto">
        <div>
          <h1 className="text-4xl font-bold mb-2">Multipizer</h1>
          <p className="mb-4">Multispecies codon optimization engine written in Rust.</p>
        </div>
        <Card>
          <CardHeader>
            <CardTitle>Paste you sequence:</CardTitle>
            <CardDescription>Paste your sequence here to optimize it for the selected species.</CardDescription>
          </CardHeader>
          <CardContent>
            <Textarea
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Paste your sequence here"
              rows={10}
            />
          </CardContent>
          <CardFooter className="grid grid-cols-4 gap-4">
            <div>
              <label className="font-bold text-sm">Seed:</label>
              <Input placeholder="42" />
            </div>
            <div>
              <label className="font-bold text-sm">Max iterations:</label>
              <Input placeholder="1000" />
            </div>
          </CardFooter>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle>Optimized sequence:</CardTitle>
            <CardDescription>Optimized sequence for the selected species.</CardDescription>
          </CardHeader>
          <CardContent>
            <Textarea value={removeAllNewLines(optimize(query))} placeholder="Optimized sequence" rows={10} readOnly />
          </CardContent>
        </Card>
      </main>
    </div>
  );
}
