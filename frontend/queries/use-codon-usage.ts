import { useCodonDatabase } from '@/contexts/codon-database-context';
import { useQuery } from '@tanstack/react-query';

export const useCodonUsageForSpecies = (orgId: string | undefined) => {
  const { query, loading } = useCodonDatabase();
  return useQuery({
    queryFn: () => {
      return query('select * from codon_usage where org_id = :org', {
        ':org': orgId!,
      });
    },
    queryKey: ['codon-usage', orgId],
    enabled: !loading && !!orgId,
  });
};
