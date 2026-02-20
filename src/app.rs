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
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
        <link rel="preload" r#as="style" href="https://fonts.googleapis.com/css2?family=Syne:wght@400;600;700;800&family=DM+Mono:ital,wght@0,400;0,500;1,400&family=DM+Sans:wght@300;400;500&display=swap"/>
        <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Syne:wght@400;600;700;800&family=DM+Mono:ital,wght@0,400;0,500;1,400&family=DM+Sans:wght@300;400;500&display=swap"/>

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
