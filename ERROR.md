error[E0038]: the trait `registry::RegisterLoader` is not dyn compatible
  --> src/registry/mod.rs:46:53
   |
46 | pub fn registry_from_name(name: &str) -> Result<Box<dyn RegisterLoader>> {
   |                                                     ^^^^^^^^^^^^^^^^^^ `registry::RegisterLoader` is not dyn compatible
   |
note: for a trait to be dyn compatible it needs to allow building a vtable
      for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
  --> src/registry/mod.rs:19:8
   |
17 | pub trait RegisterLoader {
   |           -------------- this trait is not dyn compatible...
18 |     /// Get the name of the register
19 |     fn get_register_name() -> &'static str;
   |        ^^^^^^^^^^^^^^^^^ ...because associated function `get_register_name` has no `self` parameter
...
22 |     fn load(base_path: &str, pnr_filter: Option<&HashSet<String>>) -> Result<Vec<RecordBatch>>;
   |        ^^^^ ...because associated function `load` has no `self` parameter
   = help: the following types implement `registry::RegisterLoader`:
             registry::akm::AkmRegister
             registry::bef::BefRegister
             registry::ind::IndRegister
             registry::uddf::UddfRegister
           consider defining an enum where each variant holds one of these types,
           implementing `registry::RegisterLoader` for this new enum and using it instead
   = note: `registry::RegisterLoader` may be implemented in other crates; if you want to support your users passing their own types here, you can't refer to a specific type
help: consider turning `load` into a method by giving it a `&self` argument
   |
22 |     fn load(&self, base_path: &str, pnr_filter: Option<&HashSet<String>>) -> Result<Vec<RecordBatch>>;
   |             ++++++
help: alternatively, consider constraining `load` so it does not apply to trait objects
   |
22 |     fn load(base_path: &str, pnr_filter: Option<&HashSet<String>>) -> Result<Vec<RecordBatch>> where Self: Sized;
   |                                                                                                +++++++++++++++++
help: consider turning `get_register_name` into a method by giving it a `&self` argument
   |
19 |     fn get_register_name(&self) -> &'static str;
   |                          +++++
help: alternatively, consider constraining `get_register_name` so it does not apply to trait objects
   |
19 |     fn get_register_name() -> &'static str where Self: Sized;
   |                                            +++++++++++++++++
