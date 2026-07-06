# crates.io metadata gap: repository field still points to `SuperInstance`

## Summary

The published crates.io crate for `constraint-theory-core` (version **2.2.0**)
still advertises the old `SuperInstance/constraint-theory-core` repository URL,
while the source repository and changelog have already moved to
`purplepincher/constraint-theory-core`. This document confirms the gap and
explains what is required to fix the *published* metadata.

## 1. Current state

### Source manifest (`Cargo.toml`)

On `main` in this repository, the manifest already points to the correct owner:

```toml
[package]
name = "constraint-theory-core"
version = "2.2.0"
repository = "https://github.com/purplepincher/constraint-theory-core"
homepage = "https://github.com/purplepincher/constraint-theory-core"
```

### Published crates.io metadata

The crates.io API entry for `constraint-theory-core` still reports the stale
`SuperInstance` URLs:

```json
{
  "crate": {
    "homepage": "https://github.com/SuperInstance/constraint-theory-core",
    "repository": "https://github.com/SuperInstance/constraint-theory-core"
  },
  "versions": [
    {
      "num": "2.2.0",
      "homepage": "https://github.com/SuperInstance/constraint-theory-core",
      "repository": "https://github.com/SuperInstance/constraint-theory-core"
    }
  ]
}
```

This was verified with:

```bash
curl -H "User-Agent: purplepincher-audit (contact: casey.digennaro@gmail.com)" \
  https://crates.io/api/v1/crates/constraint-theory-core
```

## 2. Can the published metadata be fixed without a new release?

No. crates.io treats every published version as immutable. The Cargo Book states:

> Take care when publishing a crate, because a publish is **permanent**. The
> version can never be overwritten, and the code cannot be deleted. There is no
> limit to the number of versions which can be published, however.
>
> -- [The Cargo Book: Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)

Because the `repository`, `homepage`, and other manifest metadata are baked into
the uploaded `.crate` file for version 2.2.0, there is no web UI or API endpoint
that allows editing those fields for an already-published version. The only
metadata-level actions available for an existing version are:

* **Yank** / **unyank** — this only prevents new dependencies from selecting
  that version; it does not change the metadata shown for it.
* **Owner changes** — these affect who can publish *future* versions, not the
  metadata of existing ones.

Therefore, the stale `SuperInstance` URLs on version 2.2.0 cannot be corrected
retroactively.

## 3. What is required to fix it?

The fix must be a new patch release, for example **2.2.1**, whose only effective
change is the corrected `repository` / `homepage` metadata already present in
`Cargo.toml` on `main`. The steps are:

1. Ensure `Cargo.toml` on the release branch contains the correct
   `purplepincher/constraint-theory-core` URLs (already true on `main`).
2. Bump `version` from `2.2.0` to `2.2.1` (a maintainer action requiring
   crates.io credentials).
3. Publish the new version with `cargo publish`.
4. Optionally yank 2.2.0 if the organization wants to discourage use of the
   version with stale metadata, though existing dependents will keep working.

No code changes are strictly necessary for a metadata-only patch, but the
version number *must* change because crates.io refuses to overwrite an existing
version.

## 4. Recommendation

* **Do not change `Cargo.toml`'s `version` field in an automated tool run.**
  Version bumps and `cargo publish` require the maintainer's crates.io API
  token.
* **Prepare and publish `constraint-theory-core 2.2.1`** with the metadata
  already corrected in source control. This is the only supported way to make
  crates.io display the `purplepincher` repository URL.
