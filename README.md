# mdx-link-extract

Fast link extraction in MDX files. This package was created specifically for [Qiskit/documentation](https://github.com/Qiskit/documentation).

## Install this test package

```bash
yarn add @qiskit/mdx-link-extract
```

## Developing

### Build

After running `yarn build`, you should see `package-template.[darwin|win32|linux].node` in the project root. This is the native addon built from [lib.rs](./src/lib.rs).

### Test

With [ava](https://github.com/avajs/ava), run `yarn test` to test the native addon. You can also switch to another testing framework if you want.

### CI

With GitHub Actions, each commit and pull request will be built and tested automatically in a [`node@20`, `@node22`] x [`macOS`, `Linux`, `Windows`] matrix.

### Release

Our GitHub action prebuilds a binary for each supported platform. We release different `npm` packages for each platform and add them to `optionalDependencies` before releasing the major package to npm. The package manager will choose which native package to download from the registry automatically. You can see the [npm](./npm) dir for details.

## Develop requirements

- Install `rustc ~= 1.88`. We recommend using [`rustup`](https://rust-lang.org/tools/install/) to install it.
- Install `node >= 22.21`. We recommend using [Node version manager (`nvm`)](https://github.com/nvm-sh/nvm).

## Test locally

- `nvm use 22.21.1`
- `yarn install`
- `yarn build`
- `yarn test`

And you should see:

```

  ✔ sync function from native code
  ─

  1 test passed
```

## Release package

When you want to release the package:

```bash
npm version [<newversion> | major | minor | patch | premajor | preminor | prepatch | prerelease [--preid=<prerelease-id>] | from-git]

git push
```

GitHub actions will do the rest job for you.

> WARN: Don't run `npm publish` manually.
