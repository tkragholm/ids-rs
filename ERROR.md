error[E0015]: cannot perform non-const deref coercion on `std::string::String` in constant functions
    --> src/model/pnr.rs:87:9
     |
87   |         &self.value
     |         ^^^^^^^^^^^
     |
     = note: attempting to deref into `str`
note: deref defined here
    --> /home/tkragholm/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2686:5
     |
2686 |     type Target = str;
     |     ^^^^^^^^^^^
note: impl defined here, but it is not `const`
    --> /home/tkragholm/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2685:1
     |
2685 | impl ops::Deref for String {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants

error[E0015]: cannot perform non-const deref coercion on `std::string::String` in constant functions
    --> src/model/covariate.rs:37:46
     |
37   |             Self::Categorical(value) => Some(value),
     |                                              ^^^^^
     |
     = note: attempting to deref into `str`
note: deref defined here
    --> /home/tkragholm/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2686:5
     |
2686 |     type Target = str;
     |     ^^^^^^^^^^^
note: impl defined here, but it is not `const`
    --> /home/tkragholm/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2685:1
     |
2685 | impl ops::Deref for String {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants

error[E0015]: cannot perform non-const deref coercion on `std::string::String` in constant functions
    --> src/model/covariate.rs:93:9
     |
93   |         &self.name
     |         ^^^^^^^^^^
     |
     = note: attempting to deref into `str`
note: deref defined here
    --> /home/tkragholm/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2686:5
     |
2686 |     type Target = str;
     |     ^^^^^^^^^^^
note: impl defined here, but it is not `const`
    --> /home/tkragholm/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2685:1
     |
2685 | impl ops::Deref for String {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants

error: aborting due to 3 previous errors; 14 warnings emitted

For more information about this error, try `rustc --explain E0015`.
Original diagnostics will follow.

warning: strict comparison of `f32` or `f64`
  --> src/utils/string_utils/parsing.rs:85:9
   |
85 |         assert_eq!(parse_f64("123.45").unwrap(), 123.45);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#float_cmp
   = note: `-W clippy::float-cmp` implied by `-W clippy::pedantic`
   = help: to override `-W clippy::pedantic` add `#[allow(clippy::float_cmp)]`
   = note: this warning originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: strict comparison of `f32` or `f64`
  --> src/utils/string_utils/parsing.rs:86:9
   |
86 |         assert_eq!(parse_f64(" 456.78 ").unwrap(), 456.78);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#float_cmp
   = note: this warning originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: `ids-rs` (lib test) generated 380 warnings (378 duplicates)
