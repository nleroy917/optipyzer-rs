'use client';

import { useState } from 'react';
import { Textarea } from '@/components/ui/textarea';

import { optimize } from 'multimizerjs';
import { removeAllNewLines } from '@/lib/utils';

export default function Home() {
  const [query, setQuery] = useState('');
  return (
    <div className="items-center justify-items-center min-h-screen p-8 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col">
        <p className="font-bold mb-2">Paste you sequence:</p>
        <Textarea
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Paste your sequence here"
          className="mb-4"
          rows={20}
        />
        <div className="mb-2">
          <p className="font-bold">Optimized sequence:</p>
          <p className="text-wrap w-[500px] break-words">{optimize(removeAllNewLines(query))}</p>
        </div>
      </main>
    </div>
  );
}
