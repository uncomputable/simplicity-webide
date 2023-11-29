mod components;
mod exec;
mod instruction;
mod util;

use components::App;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}