use leptos::prelude::*;

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
                        <div class="tier reveal"><div class="tier-top"><span class="tier-name">"☕ Buy Me a Coffee"</span><span class="tier-amount">"£3"</span></div><p class="tier-desc">"Small but meaningful. Keeps the compiler warm."</p></div>
                        <div class="tier featured reveal"><div class="tier-top"><span class="tier-name">"🚀 Boost a Feature"</span><span class="tier-amount">"£10"</span></div><p class="tier-desc">"Funds a new enhancement shader or preset type. Most popular — thank you!"</p></div>
                        <div class="tier reveal"><div class="tier-top"><span class="tier-name">"🦀 Rust Patron"</span><span class="tier-amount">"£25"</span></div><p class="tier-desc">"Serious support. Helps fund Linux/macOS build infrastructure and testing."</p></div>
                        <div class="tier reveal"><div class="tier-top"><span class="tier-name">"⚡ GPU Sponsor"</span><span class="tier-amount">"Custom"</span></div><p class="tier-desc">"Got a specific feature in mind? Let's talk. Your name in the README."</p></div>
                    </div>

                    <div class="donate-buttons">
                        <a href="https://ko-fi.com" target="_blank" class="btn-donate">"☕ Ko-fi"</a>
                        <a href="https://github.com/sponsors/gregorycarnegie" target="_blank" class="btn-donate-outline">"♥ GitHub Sponsors"</a>
                        <a href="https://buymeacoffee.com" target="_blank" class="btn-donate-outline">"🍵 Buy Me a Coffee"</a>
                    </div>
                    <p class="donate-note">"// All platforms accept one-time or recurring donations"</p>
                </div>
            </div>
        </section>
    }
}
