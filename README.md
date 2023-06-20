# Example semver violations in Rust

Each example contains two directories: `old` and `new`. Imagine that `old` is the previous version,
and `new` is the next version of the crate.

Can you find the semver violation that happened between them?

| example   | previous version | next version | diff |
| --------- | ---------------- | ------------ | ---- |
| `easy_01` | [link](https://github.com/obi1kenobi/semver-examples/blob/main/easy_01/old/src/lib.rs) | [link](https://github.com/obi1kenobi/semver-examples/blob/main/easy_01/new/src/lib.rs) | [diff](https://github.com/obi1kenobi/semver-examples/compare/easy_01) |
| `med_01`  | [link](https://github.com/obi1kenobi/semver-examples/blob/main/med_01/old/src/lib.rs) | [link](https://github.com/obi1kenobi/semver-examples/blob/main/med_01/new/src/lib.rs) | [diff](https://github.com/obi1kenobi/semver-examples/compare/med_01) |
| `med_02`  | [link](https://github.com/obi1kenobi/semver-examples/blob/main/med_02/old/src/lib.rs) | [link](https://github.com/obi1kenobi/semver-examples/blob/main/med_02/new/src/lib.rs) | [diff](https://github.com/obi1kenobi/semver-examples/compare/med_02) |
| `hard_01` | [link](https://github.com/obi1kenobi/semver-examples/blob/main/hard_01/old/src/lib.rs) | [link](https://github.com/obi1kenobi/semver-examples/blob/main/hard_01/new/src/lib.rs) | [diff](https://github.com/obi1kenobi/semver-examples/compare/hard_01) |

#### License

<sup>
Available under the <a href="LICENSE-APACHE">Apache License, Version
2.0</a>.
</sup>

<br>

<sup>
Copyright 2023-present Predrag Gruevski and Contributors.
</sup>

<br>

<sub>
Contributors are defined in the Apache-2.0 license.
The present date is determined by the timestamp of the most recent commit in the repository.
</sub>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
