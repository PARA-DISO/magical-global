# magical-global

>[!TIP]
> It is a shortcut when sharing variables like C code with ease,
> and is not a good technique from the Rust perspective.

Magical Data Sharing for Rust

## Usage

```rs
use magical_global as magical;

fn main() {
  let data:usize = 0;
  // Set data to magical global
  if magical::set_at(Box::new(data), 0).is_err() {
      println!("failed to set data");
      return;
  };
  // read global
  let shared_data = magical::get_mut_at::<usize>(0).unwrap();
  // take ownership of data
  let data = magical::take_at::<usize>(0).unwrap();
}
```

Read the example code to learn more.

## Limitations

* The possible areas where data can be placed are from 0 to 31.
* This crate has no memory management features.
  - Recommend using the `take_at` function to free memory with the rust function.

## Examples

Example code can be executed with `cargo run --example sharing` or `cargo run --example locking`

`sharing` is a simple data share, and `locking` is an Example of Mutex Lock code.