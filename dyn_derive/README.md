# dyn_derive

```rust
#[dyn_impl]
pub trait Meta: Debug + DynClone + DynPartialEq {}

#[derive(Debug, Clone, PartialEq)]
pub struct MetaImpl;

impl Meta for MetaImpl {}

#[derive(Debug, Clone, PartialEqFix)]
pub struct Bar {
    meta: Box<dyn Meta>,
}
```
