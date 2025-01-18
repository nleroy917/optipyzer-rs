import { Html, Head, Main, NextScript } from 'next/document';

export default function Document() {
  return (
    <Html lang="en">
      <Head />
      <body className="max-w-7xl mx-auto min-h-svh bg-background font-sans antialiased bg-slate-50 dark:bg-slate-950">
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}
