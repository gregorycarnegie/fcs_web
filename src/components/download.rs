use leptos::prelude::*;

#[derive(Clone, Copy)]
struct DownloadCard {
    href: &'static str,
    aria_label: &'static str,
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    tag: &'static str,
}

const FALLBACK_DOWNLOAD_CARDS: [DownloadCard; 1] = [DownloadCard {
    href: "https://github.com/gregorycarnegie/iron_cropper/releases/latest",
    aria_label: "Open latest Face Crop Studio releases",
    icon: "ℹ️",
    title: "Releases Unavailable",
    description: "Download metadata is currently unavailable. Open releases for the latest binaries.",
    tag: "Check latest release artifacts",
}];

const DOWNLOAD_CARDS: [DownloadCard; 4] = [
    DownloadCard {
        href: "https://github.com/gregorycarnegie/iron_cropper/releases/latest",
        aria_label: "Open latest GitHub release for MSI installer",
        icon: "📦",
        title: "MSI Installer",
        description: "Enterprise-style installer for Windows x86-64. Clean install and uninstall support.",
        tag: "face-crop-studio-windows-x86_64.msi",
    },
    DownloadCard {
        href: "https://github.com/gregorycarnegie/iron_cropper/releases/latest",
        aria_label: "Open latest GitHub release for setup executable",
        icon: "🔧",
        title: "Setup Executable",
        description: "Traditional Windows installer with guided setup wizard.",
        tag: "face-crop-studio-windows-x86_64-setup.exe",
    },
    DownloadCard {
        href: "https://github.com/gregorycarnegie/iron_cropper/releases/latest",
        aria_label: "Open latest GitHub release for portable zip",
        icon: "🗜️",
        title: "Portable ZIP",
        description: "No install needed. Extract anywhere and run. Ideal for servers and restricted environments.",
        tag: "face-crop-studio-windows-x86_64.zip",
    },
    DownloadCard {
        href: "https://github.com/gregorycarnegie/iron_cropper",
        aria_label: "Open Face Crop Studio source repository on GitHub",
        icon: "🦀",
        title: "Build from Source",
        description: "Full Rust workspace. Requires NASM on PATH. Linux and macOS supported via source build.",
        tag: "cargo build --release --workspace",
    },
];

#[allow(clippy::const_is_empty)]
fn resolved_download_cards() -> Vec<DownloadCard> {
    if DOWNLOAD_CARDS.is_empty() {
        FALLBACK_DOWNLOAD_CARDS.to_vec()
    } else {
        DOWNLOAD_CARDS.to_vec()
    }
}

#[component]
pub fn DownloadSection() -> impl IntoView {
    let download_cards = resolved_download_cards();

    view! {
        <section class="download-section" id="download">
            <div class="section-label">"// get started"</div>
            <h2>"Download "<span>"for free"</span></h2>
            <p style="color: var(--muted); margin-top: 1rem; max-width: 55ch; font-size: 0.95rem; line-height: 1.7;">"Windows binaries are ready to go. The release package includes the YuNet ONNX model — no manual setup needed."</p>

            <div class="download-grid">
                <For
                    each=move || download_cards.clone().into_iter()
                    key=|card| card.title
                    children=move |card| {
                        view! {
                            <a href=card.href target="_blank" rel="noopener noreferrer" class="download-card reveal" aria-label=card.aria_label>
                                <div class="dl-icon" aria-hidden="true">{card.icon}</div>
                                <h3>{card.title}</h3>
                                <p>{card.description}</p>
                                <span class="dl-tag">{card.tag}</span>
                            </a>
                        }
                    }
                />
            </div>
        </section>
    }
}
