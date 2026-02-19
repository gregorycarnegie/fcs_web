use leptos::prelude::*;

#[component]
pub fn StatsBar() -> impl IntoView {
    view! {
        <div class="stats-bar">
            <div class="stat"><span class="stat-val">"7"</span><span class="stat-label">"GPU Compute Shaders"</span></div>
            <div class="stat"><span class="stat-val">"6+"</span><span class="stat-label">"Crop Presets"</span></div>
            <div class="stat"><span class="stat-val">"4"</span><span class="stat-label">"Export Formats"</span></div>
            <div class="stat"><span class="stat-val">"97%"</span><span class="stat-label">"Rust Codebase"</span></div>
        </div>
    }
}
