use leptos::prelude::*;

#[component]
pub fn DownloadSection() -> impl IntoView {
    view! {
        <section class="download-section" id="download">
            <div class="section-label">"// get started"</div>
            <h2>"Download "<span>"for free"</span></h2>
            <p style="color: var(--muted); margin-top: 1rem; max-width: 55ch; font-size: 0.95rem; line-height: 1.7;">"Windows binaries are ready to go. The release package includes the YuNet ONNX model — no manual setup needed."</p>

            <div class="download-grid">
                <a href="https://github.com/gregorycarnegie/iron_cropper/releases/latest" target="_blank" class="download-card reveal">
                    <div class="dl-icon">"📦"</div>
                    <h3>"MSI Installer"</h3>
                    <p>"Enterprise-style installer for Windows x86-64. Clean install and uninstall support."</p>
                    <span class="dl-tag">"face-crop-studio-windows-x86_64.msi"</span>
                </a>
                <a href="https://github.com/gregorycarnegie/iron_cropper/releases/latest" target="_blank" class="download-card reveal">
                    <div class="dl-icon">"🔧"</div>
                    <h3>"Setup Executable"</h3>
                    <p>"Traditional Windows installer with guided setup wizard."</p>
                    <span class="dl-tag">"face-crop-studio-windows-x86_64-setup.exe"</span>
                </a>
                <a href="https://github.com/gregorycarnegie/iron_cropper/releases/latest" target="_blank" class="download-card reveal">
                    <div class="dl-icon">"🗜️"</div>
                    <h3>"Portable ZIP"</h3>
                    <p>"No install needed. Extract anywhere and run. Ideal for servers and restricted environments."</p>
                    <span class="dl-tag">"face-crop-studio-windows-x86_64.zip"</span>
                </a>
                <a href="https://github.com/gregorycarnegie/iron_cropper" target="_blank" class="download-card reveal">
                    <div class="dl-icon">"🦀"</div>
                    <h3>"Build from Source"</h3>
                    <p>"Full Rust workspace. Requires NASM on PATH. Linux and macOS supported via source build."</p>
                    <span class="dl-tag">"cargo build --release --workspace"</span>
                </a>
            </div>
        </section>
    }
}
