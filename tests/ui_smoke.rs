const FEATURES_SRC: &str = include_str!("../src/components/features.rs");
const DOWNLOAD_SRC: &str = include_str!("../src/components/download.rs");
const DONATE_SRC: &str = include_str!("../src/components/donate.rs");
const TECH_SRC: &str = include_str!("../src/components/tech.rs");

#[test]
fn sections_have_stable_ids() {
    assert!(FEATURES_SRC.contains("id=\"features\""));
    assert!(DOWNLOAD_SRC.contains("id=\"download\""));
    assert!(DONATE_SRC.contains("id=\"donate\""));
}

#[test]
fn tech_section_keeps_all_preset_pills() {
    let expected_presets = [
        "yunet-core",
        "yunet-utils",
        "yunet-gui",
        "yunet-cli",
        "tract-onnx",
        "wgpu/WGSL",
        "eframe/egui",
    ];

    for preset in expected_presets {
        assert!(TECH_SRC.contains(preset), "missing preset: {preset}");
    }
}
