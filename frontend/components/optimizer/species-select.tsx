import { Check, ChevronsUpDown } from 'lucide-react';

import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList } from '@/components/ui/command';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { useState } from 'react';

const species = [
  { value: '122771', label: 'African Clawed Frog (Xenopus laevis)' },
  { value: '121713', label: 'Bakers Yeast (Saccharomyces cerevisiae S288C)' },
  { value: '122001', label: 'Caenorhabditis elegans' },
  { value: '16815', label: 'E. Coli (Escherichia coli str. K-12 substr. MG1655)' },
  { value: '122056', label: 'Fruit Fly (Drosophila melanogaster)' },
  { value: '122563', label: 'Human (Homo sapiens)' },
  { value: '122638', label: 'Mouse (Mus musculus)' },
  { value: '122645', label: 'Rat (Rattus norvegicus)' },
  { value: '122263', label: 'Thale Cress (Arabidopsis thaliana)' },
  { value: '122731', label: 'Zebrafish (Danio rerio)' },
];

interface SpeciesSelectProps {
  value: string | undefined;
  setValue: (value: string) => void;
}

export function SpeciesSelect(props: SpeciesSelectProps) {
  const { value, setValue } = props;

  const [open, setOpen] = useState(false);

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button variant="outline" role="combobox" aria-expanded={open} className="w-full justify-between">
          {value ? species.find((s) => s.value === value)?.label : 'Select species...'}
          <ChevronsUpDown className="opacity-50" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="p-0">
        <Command>
          <CommandInput placeholder="Select species" className="h-9" />
          <CommandList>
            <CommandEmpty>No species found.</CommandEmpty>
            <CommandGroup>
              {species.map((s) => (
                <CommandItem
                  key={s.value}
                  value={s.value}
                  onSelect={(currentValue) => {
                    setValue(currentValue === value ? '' : currentValue);
                    setOpen(false);
                  }}
                >
                  {s.label}
                  <Check className={cn('ml-auto', value === s.value ? 'opacity-100' : 'opacity-0')} />
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
}
