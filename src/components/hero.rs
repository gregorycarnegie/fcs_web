use leptos::prelude::*;

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <section class="hero">
            <div class="hero-grid"></div>
            <div class="hero-glow"></div>
            <div class="hero-glow-2"></div>
            <div class="hero-eyebrow">"// yunet-powered · rust-built · gpu-accelerated"</div>
            <h1>"Face "<em>"Crop"</em>" Studio"</h1>
            <p class="hero-sub">
                "Precision face detection and cropping with GPU-accelerated batch processing. LinkedIn headshots, passports, IDs — deterministic, high-quality, blazing fast."
            </p>
            <div class="hero-badges">
                <span class="badge rust">"🦀 Rust"</span>
                <span class="badge">"WGSL Compute Shaders"</span>
                <span class="badge">"YuNet Face Detection"</span>
                <span class="badge">"GPU + CPU"</span>
                <span class="badge">"MIT License"</span>
            </div>
            <div class="hero-actions">
                <a href="#download" class="btn btn-primary">"⬇ Download for Windows"</a>
                <a href="https://github.com/gregorycarnegie/iron_cropper" target="_blank" rel="noopener noreferrer" class="btn btn-outline">"View on GitHub →"</a>
            </div>
        </section>
    }
}
