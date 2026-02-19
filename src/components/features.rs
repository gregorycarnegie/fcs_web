use leptos::prelude::*;

#[component]
pub fn FeaturesSection() -> impl IntoView {
    view! {
        <section id="features">
            <div class="section-label">"// capabilities"</div>
            <h2>"Everything you need for "<span>"perfect crops"</span></h2>
            <div class="features-grid">
                <div class="feature-card"><div class="feature-icon">"🎯"</div><h3>"Crop Presets"</h3><p>"LinkedIn, Passport, Instagram, ID Card, Avatar, Headshot — and fully custom dimensions. Each preset nails the correct aspect ratio automatically."</p></div>
                <div class="feature-card"><div class="feature-icon">"⚡"</div><h3>"GPU Acceleration"</h3><p>"Full wgpu/WGSL pipeline with custom compute shaders for preprocessing, enhancement, and even YuNet inference. Automatic CPU fallback when needed."</p></div>
                <div class="feature-card"><div class="feature-icon">"📊"</div><h3>"Quality Scoring"</h3><p>"Laplacian-variance sharpness scoring categorises crops as Low, Medium, or High quality. Auto-select the sharpest face in batch jobs."</p></div>
                <div class="feature-card"><div class="feature-icon">"🎨"</div><h3>"Enhancement Pipeline"</h3><p>"Auto colour, exposure, brightness, contrast, saturation, sharpening, skin smoothing, red-eye removal, and portrait background blur — CPU and GPU paths."</p></div>
                <div class="feature-card"><div class="feature-icon">"📁"</div><h3>"Mapping Workflows"</h3><p>"Import CSV, Excel, Parquet, or SQLite datasets to drive batch naming. Feed in thousands of source→output mappings in one go."</p></div>
                <div class="feature-card"><div class="feature-icon">"🖥️"</div><h3>"GUI + CLI"</h3><p>"Native egui desktop app with live preview, undo/redo, and history. Plus a full-featured CLI for scripting and automation workflows."</p></div>
                <div class="feature-card"><div class="feature-icon">"📐"</div><h3>"Positioning Modes"</h3><p>"Centre, Rule of Thirds, or fully custom offsets with pixel-precise keyboard nudges and comprehensive undo/redo support."</p></div>
                <div class="feature-card"><div class="feature-icon">"🏷️"</div><h3>"Metadata Control"</h3><p>"Preserve, strip, or customise image metadata. Export to PNG, JPEG with quality controls, or WebP — all with accurate metadata handling."</p></div>
                <div class="feature-card"><div class="feature-icon">"🔋"</div><h3>"Batch Processing"</h3><p>"Queue hundreds of images, track status per-file, derive filenames from templates, and get structured JSON/CSV failure reports for every job."</p></div>
            </div>
        </section>
    }
}
