import React from 'react';
import { ThemeProvider } from '@/components/theme-provider';
import { CodonDatabaseProvider } from '@/contexts/codon-database-context';
import { HydrationBoundary, QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import type { AppProps } from 'next/app';
import '@/styles/globals.css';

export default function App({ Component, pageProps }: AppProps) {
  const [queryClient] = React.useState(() => new QueryClient());
  return (
    <ThemeProvider attribute="class" defaultTheme="system" enableSystem disableTransitionOnChange>
      <QueryClientProvider client={queryClient}>
        <HydrationBoundary state={pageProps.dehydratedState}>
          <CodonDatabaseProvider>
            <Component {...pageProps} />
          </CodonDatabaseProvider>
        </HydrationBoundary>
        <ReactQueryDevtools />
      </QueryClientProvider>
    </ThemeProvider>
  );
}
