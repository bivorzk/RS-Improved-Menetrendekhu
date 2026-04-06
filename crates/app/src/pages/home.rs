use leptos::prelude::*;

use crate::components::feature_cards::FeatureCards;
use crate::components::page_header::SectionHeader;
use crate::components::search_panel::SearchPanel;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <section class="rounded-[36px] border border-slate-800/90 bg-slate-900/90 p-8 shadow-soft sm:p-10">
                <div class="space-y-8">
                    <SectionHeader
                        eyebrow="Modern menetrendek"
                        title="Gyors és elegáns utazástervezés Budapesten"
                        description="Tervezd meg az útvonalat valós időben, keresd a megállókat és nézd meg a térképet egy modern, mobilbarát felületen."
                    />
                    <SearchPanel/>
                </div>
            </section>

            <FeatureCards/>
        </div>
    }
}
