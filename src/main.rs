use leptos::prelude::*;

const PAGE_CSS: &str = r#"
:root {
  --bg: #0a0a0f;
  --surface: #111118;
  --card: #16161f;
  --border: #2a2a3a;
  --accent: #e85d26;
  --accent2: #f5a623;
  --rust: #b7410e;
  --text: #e8e8f0;
  --muted: #7a7a9a;
  --code-bg: #0d0d15;
}

*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

html { scroll-behavior: smooth; }

body {
  background: var(--bg);
  color: var(--text);
  font-family: 'DM Sans', sans-serif;
  font-weight: 300;
  line-height: 1.6;
  overflow-x: hidden;
}

body::before {
  content: '';
  position: fixed;
  inset: 0;
  background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 256 256' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noise'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noise)' opacity='0.04'/%3E%3C/svg%3E");
  pointer-events: none;
  z-index: 9999;
  opacity: 0.35;
}

nav {
  position: fixed;
  top: 0; left: 0; right: 0;
  z-index: 100;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1.2rem 3rem;
  background: rgba(10,10,15,0.85);
  backdrop-filter: blur(12px);
  border-bottom: 1px solid var(--border);
}

.nav-logo {
  font-family: 'Syne', sans-serif;
  font-weight: 800;
  font-size: 1.1rem;
  letter-spacing: -0.02em;
}

.nav-logo span { color: var(--accent); }

nav ul {
  list-style: none;
  display: flex;
  gap: 2rem;
}

nav a {
  color: var(--muted);
  text-decoration: none;
  font-size: 0.85rem;
  letter-spacing: 0.05em;
  text-transform: uppercase;
  transition: color 0.2s;
}

nav a:hover { color: var(--text); }

.nav-cta {
  background: var(--accent) !important;
  color: white !important;
  padding: 0.45rem 1.2rem !important;
  border-radius: 4px !important;
  font-weight: 500 !important;
  transition: background 0.2s !important;
  text-transform: none !important;
  letter-spacing: 0 !important;
}

.nav-cta:hover { background: #d14d1a !important; }

.hero {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  padding: 8rem 3rem 5rem;
  position: relative;
  overflow: hidden;
}

.hero-grid {
  position: absolute;
  inset: 0;
  background-image:
    linear-gradient(rgba(232,93,38,0.05) 1px, transparent 1px),
    linear-gradient(90deg, rgba(232,93,38,0.05) 1px, transparent 1px);
  background-size: 60px 60px;
  mask-image: radial-gradient(ellipse 70% 60% at 50% 50%, black, transparent);
}

.hero-glow {
  position: absolute;
  width: 700px; height: 700px;
  border-radius: 50%;
  background: radial-gradient(circle, rgba(232,93,38,0.12) 0%, transparent 70%);
  top: -100px; left: -200px;
  pointer-events: none;
}

.hero-glow-2 {
  position: absolute;
  width: 500px; height: 500px;
  border-radius: 50%;
  background: radial-gradient(circle, rgba(183,65,14,0.08) 0%, transparent 70%);
  bottom: 0; right: -100px;
}

.hero-eyebrow {
  font-family: 'DM Mono', monospace;
  font-size: 0.75rem;
  color: var(--accent);
  letter-spacing: 0.15em;
  text-transform: uppercase;
  margin-bottom: 1.5rem;
  opacity: 0;
  animation: fadeUp 0.6s 0.1s forwards;
}

h1 {
  font-family: 'Syne', sans-serif;
  font-weight: 800;
  font-size: clamp(3rem, 7vw, 6rem);
  line-height: 0.95;
  letter-spacing: -0.03em;
  max-width: 14ch;
  opacity: 0;
  animation: fadeUp 0.7s 0.2s forwards;
}

h1 em {
  font-style: normal;
  color: var(--accent);
}

.hero-sub {
  margin-top: 2rem;
  max-width: 52ch;
  color: var(--muted);
  font-size: 1.05rem;
  line-height: 1.7;
  opacity: 0;
  animation: fadeUp 0.7s 0.35s forwards;
}

.hero-badges {
  display: flex;
  flex-wrap: wrap;
  gap: 0.6rem;
  margin-top: 2rem;
  opacity: 0;
  animation: fadeUp 0.7s 0.45s forwards;
}

.badge {
  font-family: 'DM Mono', monospace;
  font-size: 0.7rem;
  padding: 0.3rem 0.75rem;
  border: 1px solid var(--border);
  border-radius: 100px;
  color: var(--muted);
  background: var(--surface);
  letter-spacing: 0.05em;
}

.badge.rust { border-color: var(--accent); color: var(--accent); }

.hero-actions {
  display: flex;
  gap: 1rem;
  margin-top: 2.5rem;
  flex-wrap: wrap;
  opacity: 0;
  animation: fadeUp 0.7s 0.55s forwards;
}

.btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.85rem 2rem;
  border-radius: 6px;
  font-size: 0.9rem;
  font-weight: 500;
  text-decoration: none;
  cursor: pointer;
  border: none;
  transition: all 0.2s;
  font-family: 'DM Sans', sans-serif;
}

