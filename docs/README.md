# documentation

This is documentation directory which every crate and functionality will be explained for the purpose of understanding it better, and allowing future contributors to understand and be able to add to the project.

## Syntax

The idea would be to try to make it as close to jsx as possible, however that is not possible without using macros.

The primary way the framework could work is showcased below:

```rust
#[zeal_component()]
pub fn App() -> Rsx {
    rsx! {
        <div>
            <p>Hello, World!</p>
        </div>
    }
}

fn main() {
    zeal::mount(App);
}
```
