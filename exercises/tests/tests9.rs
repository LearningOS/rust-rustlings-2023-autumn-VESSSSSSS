extern "Rust" {
    unknown command: extern "Rust" {
fn my_demo_function(a: u32) -> u32;
    fnunknown command: fn my_demo_function(a: u32) -> u32;
 my_demo_function_alias(a: u32) -> u32;
}unknown command: fn my_demo_function_alias(a: u32) -> u32;


mod Funknown command: }
unknown command:
oo {
 unknown command: mod Foo {
   // Add #[no_mangle] to prevent name mangling
    #unknown command: // Add #[no_mangle] to prevent name mangling
[no_mangle]
  unknown command: #[no_mangle]
  pub fn my_demo_function(a: u32) -> u32 {
      unknown command: pub fn my_demo_function(a: u32) -> u32 {
  a
    unknown command: a
}
}

#unknown command: }
unknown command: }
unknown command:
[cfg(test)]
munknown command: #[cfg(test)]
od tests {
    unknown command: mod tests {
use super::*;

    #unknown command: use super::*;
unknown command:
[test]
 unknown command: #[test]
   fn test_success() {
     unknown command: fn test_success() {
   // The externally imported functions are UNSAFE by default
   unknown command: // The externally imported functions are UNSAFE by default
     // because of untrusted source of other languages. You may
       unknown command: // because of untrusted source of other languages. You may
 // wrap them in safe Rust APIs to ease the burden of callers.
     unknown command: // wrap them in safe Rust APIs to ease the burden of callers.
   //
  unknown command: //
      // SAFETY: We know those functions are aliases of a safe
 unknown command: // SAFETY: We know those functions are aliases of a safe
       // Rust function.
      unknown command: // Rust function.
  unsafe {
    unknown command: unsafe {
        my_demo_function(123);
     unknown command: my_demo_function(123);
       my_demo_function_alias(456);
       unknown command: my_demo_function_alias(456);
 }