.btn-primary {
  background: var(--accent);
  color: white;
}

.btn-primary:hover { background: #d14d1a; transform: translateY(-1px); }

.btn-outline {
  background: transparent;
  color: var(--text);
  border: 1px solid var(--border);
}

.btn-outline:hover { border-color: var(--muted); background: var(--surface); }

.stats-bar {
  background: var(--surface);
  border-top: 1px solid var(--border);
  border-bottom: 1px solid var(--border);
  padding: 1.5rem 3rem;
  display: flex;
  gap: 4rem;
  flex-wrap: wrap;
}

.stat {
  display: flex;
  flex-direction: column;
  gap: 0.2rem;
}

.stat-val {
  font-family: 'Syne', sans-serif;
  font-size: 1.8rem;
  font-weight: 800;
  color: var(--accent);
}

.stat-label {
  font-size: 0.75rem;
  color: var(--muted);
  text-transform: uppercase;
  letter-spacing: 0.08em;
}

section { padding: 6rem 3rem; }

.section-label {
  font-family: 'DM Mono', monospace;
  font-size: 0.7rem;
  color: var(--accent);
  letter-spacing: 0.15em;
  text-transform: uppercase;
  margin-bottom: 1rem;
}

h2 {
  font-family: 'Syne', sans-serif;
  font-size: clamp(2rem, 4vw, 3rem);
  font-weight: 800;
  letter-spacing: -0.025em;
  line-height: 1.1;
  max-width: 20ch;
}

h2 span { color: var(--accent); }

.features-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 1.5px;
  margin-top: 4rem;
  background: var(--border);
  border: 1px solid var(--border);
}

.feature-card {
  background: var(--card);
  padding: 2rem;
  transition: background 0.2s;
}

