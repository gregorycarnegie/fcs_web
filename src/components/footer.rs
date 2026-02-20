use leptos::prelude::*;

#[component]
pub fn FooterSection() -> impl IntoView {
    view! {
        <footer>
            <div class="footer-logo">"Face"<span>"Crop"</span>" Studio"</div>
            <ul class="footer-links">
                <li><a href="https://github.com/gregorycarnegie/iron_cropper" target="_blank" rel="noopener noreferrer">"GitHub"</a></li>
                <li><a href="https://github.com/gregorycarnegie/iron_cropper/releases" target="_blank" rel="noopener noreferrer">"Releases"</a></li>
                <li><a href="https://github.com/gregorycarnegie/iron_cropper/issues" target="_blank" rel="noopener noreferrer">"Issues"</a></li>
                <li><a href="https://github.com/gregorycarnegie/iron_cropper/blob/master/README.md" target="_blank" rel="noopener noreferrer">"Docs"</a></li>
            </ul>
            <span class="footer-copy">"MIT License · Built with 🦀 Rust"</span>
        </footer>
    }
}
