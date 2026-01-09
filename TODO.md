# TODO 2026-01-09

- add filter to only consider "new" versions that are "stable" where
  the definition of stable if that of version strictly matching semver
  of numbers and dots only.
  - for instance, what prompted this todo entry is that running `cargo upgrade` in a rust project with the dependency `chacha20 = { version = "0.9.1", features = ["std"] }` caused the `chacha20` dependency to be bumped to `chacha20 = { version = "0.10.0-rc.6", features = ["std"] }` which appears to be a release-canditate and, at any rate, caused the dependent project (mu-cli) to fail with the following output:

```bash
    Updating crates.io index
error: failed to select a version for `chacha20`.
    ... required by package `mu-cli v0.1.0 (/Users/gabrielfalcao/projects/personal/mu/src/cli)`
versions that meet the requirements `^0.10.0-rc.6` are: 0.10.0-rc.6

package `mu-cli` depends on `chacha20` with feature `std` but `chacha20` does not have that feature.
 package `chacha20` does have feature `rng`


failed to select a version for `chacha20` which could resolve this conflict
```
