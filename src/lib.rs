pub mod components;
pub mod graphql;
pub mod pages;
pub mod services;

pub use components::Navigation;
pub use components::ProgressBar;
pub use pages::home::Home;
use services::graphql_service::GraphQLService;
use services::service_context::ServiceContext;

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
use std::sync::Arc;

#[component]
pub fn App() -> impl IntoView {
    // let location_state = use_location();
    let service = GraphQLService::new();

    provide_context(ServiceContext(Arc::new(service)));

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
