use leptos::prelude::*;

#[derive(Clone, Copy)]
struct FeatureItem {
    icon: &'static str,
    title: &'static str,
    description: &'static str,
}

const FALLBACK_FEATURE_ITEMS: [FeatureItem; 1] = [FeatureItem {
    icon: "ℹ️",
    title: "Configuration Pending",
    description: "Feature data is currently unavailable. Please check back shortly.",
}];

const FEATURE_ITEMS: [FeatureItem; 9] = [
    FeatureItem {
        icon: "🎯",
        title: "Crop Presets",
        description: "LinkedIn, Passport, Instagram, ID Card, Avatar, Headshot — and fully custom dimensions. Each preset nails the correct aspect ratio automatically.",
    },
    FeatureItem {
        icon: "⚡",
        title: "GPU Acceleration",
        description: "Full wgpu/WGSL pipeline with custom compute shaders for preprocessing, enhancement, and even YuNet inference. Automatic CPU fallback when needed.",
    },
    FeatureItem {
        icon: "📊",
        title: "Quality Scoring",
        description: "Laplacian-variance sharpness scoring categorises crops as Low, Medium, or High quality. Auto-select the sharpest face in batch jobs.",
    },
    FeatureItem {
        icon: "🎨",
        title: "Enhancement Pipeline",
        description: "Auto colour, exposure, brightness, contrast, saturation, sharpening, skin smoothing, red-eye removal, and portrait background blur — CPU and GPU paths.",
    },
    FeatureItem {
        icon: "📁",
        title: "Mapping Workflows",
        description: "Import CSV, Excel, Parquet, or SQLite datasets to drive batch naming. Feed in thousands of source→output mappings in one go.",
    },
    FeatureItem {
        icon: "🖥️",
        title: "GUI + CLI",
        description: "Native egui desktop app with live preview, undo/redo, and history. Plus a full-featured CLI for scripting and automation workflows.",
    },
    FeatureItem {
        icon: "📐",
        title: "Positioning Modes",
        description: "Centre, Rule of Thirds, or fully custom offsets with pixel-precise keyboard nudges and comprehensive undo/redo support.",
    },
    FeatureItem {
        icon: "🏷️",
        title: "Metadata Control",
        description: "Preserve, strip, or customise image metadata. Export to PNG, JPEG with quality controls, or WebP — all with accurate metadata handling.",
    },
    FeatureItem {
        icon: "🔋",
        title: "Batch Processing",
        description: "Queue hundreds of images, track status per-file, derive filenames from templates, and get structured JSON/CSV failure reports for every job.",
    },
];

#[allow(clippy::const_is_empty)]
fn resolved_feature_items() -> Vec<FeatureItem> {
    if FEATURE_ITEMS.is_empty() {
        FALLBACK_FEATURE_ITEMS.to_vec()
    } else {
        FEATURE_ITEMS.to_vec()
    }
}

#[component]
pub fn FeaturesSection() -> impl IntoView {
    let feature_items = resolved_feature_items();

    view! {
        <section id="features">
            <div class="section-label">"// capabilities"</div>
            <h2>"Everything you need for "<span>"perfect crops"</span></h2>
            <div class="features-grid">
                <For
                    each=move || feature_items.clone().into_iter()
                    key=|item| item.title
                    children=move |item| {
                        view! {
                            <div class="feature-card">
                                <div class="feature-icon">{item.icon}</div>
                                <h3>{item.title}</h3>
                                <p>{item.description}</p>
                            </div>
                        }
                    }
                />
            </div>
        </section>
    }
}
