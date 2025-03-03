Run PyO3/maturin-action@v1
Found maturin version requirement maturin>=1.0,<2.0 specified in pyproject.toml
Found maturin release from manifest: v1.8.2
Install Rust target
Install maturin
/Users/runner/work/_temp/b95b2fe5-1bd2-4516-b480-0e879da87b02/maturin build --release --out dist -b bin
error: failed to load manifest for workspace member `/Users/runner/work/ids-rs/ids-rs/crates/ids`
referenced by workspace at `/Users/runner/work/ids-rs/ids-rs/Cargo.toml`

Caused by:
failed to read `/Users/runner/work/ids-rs/ids-rs/crates/ids/Cargo.toml`

Caused by:
No such file or directory (os error 2)
💥 maturin failed
Caused by: Cargo metadata failed. Does your crate compile with `cargo build`?
Caused by: `cargo metadata` exited with an error:
Error: The process '/Users/runner/work/_temp/b95b2fe5-1bd2-4516-b480-0e879da87b02/maturin' failed with exit code 1
at ExecState._setResult (/Users/runner/work/_actions/PyO3/maturin-action/v1/dist/index.js:1702:25)
at ExecState.CheckComplete (/Users/runner/work/_actions/PyO3/maturin-action/v1/dist/index.js:1685:18)
at ChildProcess.<anonymous> (/Users/runner/work/_actions/PyO3/maturin-action/v1/dist/index.js:1579:27)
at ChildProcess.emit (node:events:519:28)
at maybeClose (node:internal/child_process:1105:16)
at ChildProcess._handle.onexit (node:internal/child_process:305:5)
Error: The process '/Users/runner/work/_temp/b95b2fe5-1bd2-4516-b480-0e879da87b02/maturin' failed with exit code 1
