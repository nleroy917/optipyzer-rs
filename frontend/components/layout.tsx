import Head from 'next/head';
import { Fragment } from 'react';
import { ThemeToggle } from './theme-toggle';

type Props = {
  children: React.ReactNode;
  title?: string;
  description?: string;
  footer?: boolean;
  nav?: boolean;
};

export default function Layout({ children, title, description }: Props) {
  return (
    <Fragment>
      <Head>
        <title>{title || 'Multipyzer'}</title>
        <meta
          name="description"
          content={
            description ||
            'Performance-critical genomic interval analysis and preprocessors written in rust, target to the browser with WASM.'
          }
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <header>
        <div className="absolute top-0 right-0 p-4">
          <ThemeToggle />
        </div>
      </header>
      <main>{children}</main>
    </Fragment>
  );
}
