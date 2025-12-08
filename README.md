# mdx-link-extract

Fast link extraction in MDX files. This package was created specifically for [Qiskit/documentation](https://github.com/Qiskit/documentation).

## Install this test package

```bash
yarn add @qiskit/mdx-link-extract
```

## Developing

### Build

After running `npm run build`, you should see `package-template.[darwin|win32|linux].node` in the project root. This is the native addon built from [lib.rs](./src/lib.rs).

### Test

With [ava](https://github.com/avajs/ava), run `npm run test` to test the native addon. You can also switch to another testing framework if you want.

### CI

With GitHub Actions, each commit and pull request will be built and tested automatically in a [`node@20`, `@node22`] x [`macOS`, `Linux`, `Windows`] matrix.

### Release

Our GitHub action prebuilds a binary for each supported platform. We release different `npm` packages for each platform and add them to `optionalDependencies` before releasing the major package to npm. The package manager will choose which native package to download from the registry automatically. You can see the [npm](./npm) dir for details.

## Develop requirements

- Install the latest `Rust`
- Install `Node.js@10+` which fully supported `Node-API`

## Test in local

- npm ci
- npm run build
- npm run test

And you will see:

```bash
$ ava --verbose

  ✔ sync function from native code
  ✔ sleep function from native code (201ms)
  ─

  2 tests passed
✨  Done in 1.12s.
```

## Release package

When you want to release the package:

```bash
npm version [<newversion> | major | minor | patch | premajor | preminor | prepatch | prerelease [--preid=<prerelease-id>] | from-git]

git push
```

GitHub actions will do the rest job for you.

> WARN: Don't run `npm publish` manually.
