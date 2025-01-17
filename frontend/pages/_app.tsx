import { ThemeProvider } from '@/components/theme-provider';
import { CodonDatabaseProvider } from '@/contexts/codon-database-context';
import '@/styles/globals.css';
import type { AppProps } from 'next/app';

export default function App({ Component, pageProps }: AppProps) {
  return (
    <ThemeProvider attribute="class" defaultTheme="system" enableSystem disableTransitionOnChange>
      <CodonDatabaseProvider>
        <Component {...pageProps} />
      </CodonDatabaseProvider>
    </ThemeProvider>
  );
}
