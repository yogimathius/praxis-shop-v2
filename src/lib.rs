pub mod pages;
pub use pages::home::Home;

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes fallback=move || view! { <p>"Not found."</p> }>
                    <ParentRoute
                        path=path!("")
                        view=|| view! { <Outlet/> }
                    >
                        <Route
                            path=path!("")
                            view=Home
                        />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}
