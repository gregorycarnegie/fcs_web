use leptos::prelude::*;

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
    explorer_url: &'static str,
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
        href: "https://ko-fi.com",
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
        href: "https://buymeacoffee.com",
        aria_label: "Donate via Buy Me a Coffee",
        class_name: "btn-donate-outline",
        text: "🍵 Buy Me a Coffee",
    },
];

const CRYPTO_WALLETS: [CryptoWallet; 3] = [
    CryptoWallet {
        symbol: "BTC",
        network: "Bitcoin",
        address: "bc1qreplacewithyourbtcaddresshere00000000000000",
        explorer_url: "https://www.blockchain.com/explorer/addresses/btc/bc1qreplacewithyourbtcaddresshere00000000000000",
    },
    CryptoWallet {
        symbol: "ETH",
        network: "Ethereum",
        address: "0xReplaceWithYourEthAddress000000000000000000",
        explorer_url: "https://etherscan.io/address/0xReplaceWithYourEthAddress000000000000000000",
    },
    CryptoWallet {
        symbol: "SOL",
        network: "Solana",
        address: "ReplaceWithYourSolAddress11111111111111111111111",
        explorer_url: "https://solscan.io/account/ReplaceWithYourSolAddress11111111111111111111111",
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
                    <p style="font-family: 'DM Mono', monospace; font-size: 0.8rem; color: var(--accent);">"\"Good tools should be accessible to everyone.\""</p>
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
                                view! {
                                    <div class="crypto-wallet-row">
                                        <div class="crypto-wallet-meta">
                                            <span class="crypto-wallet-symbol">{wallet.symbol}</span>
                                            <span class="crypto-wallet-network">{wallet.network}</span>
                                        </div>
                                        <code class="crypto-wallet-address">{wallet.address}</code>
                                        <a
                                            href=wallet.explorer_url
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            class="crypto-wallet-link"
                                            aria-label=format!("Open {} wallet address in explorer", wallet.symbol)
                                        >
                                            "View"
                                        </a>
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
