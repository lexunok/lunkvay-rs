mod app;
mod components;
mod config;
mod models;
mod pages;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App)
}
