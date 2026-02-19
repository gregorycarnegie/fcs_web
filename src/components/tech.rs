use leptos::prelude::*;

#[component]
pub fn TechSection() -> impl IntoView {
    let presets = vec![
        "yunet-core",
        "yunet-utils",
        "yunet-gui",
        "yunet-cli",
        "tract-onnx",
        "wgpu/WGSL",
        "eframe/egui",
    ];
    let (active_preset, set_active_preset) = signal(presets[0].to_string());

    view! {
        <section class="tech-section">
            <div class="tech-inner">
                <div>
                    <div class="section-label">"// architecture"</div>
                    <h2>"Built for "<span>"performance"</span></h2>
                    <p style="color: var(--muted); margin-top: 1.5rem; line-height: 1.75; font-size: 0.95rem;">"Face Crop Studio is a Rust workspace with a clean crate separation: "<strong style="color: var(--text);">"yunet-core"</strong>" handles detection and crop maths, "<strong style="color: var(--text);">"yunet-utils"</strong>" provides shared helpers, and "<strong style="color: var(--text);">"yunet-gui"</strong>" / "<strong style="color: var(--text);">"yunet-cli"</strong>" are the surfaces."</p>
                    <p style="color: var(--muted); margin-top: 1rem; line-height: 1.75; font-size: 0.95rem;">"GPU context pooling means the CLI handles batch jobs efficiently, and the GUI shares the wgpu context with eframe's renderer for zero-overhead compute."</p>
                    <div class="presets-scroll" style="margin-top: 2rem;">
                        <For
                            each=move || presets.clone().into_iter()
                            key=|preset| *preset
                            children=move |preset| {
                                let selected = move || active_preset.get() == preset;
                                view! {
                                    <span
                                        class=move || if selected() { "preset-pill active" } else { "preset-pill" }
                                        on:click=move |_| set_active_preset.set(preset.to_string())
                                    >
                                        {preset}
                                    </span>
                                }
                            }
                        />
                    </div>
                </div>
                <div>
                    <div class="code-block">
                        <div class="code-header">"yunet-cli — batch example"</div>
                        <pre style="white-space: pre-wrap; margin: 0;">
"# Batch crop to LinkedIn preset with GPU
yunet-cli \
  --input ./photos/ \
  --preset LinkedIn \
  --face-height 60 \
  --gpu \
  --enhance \
  --quality-threshold Medium \
  --output ./crops/

# Mapping-driven batch from Excel
yunet-cli \
  --mapping-file roster.xlsx \
  --mapping-source photo_path \
  --mapping-output employee_id \
  --preset IDCard \
  --output ./id_crops/

# Benchmark GPU vs CPU preprocessing
yunet-cli --benchmark-preprocess"
                        </pre>
                    </div>
                </div>
            </div>
        </section>
    }
}