.feature-card:hover { background: #1c1c28; }

.feature-icon {
  width: 44px; height: 44px;
  background: rgba(232,93,38,0.1);
  border: 1px solid rgba(232,93,38,0.25);
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.3rem;
  margin-bottom: 1.2rem;
}

.feature-card h3 {
  font-family: 'Syne', sans-serif;
  font-weight: 700;
  font-size: 1rem;
  margin-bottom: 0.5rem;
  letter-spacing: -0.01em;
}

.feature-card p {
  font-size: 0.85rem;
  color: var(--muted);
  line-height: 1.65;
}

.tech-section {
  background: var(--surface);
  border-top: 1px solid var(--border);
  border-bottom: 1px solid var(--border);
}

.tech-inner {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 5rem;
  align-items: center;
}

.code-block {
  background: var(--code-bg);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 1.5rem;
  font-family: 'DM Mono', monospace;
  font-size: 0.78rem;
  line-height: 1.8;
  overflow-x: auto;
}

.code-block .comment { color: #4a4a6a; }
.code-block .keyword { color: #e85d26; }
.code-block .func { color: #f5a623; }
.code-block .string { color: #7ec4a0; }
.code-block .type { color: #8ab4f8; }

.code-header {
  font-family: 'DM Mono', monospace;
  font-size: 0.65rem;
  color: var(--muted);
  letter-spacing: 0.1em;
  margin-bottom: 1rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--border);
  text-transform: uppercase;
}

.presets-scroll {
  display: flex;
  gap: 1rem;
  margin-top: 3rem;
  flex-wrap: wrap;
}

.preset-pill {
  padding: 0.6rem 1.4rem;
  border: 1px solid var(--border);
  border-radius: 100px;
  font-family: 'DM Mono', monospace;
  font-size: 0.75rem;
  color: var(--muted);
  background: var(--card);
  cursor: pointer;
  transition: all 0.2s;
}

.preset-pill:hover, .preset-pill.active {
  border-color: var(--accent);
  color: var(--accent);
  background: rgba(232,93,38,0.05);
}

.download-section {
  background: linear-gradient(135deg, rgba(232,93,38,0.08) 0%, rgba(10,10,15,0) 60%);
  border-top: 1px solid var(--border);
}

.download-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 1rem;
  margin-top: 3rem;
}

.download-card {
  background: var(--card);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 1.8rem;
  text-decoration: none;
  color: inherit;
  transition: all 0.2s;
  display: flex;
  flex-direction: column;
  gap: 0.8rem;
}

.download-card:hover {
  border-color: var(--accent);
  background: rgba(232,93,38,0.05);
  transform: translateY(-2px);
}

.download-card .dl-icon { font-size: 1.8rem; }

.download-card h3 {
  font-family: 'Syne', sans-serif;
  font-weight: 700;
  font-size: 0.95rem;
}

.download-card p { font-size: 0.8rem; color: var(--muted); }

.dl-tag {
  display: inline-block;
  font-family: 'DM Mono', monospace;
  font-size: 0.65rem;
  padding: 0.2rem 0.6rem;
  border-radius: 4px;
  background: rgba(232,93,38,0.15);
  color: var(--accent);
  margin-top: auto;
}

.donate-section { background: var(--surface); border-top: 1px solid var(--border); }

.donate-inner {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 5rem;
  align-items: start;
}

.donate-copy h2 { margin-bottom: 1.5rem; }
.donate-copy p { color: var(--muted); margin-bottom: 1rem; line-height: 1.75; font-size: 0.95rem; }

.donate-tiers {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.tier {
  background: var(--card);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 1.5rem;
  transition: border-color 0.2s;
  position: relative;
  overflow: hidden;
}

.tier::before {
  content: '';
  position: absolute;
  left: 0; top: 0; bottom: 0;
  width: 3px;
  background: var(--border);
  transition: background 0.2s;
}

.tier:hover { border-color: var(--accent); }
.tier:hover::before { background: var(--accent); }
.tier.featured { border-color: rgba(232,93,38,0.4); }
.tier.featured::before { background: var(--accent); }

.tier-top {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  margin-bottom: 0.5rem;
}

.tier-name {
  font-family: 'Syne', sans-serif;
  font-weight: 700;
  font-size: 0.9rem;
}

.tier-amount {
  font-family: 'DM Mono', monospace;
  font-size: 1.2rem;
  color: var(--accent);
  font-weight: 500;
}

.tier-desc { font-size: 0.8rem; color: var(--muted); }

.donate-buttons {
  display: flex;
  gap: 0.8rem;
  margin-top: 1.5rem;
  flex-wrap: wrap;
}

.btn-donate {
  background: var(--accent);
  color: white;
  padding: 0.75rem 1.5rem;
  border-radius: 6px;
  font-size: 0.85rem;
  font-weight: 500;
  text-decoration: none;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.btn-donate:hover { background: #d14d1a; transform: translateY(-1px); }

.btn-donate-outline {
  background: transparent;
  color: var(--text);
  padding: 0.75rem 1.5rem;
  border-radius: 6px;
  font-size: 0.85rem;
  font-weight: 500;
  text-decoration: none;
  border: 1px solid var(--border);
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.btn-donate-outline:hover { border-color: var(--muted); }

.donate-note {
  font-size: 0.75rem;
  color: var(--muted);
  margin-top: 1rem;
  font-family: 'DM Mono', monospace;
}

footer {
  border-top: 1px solid var(--border);
  padding: 2rem 3rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 1rem;
}

.footer-logo {
  font-family: 'Syne', sans-serif;
  font-weight: 800;
  font-size: 0.9rem;
}

.footer-logo span { color: var(--accent); }

.footer-links {
  display: flex;
  gap: 1.5rem;
  list-style: none;
}

.footer-links a {
  color: var(--muted);
  text-decoration: none;
  font-size: 0.8rem;
  transition: color 0.2s;
}

.footer-links a:hover { color: var(--text); }

.footer-copy {
  font-size: 0.75rem;
  color: var(--muted);
  font-family: 'DM Mono', monospace;
}

@keyframes fadeUp {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}

.reveal {
  opacity: 0;
  transform: translateY(24px);
  transition: opacity 0.6s ease, transform 0.6s ease;
}

.reveal.visible {
  opacity: 1;
  transform: none;
}

@media (max-width: 768px) {
  nav { padding: 1rem 1.5rem; }
  nav ul { display: none; }
  .hero { padding: 7rem 1.5rem 4rem; }
  section { padding: 4rem 1.5rem; }
  .tech-inner, .donate-inner { grid-template-columns: 1fr; gap: 2.5rem; }
  .stats-bar { padding: 1.5rem; gap: 2rem; }
  footer { flex-direction: column; text-align: center; }
}
"#;

#[cfg(target_arch = "wasm32")]
fn init_reveal_observer() {
    use wasm_bindgen::{JsCast, closure::Closure};
    use web_sys::{Element, IntersectionObserver, IntersectionObserverInit};

    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };

    let callback = Closure::wrap(Box::new(move |entries: js_sys::Array, _observer: IntersectionObserver| {
        for entry in entries.iter() {
            let intersection_entry: web_sys::IntersectionObserverEntry = entry.unchecked_into();
            if intersection_entry.is_intersecting() {
                let target: Element = intersection_entry.target();
                let _ = target.class_list().add_1("visible");
            }
        }
    }) as Box<dyn FnMut(js_sys::Array, IntersectionObserver)>);

    let options = IntersectionObserverInit::new();
    options.set_threshold(&0.1f64.into());

    let observer = IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options);
    let Ok(observer) = observer else {
        return;
    };

    if let Ok(elements) = document.query_selector_all(".feature-card, .download-card, .tier, .stat, .tech-inner > div") {
        for i in 0..elements.length() {
            if let Some(el) = elements.get(i) {
                let el: Element = el.unchecked_into();
                let _ = el.class_list().add_1("reveal");
                observer.observe(&el);
            }
        }
    }

    callback.forget();
    std::mem::forget(observer);
}

#[cfg(not(target_arch = "wasm32"))]
fn init_reveal_observer() {}

#[component]
fn App() -> impl IntoView {
    let presets = vec![
        "yunet-core",
        "yunet-utils",
        "yunet-gui",
        "yunet-cli",
        "tract-onnx",
        "wgpu/WGSL",
        "eframe/egui",
    ];
    let (active_preset, set_active_preset) = signal(presets[0].to_string());

    Effect::new(move |_| {
        init_reveal_observer();
    });

    view! {
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
        <link href="https://fonts.googleapis.com/css2?family=Syne:wght@400;600;700;800&family=DM+Mono:ital,wght@0,400;0,500;1,400&family=DM+Sans:wght@300;400;500&display=swap" rel="stylesheet"/>
        <style>{PAGE_CSS}</style>

        <nav>
            <div class="nav-logo">"Face"<span>"Crop"</span>" Studio"</div>
            <ul>
                <li><a href="#features">"Features"</a></li>
                <li><a href="#download">"Download"</a></li>
                <li><a href="#donate">"Donate"</a></li>
                <li><a href="https://github.com/gregorycarnegie/iron_cropper" target="_blank">"GitHub"</a></li>
                <li><a href="#download" class="nav-cta">"Get it Free"</a></li>
            </ul>
        </nav>

        <section class="hero">
            <div class="hero-grid"></div>
            <div class="hero-glow"></div>
            <div class="hero-glow-2"></div>
            <div class="hero-eyebrow">"// yunet-powered · rust-built · gpu-accelerated"</div>
            <h1>"Face "<em>"Crop"</em>" Studio"</h1>
            <p class="hero-sub">
                "Precision face detection and cropping with GPU-accelerated batch processing. LinkedIn headshots, passports, IDs — deterministic, high-quality, blazing fast."
            </p>
            <div class="hero-badges">
                <span class="badge rust">"🦀 Rust"</span>
                <span class="badge">"WGSL Compute Shaders"</span>
                <span class="badge">"YuNet Face Detection"</span>
                <span class="badge">"GPU + CPU"</span>
                <span class="badge">"MIT License"</span>
            </div>
            <div class="hero-actions">
                <a href="#download" class="btn btn-primary">"⬇ Download for Windows"</a>
                <a href="https://github.com/gregorycarnegie/iron_cropper" target="_blank" class="btn btn-outline">"View on GitHub →"</a>
            </div>
        </section>

        <div class="stats-bar">
            <div class="stat"><span class="stat-val">"7"</span><span class="stat-label">"GPU Compute Shaders"</span></div>
            <div class="stat"><span class="stat-val">"6+"</span><span class="stat-label">"Crop Presets"</span></div>
            <div class="stat"><span class="stat-val">"4"</span><span class="stat-label">"Export Formats"</span></div>
            <div class="stat"><span class="stat-val">"97%"</span><span class="stat-label">"Rust Codebase"</span></div>
        </div>

        <section id="features">
            <div class="section-label">"// capabilities"</div>
            <h2>"Everything you need for "<span>"perfect crops"</span></h2>
            <div class="features-grid">
                <div class="feature-card"><div class="feature-icon">"🎯"</div><h3>"Crop Presets"</h3><p>"LinkedIn, Passport, Instagram, ID Card, Avatar, Headshot — and fully custom dimensions. Each preset nails the correct aspect ratio automatically."</p></div>
                <div class="feature-card"><div class="feature-icon">"⚡"</div><h3>"GPU Acceleration"</h3><p>"Full wgpu/WGSL pipeline with custom compute shaders for preprocessing, enhancement, and even YuNet inference. Automatic CPU fallback when needed."</p></div>
                <div class="feature-card"><div class="feature-icon">"📊"</div><h3>"Quality Scoring"</h3><p>"Laplacian-variance sharpness scoring categorises crops as Low, Medium, or High quality. Auto-select the sharpest face in batch jobs."</p></div>
                <div class="feature-card"><div class="feature-icon">"🎨"</div><h3>"Enhancement Pipeline"</h3><p>"Auto colour, exposure, brightness, contrast, saturation, sharpening, skin smoothing, red-eye removal, and portrait background blur — CPU and GPU paths."</p></div>
                <div class="feature-card"><div class="feature-icon">"📁"</div><h3>"Mapping Workflows"</h3><p>"Import CSV, Excel, Parquet, or SQLite datasets to drive batch naming. Feed in thousands of source→output mappings in one go."</p></div>
                <div class="feature-card"><div class="feature-icon">"🖥️"</div><h3>"GUI + CLI"</h3><p>"Native egui desktop app with live preview, undo/redo, and history. Plus a full-featured CLI for scripting and automation workflows."</p></div>
                <div class="feature-card"><div class="feature-icon">"📐"</div><h3>"Positioning Modes"</h3><p>"Centre, Rule of Thirds, or fully custom offsets with pixel-precise keyboard nudges and comprehensive undo/redo support."</p></div>
                <div class="feature-card"><div class="feature-icon">"🏷️"</div><h3>"Metadata Control"</h3><p>"Preserve, strip, or customise image metadata. Export to PNG, JPEG with quality controls, or WebP — all with accurate metadata handling."</p></div>
                <div class="feature-card"><div class="feature-icon">"🔋"</div><h3>"Batch Processing"</h3><p>"Queue hundreds of images, track status per-file, derive filenames from templates, and get structured JSON/CSV failure reports for every job."</p></div>
            </div>
        </section>

        <section class="tech-section">
            <div class="tech-inner">
                <div>
                    <div class="section-label">"// architecture"</div>
                    <h2>"Built for "<span>"performance"</span></h2>
                    <p style="color: var(--muted); margin-top: 1.5rem; line-height: 1.75; font-size: 0.95rem;">"Face Crop Studio is a Rust workspace with a clean crate separation: "<strong style="color: var(--text);">"yunet-core"</strong>" handles detection and crop maths, "<strong style="color: var(--text);">"yunet-utils"</strong>" provides shared helpers, and "<strong style="color: var(--text);">"yunet-gui"</strong>" / "<strong style="color: var(--text);">"yunet-cli"</strong>" are the surfaces."</p>
                    <p style="color: var(--muted); margin-top: 1rem; line-height: 1.75; font-size: 0.95rem;">"GPU context pooling means the CLI handles batch jobs efficiently, and the GUI shares the wgpu context with eframe's renderer for zero-overhead compute."</p>
                    <div class="presets-scroll" style="margin-top: 2rem;">
                        <For
                            each=move || presets.clone().into_iter()
                            key=|preset| *preset
                            children=move |preset| {
                                let selected = move || active_preset.get() == preset;
                                view! {
                                    <span
                                        class=move || if selected() { "preset-pill active" } else { "preset-pill" }
                                        on:click=move |_| set_active_preset.set(preset.to_string())
                                    >
                                        {preset}
                                    </span>
                                }
                            }
                        />
                    </div>
                </div>
                <div>
                    <div class="code-block">
                        <div class="code-header">"yunet-cli — batch example"</div>
                        <pre style="white-space: pre-wrap; margin: 0;">
"# Batch crop to LinkedIn preset with GPU
yunet-cli \
  --input ./photos/ \
  --preset LinkedIn \
  --face-height 60 \
  --gpu \
  --enhance \
  --quality-threshold Medium \
  --output ./crops/

# Mapping-driven batch from Excel
yunet-cli \
  --mapping-file roster.xlsx \
  --mapping-source photo_path \
  --mapping-output employee_id \
  --preset IDCard \
  --output ./id_crops/

# Benchmark GPU vs CPU preprocessing
yunet-cli --benchmark-preprocess"
                        </pre>
                    </div>
                </div>
            </div>
        </section>

        <section class="download-section" id="download">
            <div class="section-label">"// get started"</div>
            <h2>"Download "<span>"for free"</span></h2>
            <p style="color: var(--muted); margin-top: 1rem; max-width: 55ch; font-size: 0.95rem; line-height: 1.7;">"Windows binaries are ready to go. The release package includes the YuNet ONNX model — no manual setup needed."</p>

            <div class="download-grid">
                <a href="https://github.com/gregorycarnegie/iron_cropper/releases/latest" target="_blank" class="download-card reveal">
                    <div class="dl-icon">"📦"</div>
                    <h3>"MSI Installer"</h3>
                    <p>"Enterprise-style installer for Windows x86-64. Clean install and uninstall support."</p>
                    <span class="dl-tag">"face-crop-studio-windows-x86_64.msi"</span>
                </a>
                <a href="https://github.com/gregorycarnegie/iron_cropper/releases/latest" target="_blank" class="download-card reveal">
                    <div class="dl-icon">"🔧"</div>
                    <h3>"Setup Executable"</h3>
                    <p>"Traditional Windows installer with guided setup wizard."</p>
                    <span class="dl-tag">"face-crop-studio-windows-x86_64-setup.exe"</span>
                </a>
                <a href="https://github.com/gregorycarnegie/iron_cropper/releases/latest" target="_blank" class="download-card reveal">
                    <div class="dl-icon">"🗜️"</div>
                    <h3>"Portable ZIP"</h3>
                    <p>"No install needed. Extract anywhere and run. Ideal for servers and restricted environments."</p>
                    <span class="dl-tag">"face-crop-studio-windows-x86_64.zip"</span>
                </a>
                <a href="https://github.com/gregorycarnegie/iron_cropper" target="_blank" class="download-card reveal">
                    <div class="dl-icon">"🦀"</div>
                    <h3>"Build from Source"</h3>
                    <p>"Full Rust workspace. Requires NASM on PATH. Linux and macOS supported via source build."</p>
                    <span class="dl-tag">"cargo build --release --workspace"</span>
                </a>
            </div>
        </section>

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

        <footer>
            <div class="footer-logo">"Face"<span>"Crop"</span>" Studio"</div>
            <ul class="footer-links">
                <li><a href="https://github.com/gregorycarnegie/iron_cropper" target="_blank">"GitHub"</a></li>
                <li><a href="https://github.com/gregorycarnegie/iron_cropper/releases" target="_blank">"Releases"</a></li>
                <li><a href="https://github.com/gregorycarnegie/iron_cropper/issues" target="_blank">"Issues"</a></li>
                <li><a href="https://github.com/gregorycarnegie/iron_cropper/blob/master/README.md" target="_blank">"Docs"</a></li>
            </ul>
            <span class="footer-copy">"MIT License · Built with 🦀 Rust"</span>
        </footer>
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    mount_to_body(App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("This Leptos app targets wasm32. Build with trunk or leptos tooling.");
}
