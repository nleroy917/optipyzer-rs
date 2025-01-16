import { NextConfigComplete } from 'next/dist/server/config-shared';

function patchWasmModuleImport(config: NextConfigComplete, isServer: boolean) {
  config.experiments = Object.assign(config.experiments || {}, {
    asyncWebAssembly: true,
  });

  config.optimization.moduleIds = 'named';

  config.module.rules.push({
    test: /\.wasm$/,
    type: 'webassembly/async',
  });

  // TODO: improve this function -> track https://github.com/vercel/next.js/issues/25852
  if (isServer) {
    // @ts-expect-error ignore
    config.output.webassemblyModuleFilename = './../static/wasm/[modulehash].wasm';
  } else {
    // @ts-expect-error ignore
    config.output.webassemblyModuleFilename = 'static/wasm/[modulehash].wasm';
  }
}

/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  webpack: function (config: NextConfigComplete, options: { isServer: boolean }) {
    patchWasmModuleImport(config, options.isServer);
    config.experiments = { asyncWebAssembly: true, layers: true };
    if (!options.isServer) {
      config.resolve.fallback.fs = false;
    }
    return config;
  },
};

module.exports = nextConfig;
