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

Our GitHub Action prebuilds a binary for each supported platform. We release different `npm` packages for each platform and add them to `optionalDependencies` before releasing the major package to npm. The package manager will choose which native package to download from the registry automatically.

### Release package

The process isn't ideal for our workflow, we inherited it from the `napi-rs` template. The release action looks for a new commit on main that starts with a valid semver number (such as `4.12.3`). Since we only allow merge commits, this means we need to merge a PR with the semver as a title. We also need to update the `package.json` in the same PR.

Follow these steps for a foolproof way to make a new release:

1. Make a new branch from up-to-date `main`.
   ```sh
   git switch -c release && git reset --hard origin/main && git pull origin main
   ```
2. Run the following command to update `package.json` and make a commit:

   ```bash
   npm version [<newversion> | major | minor | patch | premajor | preminor | prepatch | prerelease [--preid=<prerelease-id>] | from-git]
   ```

3. Run `gh pr create` and accept the defaults. This will create a PR with the correct title. Submit the PR.
4. Once CI has passed and been approved, merge the PR. This will trigger a release.

> WARN: Don't run `npm publish` manually.
