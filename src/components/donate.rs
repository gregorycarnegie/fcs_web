use leptos::prelude::*;
use qrcodegen::{QrCode, QrCodeEcc};

#[derive(Clone, Copy)]
struct DonateTier {
    name: &'static str,
    amount: &'static str,
    description: &'static str,
    featured: bool,
}

#[derive(Clone, Copy)]
struct DonateLink {
    href: &'static str,
    aria_label: &'static str,
    class_name: &'static str,
    text: &'static str,
}

#[derive(Clone, Copy)]
struct CryptoWallet {
    symbol: &'static str,
    network: &'static str,
    address: &'static str,
    explorer_base_url: &'static str,
}

const FALLBACK_DONATE_TIERS: [DonateTier; 1] = [DonateTier {
    name: "💡 Support Access",
    amount: "Any",
    description: "Donation tiers are currently unavailable. Use any support link below.",
    featured: false,
}];

const FALLBACK_DONATE_LINKS: [DonateLink; 1] = [DonateLink {
    href: "https://github.com/sponsors/gregorycarnegie",
    aria_label: "Support via GitHub Sponsors",
    class_name: "btn-donate-outline",
    text: "♥ Support Project",
}];

const DONATE_TIERS: [DonateTier; 4] = [
    DonateTier {
        name: "☕ Buy Me a Coffee",
        amount: "£3",
        description: "Small but meaningful. Keeps the compiler warm.",
        featured: false,
    },
    DonateTier {
        name: "🚀 Boost a Feature",
        amount: "£10",
        description: "Funds a new enhancement shader or preset type. Most popular — thank you!",
        featured: true,
    },
    DonateTier {
        name: "🦀 Rust Patron",
        amount: "£25",
        description: "Serious support. Helps fund Linux/macOS build infrastructure and testing.",
        featured: false,
    },
    DonateTier {
        name: "⚡ GPU Sponsor",
        amount: "Custom",
        description: "Got a specific feature in mind? Let's talk. Your name in the README.",
        featured: false,
    },
];

const DONATE_LINKS: [DonateLink; 3] = [
    DonateLink {
        href: "https://ko-fi.com/gregory_carnegie",
        aria_label: "Donate via Ko-fi",
        class_name: "btn-donate",
        text: "☕ Ko-fi",
    },
    DonateLink {
        href: "https://github.com/sponsors/gregorycarnegie",
        aria_label: "Sponsor on GitHub Sponsors",
        class_name: "btn-donate-outline",
        text: "♥ GitHub Sponsors",
    },
    DonateLink {
        href: "https://buymeacoffee.com/gregory_carnegie",
        aria_label: "Donate via Buy Me a Coffee",
        class_name: "btn-donate-outline",
        text: "🍵 Buy Me a Coffee",
    },
];

const CRYPTO_WALLETS: [CryptoWallet; 3] = [
    CryptoWallet {
        symbol: "BTC",
        network: "Bitcoin",
        address: "bc1qkuj79evqn87fagxygpfmgt4qmnqg4mm6qdejmg",
        explorer_base_url: "https://www.blockchain.com/explorer/addresses/btc/",
    },
    CryptoWallet {
        symbol: "ETH",
        network: "Ethereum",
        address: "0x32Fa5C823D4fa3A4745AAc7C56325ED849340F28",
        explorer_base_url: "https://etherscan.io/address/",
    },
    CryptoWallet {
        symbol: "SOL",
        network: "Solana",
        address: "8z1BWsshYfkdgJm38RHRuhhVkYiPFSZLXbaSMTNch8e1",
        explorer_base_url: "https://solscan.io/account/",
    },
];

#[allow(clippy::const_is_empty)]
fn resolved_donate_tiers() -> Vec<DonateTier> {
    if DONATE_TIERS.is_empty() {
        FALLBACK_DONATE_TIERS.to_vec()
    } else {
        DONATE_TIERS.to_vec()
    }
}

#[allow(clippy::const_is_empty)]
fn resolved_donate_links() -> Vec<DonateLink> {
    if DONATE_LINKS.is_empty() {
        FALLBACK_DONATE_LINKS.to_vec()
    } else {
        DONATE_LINKS.to_vec()
    }
}

