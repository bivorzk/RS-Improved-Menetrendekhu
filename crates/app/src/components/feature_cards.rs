use leptos::prelude::*;

#[component]
pub fn FeatureCards() -> impl IntoView {
    view! {
        <div class="grid gap-4 sm:grid-cols-3">
            <div class="glass-card p-6">
                <h2 class="text-lg font-semibold text-white">"Indulás"</h2>
                <p class="mt-3 text-sm leading-6 text-slate-300">"Gyorsan és egyszerűen indíthatod a keresést Budapesten vagy a környéken."</p>
            </div>
            <div class="glass-card p-6">
                <h2 class="text-lg font-semibold text-white">"Mobilra optimalizált"</h2>
                <p class="mt-3 text-sm leading-6 text-slate-300">"A felület minden eszközön letisztult és könnyen használható marad."</p>
            </div>
            <div class="glass-card p-6">
                <h2 class="text-lg font-semibold text-white">"Gyors áttekintés"</h2>
                <p class="mt-3 text-sm leading-6 text-slate-300">"A keresési eredmények kártyákon jelennek meg, így azonnal megtalálod a legjobb utat."</p>
            </div>
        </div>
    }
}
