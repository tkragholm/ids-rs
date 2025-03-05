unused dependencies:
`core v1.2.5 (/Users/tobiaskragholm/dev/ids-rs/crates/core)`
└─── dependencies
├─── "bit-vec"
├─── "mimalloc"
├─── "ndarray"
└─── "rand_distr"
`loader v1.2.5 (/Users/tobiaskragholm/dev/ids-rs/crates/loader)`
└─── dependencies
└─── "indicatif-log-bridge"
`types v1.2.5 (/Users/tobiaskragholm/dev/ids-rs/crates/types)`
└─── dependencies
└─── "parking_lot"
`utils v1.2.5 (/Users/tobiaskragholm/dev/ids-rs/crates/utils)`
└─── dependencies
├─── "anyhow"
└─── "tempfile"
Note: These dependencies might be used by other targets.
To find dependencies that are not used by any target, enable `--all-targets`.
Note: They might be false-positive.
For example, `cargo-udeps` cannot detect usage of crates that are only used in doc-tests.
To ignore some dependencies, write `package.metadata.cargo-udeps.ignore` in Cargo.toml.
