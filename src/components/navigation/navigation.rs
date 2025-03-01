use leptos::prelude::*;
use leptos_router::components::*;
use wasm_bindgen::prelude::*;
// Let's define our own Location enum since we need it
#[derive(Clone, Debug, PartialEq)]
pub enum Location {
    Home,
    Tasks,
    Goals,
    Progress,
}

#[wasm_bindgen(module = "/src/components/navigation/navigation.module.css")]
extern "C" {
    #[wasm_bindgen(js_name = "default")]
    static STYLES: JsValue;
}

#[component]
pub fn Navigation() -> impl IntoView {
    // Create signals for location state
    let (location, set_location) = signal(Location::Home);

    view! {
        <nav class="nav">
            <div class="nav-item" class:active=move || location.get() == Location::Home>
                <span class="link">
                    <A
                        href="/"
                        on:click=move |_| set_location.set(Location::Home)
                    >
                        "Home"
                    </A>
                </span>
            </div>
            <div class="nav-item" class:active=move || location.get() == Location::Goals>
                <span class="link">
                    <A
                        href="/progress"
                        on:click=move |_| set_location.set(Location::Progress)
                    >
                        "Progress"
                    </A>
                </span>
            </div>
            // <div class="nav-item" class:active=move || location.get() == Location::Tasks>
            //     <span class="link">
            //         <A
            //             href="/tasks"
            //             on:click=move |_| set_location.set(Location::Tasks)
            //         >
            //             "Tasks"
            //         </A>
            //     </span>
            // </div>
            <div class="nav-item" class:active=move || location.get() == Location::Goals>
                <span class="link">
                    <A
                        href="/goals"
                        on:click=move |_| set_location.set(Location::Goals)
                    >
                        "Goals"
                    </A>
                </span>
            </div>
        </nav>
    }
}
