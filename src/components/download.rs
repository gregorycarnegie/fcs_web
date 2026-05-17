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
    href: "https://github.com/gregorycarnegie/face-crop-studio/releases/latest",
    aria_label: "Open latest Face Crop Studio releases",
    icon: "ℹ️",
    title: "Releases Unavailable",
    description: "Download metadata is currently unavailable. Open releases for the latest binaries.",
    tag: "Check latest release artifacts",
}];

const DOWNLOAD_CARDS: [DownloadCard; 7] = [
    DownloadCard {
        href: "https://github.com/gregorycarnegie/face-crop-studio/releases/latest",
        aria_label: "Open latest GitHub release for Windows MSI installer",
        icon: "📦",
        title: "Windows MSI",
        description: "Enterprise-style installer for Windows x86-64. Clean install and uninstall support.",
        tag: "face-crop-studio-windows-x86_64.msi",
    },
    DownloadCard {
        href: "https://github.com/gregorycarnegie/face-crop-studio/releases/latest",
        aria_label: "Open latest GitHub release for Windows setup executable",
        icon: "🔧",
        title: "Windows Setup",
        description: "Traditional Windows installer with guided setup wizard.",
        tag: "face-crop-studio-windows-x86_64-setup.exe",
    },
    DownloadCard {
        href: "https://github.com/gregorycarnegie/face-crop-studio/releases/latest",
        aria_label: "Open latest GitHub release for Windows portable zip",
        icon: "🗜️",
        title: "Windows Portable",
        description: "No install needed. Extract anywhere and run. Ideal for servers and restricted environments.",
        tag: "face-crop-studio-windows-x86_64.zip",
    },
    DownloadCard {
        href: "https://github.com/gregorycarnegie/face-crop-studio/releases/latest",
        aria_label: "Open latest GitHub release for macOS Apple Silicon disk image",
        icon: "🍎",
        title: "macOS (Apple Silicon)",
        description: "Signed disk image for Apple Silicon Macs. Drag to Applications and launch.",
        tag: "face-crop-studio-aarch64-apple-darwin.dmg",
    },
    DownloadCard {
        href: "https://github.com/gregorycarnegie/face-crop-studio/releases/latest",
        aria_label: "Open latest GitHub release for Linux AppImage",
        icon: "🐧",
        title: "Linux AppImage",
        description: "Portable Linux binary for x86-64. Make it executable and run — no install required.",
        tag: "face-crop-studio-x86_64.AppImage",
    },
    DownloadCard {
        href: "https://github.com/gregorycarnegie/face-crop-studio/releases/latest",
        aria_label: "Open latest GitHub release for Debian package",
        icon: "📥",
        title: "Linux .deb",
        description: "Debian/Ubuntu package for x86-64. Install with apt or dpkg for system integration.",
        tag: "face-crop-studio-x86_64.deb",
    },
    DownloadCard {
        href: "https://github.com/gregorycarnegie/face-crop-studio",
        aria_label: "Open Face Crop Studio source repository on GitHub",
        icon: "🦀",
        title: "Build from Source",
        description: "Full Rust workspace. Requires NASM on PATH. Build for any platform Rust supports.",
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
            <p style="color: var(--muted); margin-top: 1rem; max-width: 55ch; font-size: 0.95rem; line-height: 1.7;">"Native binaries for Windows, macOS, and Linux. The release package includes the YuNet ONNX model — no manual setup needed."</p>

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
