# documentation

This is documentation directory which every crate and functionality will be explained for the purpose of understanding it better, and allowing future contributors to understand and be able to add to the project.

## Syntax

The idea would be to try to make it as close to jsx as possible, however that is not possible without using macros.

The primary two ways the framework could work are showcased below:

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

or more monadic:

```rust
#[zeal_component()]
pub fn App() -> Node {
    let root = zeal::root();
    let component = root.children(
        div().children(
            p().children("Hello, World!")
        )
    );

    component
}

fn main() {
    zeal::mount(App);
}
```
