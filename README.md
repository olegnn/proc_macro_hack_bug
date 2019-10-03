## Problem

Can't invoke nested macro without increasing `internal_macro_calls`. In test example this value in `join_all!` macro attribute is `5` because in code 

```rust
let results = join_all!(ready(join_all!(ready(7usize)).0), ready(join_all!(ready(8usize)).0), ready(join_all!(ready(join_all!(ready(9usize)).0)).0));
```

we have `5` calls of `join_all!` macro each of which inserts 1 nested `join!` macro. Of course this problem can be solved by setting `internal_macro_calls` to maximum value, but...

Unfortunately, in situation with `join_all_x2!` even value of `60` doesn't help.

__Run tests:__

```shell
cargo test
```

__Expand macros:__ 

```shell
cargo rustc --profile=check -- -Zunstable-options --pretty=expanded -Z external-macro-backtrace
```
