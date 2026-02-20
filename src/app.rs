use leptos::prelude::*;

use crate::components::{
    DonateSection, DownloadSection, FeaturesSection, FooterSection, HeroSection, NavBar, StatsBar,
    TechSection,
};
use crate::hooks::use_reveal_observer;

#[component]
pub fn App() -> impl IntoView {
    use_reveal_observer();

    view! {
        <NavBar/>
        <main id="main-content">
            <HeroSection/>
            <StatsBar/>
            <FeaturesSection/>
            <TechSection/>
            <DownloadSection/>
            <DonateSection/>
        </main>
        <FooterSection/>
    }
}
