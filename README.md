# LuckPerms-rs
This is developed for personal use, and thus I will not be guaranteeing any support for this project.  
Feel free to create pull requests if you want to contribute.

This library is a full implementation of the [LuckPerms REST API](https://luckperms.net/wiki/Standalone-and-REST-API) in Rust.  
Documentation is limited, but it should be fairly easy to use.

## Usage
Add the following to your `Cargo.toml`:
```toml
[dependencies]
luckperms-rs = "0.1.0"
```

## Example
```rust
use luckperms_rs::LuckPerms;

#[tokio::main]
async fn main() {
    let luckperms = LuckPerms::new("http://localhost:8080", "key").await.unwrap();
    let user = luckperms.get_user("uuid").await.unwrap();
    println!("{:?}", user);
}
```
