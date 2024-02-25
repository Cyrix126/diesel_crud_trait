# Macro Derive

```rust,ignore
#[derive(CrudAble)]
```

The derive trait will automaticlly implement the trait CrudAble.
```
#[crud(method=bool)]
```
Some methods can be disabled to prevent their use (will return an error).
