# mdx-link-extract

Fast link extraction in MDX files. This package was created specifically for [Qiskit/documentation](https://github.com/Qiskit/documentation).

## Installation

After the first release of the package, you should be able to install it like any other `npm` package:

```bash
npm install @qiskit/mdx-link-extract
# or
yarn add @qiskit/mdx-link-extract
```

## Developing

- Install `rustc ~= 1.88`. We recommend using [`rustup`](https://rust-lang.org/tools/install/) to install it.
- Install `node >= 22.21`. We recommend using [Node version manager (`nvm`)](https://github.com/nvm-sh/nvm).

## Test locally

- `nvm use 22.21.1`
- `yarn install`
- `yarn build`
- `yarn test`

You should see all the tests pass.

## Publishing

Our GitHub action prebuilds a binary for each supported platform. We release different `npm` packages for each platform and add them to `optionalDependencies` before releasing the major package to npm. The package manager will choose which native package to download from the registry automatically.

### Release package

When you want to release the package:

```bash
npm version [<newversion> | major | minor | patch | premajor | preminor | prepatch | prerelease [--preid=<prerelease-id>] | from-git]

git push
```

GitHub actions will do the rest job for you.

> WARN: Don't run `npm publish` manually.
