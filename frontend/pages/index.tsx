import { useState } from 'react';

import { OptimizationPlayground } from '@/components/optimizer/playground';
import { useCodonDatabase } from '@/contexts/codon-database-context';
import Layout from '@/components/layout';
import { useCodonUsageForSpecies } from '@/queries/use-codon-usage';
import { convertQueryResultToObjects } from '@/lib/utils';

export default function Home() {
  const [query, setQuery] = useState('');
  const [orgId, setOrgId] = useState<string | undefined>();

  const { progress, loading, error } = useCodonDatabase();
  const { data: codonUsage } = useCodonUsageForSpecies(orgId);

  return (
    <Layout title="Multimizer">
      <div className="items-center justify-items-center min-h-screen p-8 font-[family-name:var(--font-geist-sans)]">
        <div className="flex flex-col gap-4">
          <div>
            <h1 className="text-4xl font-bold mb-2">Multimizer</h1>
            <p className="mb-4">Multispecies codon optimization engine written in Rust.</p>
          </div>
          <OptimizationPlayground
            query={query}
            setQuery={setQuery}
            dbLoadingProgress={progress}
            dbLoadingError={error}
            dbLoading={loading}
            orgId={orgId}
            setOrgId={setOrgId}
          />
        </div>
        <div>
          <code>
            {codonUsage && (
              <pre>
                OrdId: {orgId}
                Usage: {JSON.stringify(convertQueryResultToObjects(codonUsage), null, 2)}
              </pre>
            )}
          </code>
        </div>
      </div>
    </Layout>
  );
}
