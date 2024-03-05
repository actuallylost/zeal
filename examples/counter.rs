// Below should create a counter that updates every time a button is pressed
use zeal::prelude::*;

#[zeal_component()]
pub fn App() -> Rsx {
    // Creates count.get() and count.set() functions to access and set state value.
    let count = state(0);

    // get_current waits for a value to be updated before fetching it, so it's always updated
    rsx! {
        <button onclick={count.set(count.get() + 1)}>
            Click me: {count.get_current()}
        </button>
    }
}

fn main() {
    zeal::mount(App);
}
