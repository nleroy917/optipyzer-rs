import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Textarea } from '@/components/ui/textarea';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Code, MoreHorizontal, SaveIcon, ShareIcon } from 'lucide-react';

interface OptimizationPlaygroundProps {
  query: string;
  setQuery: (query: string) => void;
}

export const OptimizationPlayground = (props: OptimizationPlaygroundProps) => {
  const { query, setQuery } = props;
  return (
    <Card>
      <CardHeader className="p-0 border-b flex flex-row items-center justify-between">
        <div className="px-6 py-4">
          <CardTitle>Optimize sequences</CardTitle>
        </div>
        <div className="px-6 py-2 flex flex-row items-center gap-2">
          <Button size="sm" variant="outline">
            <SaveIcon size={16} />
            Save
          </Button>
          <Button size="sm" variant="outline">
            <ShareIcon size={16} />
            Share
          </Button>
          <Button size="sm" variant="outline">
            <Code size={16} />
            View code
          </Button>
          <DropdownMenu modal={false}>
            <DropdownMenuTrigger asChild>
              <Button variant="outline" size="sm">
                <MoreHorizontal size={16} />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent>
              <DropdownMenuItem>About</DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem className="text-destructive">Reset</DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
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
