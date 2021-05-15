# project description.
the name register service can be used to register a vanity name.
to avoid malicious node censoring transaction, there are two steps:

1. send transaction to register the name's hash
2. send transaction to claim the name

## run test case as following, the latest nightly rustc has a panic.
cargo +stable test