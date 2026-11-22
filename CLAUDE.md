# totalmapper

## Building and testing

Use `cargo +1.89.0` (e.g. `cargo +1.89.0 test`). The default stable toolchain
(1.82) is too old for some dependencies, which need edition 2024.

## Releasing a new version

1. Bump `version` in `Cargo.toml` and the `totalmapper` entry in `Cargo.lock`
   (check with `cargo +1.89.0 build --locked`).
2. Commit it as "Bump version to X.Y.Z".
3. Push `main`.
4. Tag that bump commit `vX.Y.Z` (a lightweight tag, like the existing ones).
5. Push only the tag: `git push origin vX.Y.Z`. Do not use `git push --tags`.
   The tag push triggers `.github/workflows/release.yml`, which builds the
   .deb and static tarball assets and attaches them to a GitHub release.
6. After the release assets exist, update the package links under
   "Packages" in `README.md` to the new version, commit as "Update release
   artifact links in README", and push `main`.
