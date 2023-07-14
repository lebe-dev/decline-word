# decline word

**1. Add dependency:**

```
cargo add decline-word
```

**2. Use:**

```rust
let value = decline_for_num(12412, &("минута", "минуты", "минут"));
println!("{value}");

let value = decline_for_num(387222, &("рубль", "рубля", "рублей"));
println!("{value}");
```

Output:

```shell
минут
рубля
```