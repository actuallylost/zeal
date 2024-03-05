// Below should render a page that says "Hello, World!" in a paragraph tag
use zeal::prelude::*;

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
