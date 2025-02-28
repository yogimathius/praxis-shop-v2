use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen(module = "/src/pages/home.module.css")]
extern "C" {}

#[component]
pub fn Home() -> impl IntoView {
    let (hover_index, set_hover_index) = signal(-1);

    let principles = vec![
        ("Praxis", "Theory into Action"),
        ("Forge", "Shape Your Future"),
        ("Intention", "Purposeful Progress"),
        ("Mastery", "Continuous Growth"),
    ];

    view! {
        <div class="home-container">
            <h2 class="home-title fade-in">"Welcome to Praxis Forge"</h2>

            <p class="subtitle slide-in">
                "Transform intentions into actions, actions into habits, habits into mastery."
            </p>

            <div class="principles-grid">
                {principles.into_iter().enumerate().map(|(i, (title, desc))| {
                    let i = i;
                    view! {
                        <div
                            class="principle-card"
                            class:active=move || hover_index.get() == i as i32
                            on:mouseenter=move |_| set_hover_index.set(i as i32)
                            on:mouseleave=move |_| set_hover_index.set(-1)
                        >
                            <h3 class="principle-title">{title}</h3>
                            <p class="principle-desc">{desc}</p>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <div class="cta-section">
                <a href="/tasks" class="cta-button">
                    "Start Your Journey"
                </a>
            </div>
        </div>
    }
}
