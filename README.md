<h1 align="center">Polychrome</h1>

<p align="center">
  <img src="https://raw.githubusercontent.com/EliasLeguizamon123/polychrome/main/logo.png?raw=true" alt="drawing" style="width:300px;"/>
</p>

<p align="center">Polychome is a crate for printing styled text in your terminal.</p>

## Usage

Just add to your `Cargo.toml`

```toml
[dependencies]
polychrome = "2.0.2"
```

an then just use it! 

```rust
use polychrome::ColorPrintExt;

fn main() {
    println!("{}", "Hello, world!".color(255, 0, 0).underline(None));
    println!("{}", "Hello, world!".color(0, 0, 255).underline(Some("stripe")));
}
```