fn wallet_qr_data_uri(address: &str) -> String {
    use base64::Engine as _;

    let Some(qr) = QrCode::encode_text(address, QrCodeEcc::Medium).ok() else {
        return "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 180 180'%3E%3Crect width='100%25' height='100%25' fill='%23f5f5f5'/%3E%3C/svg%3E".to_string();
    };

    const BORDER: i32 = 2;
    const MODULE_SIZE: i32 = 6;

    let size = qr.size();
    let canvas = (size + (BORDER * 2)) * MODULE_SIZE;

    let mut svg = format!(
        "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 {0} {0}' shape-rendering='crispEdges'><rect width='100%' height='100%' fill='white'/>",
        canvas
    );

    for y in 0..size {
        for x in 0..size {
            if qr.get_module(x, y) {
                let x_pos = (x + BORDER) * MODULE_SIZE;
                let y_pos = (y + BORDER) * MODULE_SIZE;
                svg.push_str(&format!(
                    "<rect x='{x_pos}' y='{y_pos}' width='{MODULE_SIZE}' height='{MODULE_SIZE}' fill='black'/>"
                ));
            }
        }
    }

    svg.push_str("</svg>");

    format!(
        "data:image/svg+xml;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(svg)
    )
}

#[component]
pub fn DonateSection() -> impl IntoView {
    let donate_tiers = resolved_donate_tiers();
    let donate_links = resolved_donate_links();

    view! {
        <section class="donate-section" id="donate">
            <div class="donate-inner">
                <div class="donate-copy">
                    <div class="section-label">"// support development"</div>
                    <h2>"Help make it "<span>"better"</span></h2>
                    <p>"Face Crop Studio is free and open-source. Every donation goes directly into development time — better GPU pipelines, new preset types, Linux and macOS builds, and UI improvements."</p>
                    <p>"Even a small contribution keeps the project moving forward. No subscription, no paywalls, no strings attached."</p>
                    <p style="font-family: var(--font-mono); font-size: 0.8rem; color: var(--accent);">"\"Good tools should be accessible to everyone.\""</p>
                </div>

                <div>
                    <div class="donate-tiers">
                        <For
                            each=move || donate_tiers.clone().into_iter()
                            key=|tier| tier.name
                            children=move |tier| {
                                let class_name = if tier.featured { "tier featured reveal" } else { "tier reveal" };
                                view! {
                                    <div class=class_name>
                                        <div class="tier-top">
                                            <span class="tier-name">{tier.name}</span>
                                            <span class="tier-amount">{tier.amount}</span>
                                        </div>
                                        <p class="tier-desc">{tier.description}</p>
                                    </div>
                                }
                            }
                        />
                    </div>

                    <div class="donate-buttons">
                        <For
                            each=move || donate_links.clone().into_iter()
                            key=|link| link.text
                            children=move |link| {
                                view! {
                                    <a href=link.href target="_blank" rel="noopener noreferrer" class=link.class_name aria-label=link.aria_label>{link.text}</a>
                                }
                            }
                        />
                    </div>
                    <div class="crypto-wallets" aria-label="Crypto donation wallets">
                        <div class="crypto-title">"Crypto donations"</div>
                        <p class="crypto-sub">"Prefer crypto? Use one of the wallets below."</p>
                        <For
                            each=move || CRYPTO_WALLETS.into_iter()
                            key=|wallet| wallet.symbol
                            children=move |wallet| {
                                let (show_qr, set_show_qr) = signal(false);
                                let explorer_url = format!("{}{}", wallet.explorer_base_url, wallet.address);
                                let qr_src = wallet_qr_data_uri(wallet.address);
                                view! {
                                    <div class="crypto-wallet-item">
                                        <div class="crypto-wallet-row">
                                            <div class="crypto-wallet-meta">
                                                <span class="crypto-wallet-symbol">{wallet.symbol}</span>
                                                <span class="crypto-wallet-network">{wallet.network}</span>
                                            </div>
                                            <code class="crypto-wallet-address">{wallet.address}</code>
                                            <div class="crypto-wallet-actions">
                                                <button
                                                    type="button"
                                                    class="crypto-wallet-qr-btn"
                                                    aria-expanded=move || if show_qr.get() { "true" } else { "false" }
                                                    on:click=move |_| set_show_qr.update(|v| *v = !*v)
                                                >
                                                    {move || if show_qr.get() { "Hide QR" } else { "Show QR" }}
                                                </button>
                                                <a
                                                    href=explorer_url
                                                    target="_blank"
                                                    rel="noopener noreferrer"
                                                    class="crypto-wallet-link"
                                                    aria-label=format!("Open {} wallet address in explorer", wallet.symbol)
                                                >
                                                    "View"
                                                </a>
                                            </div>
                                        </div>
                                        <img
                                            class="crypto-wallet-qr"
                                            src=qr_src
                                            alt=format!("QR code for {} wallet address", wallet.symbol)
                                            loading="lazy"
                                            style:display=move || if show_qr.get() { "block" } else { "none" }
                                        />
                                    </div>
                                }
                            }
                        />
                    </div>
                    <p class="donate-note">"// All platforms accept one-time or recurring donations"</p>
                </div>
            </div>
        </section>
    }
}
