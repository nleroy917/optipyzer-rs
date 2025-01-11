function patchWasmModuleImport(config, isServer) {
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
    config.output.webassemblyModuleFilename = './../static/wasm/[modulehash].wasm';
  } else {
    config.output.webassemblyModuleFilename = 'static/wasm/[modulehash].wasm';
  }
}

/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  webpack: function (config, options) {
    patchWasmModuleImport(config, options.isServer);
    config.experiments = { asyncWebAssembly: true, layers: true };
    return config;
  },
};

module.exports = nextConfig;
