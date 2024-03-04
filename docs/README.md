# documentation

This is documentation directory which every crate and functionality will be explained for the purpose of understanding it better, and allowing future contributors to understand and be able to add to the project.

## Syntax

The idea would be to try to make it as close to jsx as possible, without using macros (since that's what yew and leptos do already) - and see if there's a way to make it work with the rust compiler.

There are a few ways the syntax for rsx could be implemented, the two primary ones are:

```rust
fn Page() -> rsx::Html {
    rsx! {
        <div>
            <p> Hello, world! </p>
        </div>
    }
}

fn main() {
    rsx::mount(Page).serve();
}
```

or it could somehow be exactly like jsx:

```rust
fn Page() {
    <div>
        <p> Hello, world! </p>
    </div>
}

fn main() {
    rsx::mount(Page).serve();
}
```
