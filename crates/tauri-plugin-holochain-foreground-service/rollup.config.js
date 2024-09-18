import { readFileSync } from 'fs'
import { join } from 'path'
import { cwd } from 'process'
import typescript from '@rollup/plugin-typescript'
import terser from '@rollup/plugin-terser'
import { nodeResolve } from '@rollup/plugin-node-resolve';

const pkg = JSON.parse(readFileSync(join(cwd(), 'package.json'), 'utf8'))

export default [
  {
    input: 'dist-js-tmp/holochain-env/index.js',
    output: [
      {
        file: "dist-js/holochain-env/index.min.js",
        format: 'esm',
      },
      {
        file: "android/src/main/javascript/injectHolochainClientEnv.min.js",
        format: 'esm'
      },
    ],
    plugins: [
      terser(),      
      // Include imported dependencies in the output bundle
      nodeResolve(),
    ],
  },
]