use leptos::prelude::*;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav aria-label="Primary">
            <div class="nav-logo">"Face"<span>"Crop"</span>" Studio"</div>
            <ul>
                <li><a href="#features">"Features"</a></li>
                <li><a href="#download">"Download"</a></li>
                <li><a href="#donate">"Donate"</a></li>
                <li><a href="https://github.com/gregorycarnegie/iron_cropper" target="_blank" rel="noopener noreferrer">"GitHub"</a></li>
                <li><a href="#download" class="nav-cta">"Get it Free"</a></li>
            </ul>
        </nav>
    }
}
