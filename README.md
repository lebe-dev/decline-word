# decline word

How to use:

```rust
let value = decline_for_num(12412, &("минута", "минуты", "минут"));
println!("{value}");

let value = decline_for_num(387222, &("рубль", "рубля", "рублей"));
println!("{value}");
```

Example output 1 & 2:

```shell
минут
рубля
```