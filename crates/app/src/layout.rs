use leptos::prelude::*;
use crate::components::navigation::TopNav;

#[component]
pub fn AppLayout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-slate-950 text-slate-100">
            <TopNav/>
            <main class="mx-auto w-full max-w-7xl px-4 py-8 sm:px-6 lg:px-10">
                {children()}
            </main>
        </div>
    }
}
