import { useCodonDatabase } from '@/contexts/codon-database-context';
import { useQuery } from '@tanstack/react-query';

export const useCodonUsageForSpecies = (orgId: string | undefined) => {
  const { query, loading } = useCodonDatabase();
  return useQuery({
    queryFn: () => {
      return query(
        'select TTT,TTC,TTA,TTG,CTT,CTC,CTA,CTG,ATT,ATC,ATA,ATG,GTT,GTC,GTA,GTG,TAT,TAC,TAA,TAG,CAT,CAC,CAA,CAG,AAT,AAC,AAA,AAG,GAT,GAC,GAA,GAG,TCT,TCC,TCA,TCG,CCT,CCC,CCA,CCG,ACT,ACC,ACA,ACG,GCT,GCC,GCA,GCG,TGT,TGC,TGA,TGG,CGT,CGC,CGA,CGG,AGT,AGC,AGA,AGG,GGT,GGC,GGA,GGG from codon_usage where org_id = :org',
        {
          ':org': orgId!,
        },
      );
    },
    queryKey: ['codon-usage', orgId],
    enabled: !loading && !!orgId,
  });
};
