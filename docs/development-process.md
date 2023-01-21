# imgui-rs development process

As summarised in [issue #665](https://github.com/imgui-rs/imgui-rs/issues/665)

In summary:

1. There is a `main` branch
2. For each semver compatible release there is a stable branch (e.g `0.9-stable`)
3. Each patch release becomes a tagged commit on this stable branch (e.g `v0.9.5` would come from a tagged commit on the `0.9-stable` branch)

## General process

Day to day development

1. Work on `main` branch
2. General PR's are submitted against the `main` branch

When it is time to make a new release, we create a `x.y-stable` branch (e.g `0.9-stable`) from `main`

1. Ensure `CHANGELOG` is up to date
2. Ensure README is up-to-date (including the Dear ImGui Version in badge URL, MSRV)
3. Bump `version` in the various `Cargo.toml`
4. A stable branch is created, e.g `git switch -c 0.9-stable` and pushed to Github
5. Publish various crates (noting it has to be done starting with `imgui-sys`, then `imgui`, then the others)
  ```
  cargo publish -p imgui-sys
  cargo publish -p imgui
  cargo publish -p imgui-winit-support --no-verify
  cargo publish -p imgui-glium-renderer
  cargo publish -p imgui-glow-renderer
  cargo publish -p imgui-sdl2-support
  ```
6. Create annotated tag `v0.9.0` and push to github
7. Create Release for this version on Github
8. Update and close any relevant tickets

All further PR's are still done to `main`

1. If they are applicable to previous release (e.g bugfixes or non-breaking changes), they are cherry-picked back to the applicable `stable` branch(es)

## When to bump versions in Cargo.toml

Only before publishing to crates.io.

This makes users able use `[patch.crates-io]` without hand-editing versions throughout their dependency tree (typically impossible without forking/editing transitive dependencies, even if there are no breaking code changes otherwise).
