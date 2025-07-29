use std::rc::Rc;

use leptos::{component, ev, view, CollectView, IntoView, View};

#[component]
pub fn Dropdown(
    name: &'static str,
    #[prop(into)] options: Rc<[&'static str]>,
    select_option: impl Fn(&'static str) + Copy + 'static,
) -> impl IntoView {
    let options_view = move || -> View {
        options
            .iter()
            .map(|name| {
                view! {
                    <Option name=name select_option=select_option />
                }
            })
            .collect_view()
    };

    view! {
        <div class="dropdown">
            <button class="button dropdown-button">
                {name}" "
                <i class="dropdown-chevron">
                    <svg xmlns="http://www.w3.org/2000/svg" width="13" height="9" viewBox="0 0 13 9" fill="none">
                    <path d="M7.72437 8.04315C6.95677 8.75674 5.76848 8.75674 5.00088 8.04315L1.07448 4.393C-0.256421 3.15574 0.619052 0.928198 2.43622 0.928199L10.289 0.928199C12.1062 0.928199 12.9817 3.15574 11.6508 4.393L7.72437 8.04315Z" fill="white"/>
                    </svg>
                </i>
            </button>
            <div class="dropdown-content">
                {options_view}
            </div>
        </div>
    }
}

#[component]
fn Option(name: &'static str, select_option: impl Fn(&'static str) + 'static) -> impl IntoView {
    let button_click = move |_event: ev::MouseEvent| select_option(name);
    view! {
        <button
            class="action-button"
            on:click=button_click
        >
            {name}
        </button>
    }
}
