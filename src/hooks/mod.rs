use leptos::prelude::*;

pub fn use_reveal_observer() {
    Effect::new(move |_| init_reveal_observer());
}

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

    if window
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .is_some_and(|media| media.matches())
    {
        if let Ok(elements) = document
            .query_selector_all(".feature-card, .download-card, .tier, .stat, .tech-inner > div")
        {
            for i in 0..elements.length() {
                if let Some(el) = elements.get(i) {
                    let el: Element = el.unchecked_into();
                    let _ = el.class_list().add_1("visible");
                }
            }
        }
        return;
    }

    let callback = Closure::wrap(Box::new(
        move |entries: js_sys::Array, _observer: IntersectionObserver| {
            for entry in entries.iter() {
                let intersection_entry: web_sys::IntersectionObserverEntry = entry.unchecked_into();
                if intersection_entry.is_intersecting() {
                    let target: Element = intersection_entry.target();
                    let _ = target.class_list().add_1("visible");
                }
            }
        },
    ) as Box<dyn FnMut(js_sys::Array, IntersectionObserver)>);

    let options = IntersectionObserverInit::new();
    options.set_threshold(&0.1f64.into());

    let observer =
        IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options);
    let Ok(observer) = observer else {
        return;
    };

    if let Ok(elements) = document
        .query_selector_all(".feature-card, .download-card, .tier, .stat, .tech-inner > div")
    {
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
