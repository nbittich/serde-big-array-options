# serde-big-array-options

Simpler alternative to [serde_with](https://crates.io/crates/serde_with).
If you need something more than `[Option<T>;N]`, use that instead.

### Usage

```rust
const DECK_SIZE:usize = 52;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Game {
    #[serde(
        serialize_with = "serde_big_array_options::serialize",
        deserialize_with = "serde_big_array_options::deserialize"
    )]
    pub back_in_deck: [Option<usize>; DECK_SIZE],
}

```
