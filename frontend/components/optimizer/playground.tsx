import { motion } from 'framer-motion';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Textarea } from '@/components/ui/textarea';
import { Input } from '@/components/ui/input';

import { cn } from '@/lib/utils';
import { SpeciesSelect } from './species-select';

interface OptimizationPlaygroundProps {
  query: string;
  setQuery: (query: string) => void;
  dbLoadingProgress: number;
  dbLoadingError: string | null;
  dbLoading: boolean;
  orgId: string | undefined;
  setOrgId: (val: string) => void;
}

export const OptimizationPlayground = (props: OptimizationPlaygroundProps) => {
  const { query, setQuery, dbLoading, dbLoadingError, dbLoadingProgress, orgId, setOrgId } = props;

  return (
    <Card>
      <CardHeader className="p-0 border-b">
        <div className="px-6 py-4 flex items-center justify-between">
          <CardTitle>Optimize sequences</CardTitle>
          {dbLoading ? (
            <span className="text-xs font-bold rounded-full px-2 py-1 bg-amber-100 text-amber-800 flex items-center gap-2">
              <span className="relative flex h-3 w-3">
                <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-amber-400 opacity-75" />
                <span className="relative inline-flex rounded-full h-3 w-3 bg-amber-500" />
              </span>
              Initializing
            </span>
          ) : (
            <span className="text-xs font-bold rounded-full px-2 py-1 bg-emerald-100 text-emerald-800 flex items-center gap-2">
              <span className="relative flex h-3 w-3">
                <span className="absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75" />
                <span className="relative inline-flex rounded-full h-3 w-3 bg-emerald-500" />
              </span>
              Ready
            </span>
          )}
          {dbLoadingError && <p className="text-red-500">{dbLoadingError}</p>}
        </div>
        <div className="w-full relative">
          <motion.div
            style={{
              width: `${dbLoadingProgress * 100}%`,
            }}
            className={cn(
              'absolute top-0 left-0 w-full h-0.5 bg-amber-300 transition-transform duration-300',
              dbLoadingProgress === 1 && 'bg-emerald-300',
            )}
          />
        </div>
      </CardHeader>
      <CardContent className="p-4 grid grid-cols-4 gap-4">
        <div className="col-span-3">
          <Textarea
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="Paste your sequence(s) here"
            rows={25}
          />
        </div>
        <div className="col-span-1 flex flex-col gap-4">
          <div className="flex flex-col">
            <label className="font-semibold text-sm">Species:</label>
            <SpeciesSelect value={orgId} setValue={setOrgId} />
          </div>
          <div>
            <label className="font-semibold text-sm">Seed:</label>
            <Input placeholder="42" />
          </div>
          <div>
            <label className="font-semibold text-sm">Max iterations:</label>
            <Input placeholder="1000" />
          </div>
        </div>
      </CardContent>
    </Card>
  );
};
