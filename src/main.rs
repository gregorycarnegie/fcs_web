#[cfg(target_arch = "wasm32")]
fn main() {
    leptos::mount::mount_to_body(fcs_web::App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("This Leptos app targets wasm32. Build with trunk or leptos tooling.");
}
