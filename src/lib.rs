pub mod components;
pub mod pages;
pub use components::Navigation;
pub use components::ProgressBar;
pub use pages::home::Home;

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Navigation/>
                <Routes fallback=move || view! { <p>"Not found."</p> }>
                    <ParentRoute
                        path=path!("")
                        view=|| view! { <Outlet/> }
                    >
                        <Route
                            path=path!("")
                            view=Home
                        />
                        <Route
                            path=path!("progress")
                            view=ProgressBar
                        />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}
