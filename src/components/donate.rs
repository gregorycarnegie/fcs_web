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

#[component]
pub fn DonateSection() -> impl IntoView {
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
                            each=move || DONATE_TIERS.into_iter()
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
                            each=move || DONATE_LINKS.into_iter()
                            key=|link| link.text
                            children=move |link| {
                                view! {
                                    <a href=link.href target="_blank" rel="noopener noreferrer" class=link.class_name aria-label=link.aria_label>{link.text}</a>
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
