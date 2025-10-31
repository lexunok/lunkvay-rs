use leptos::prelude::*;
use stylance::import_style;

import_style!(style, "spinner.module.scss");

#[component]
pub fn Spinner() -> impl IntoView {
    view! {
        <div class=style::spinner></div>
    }
}
