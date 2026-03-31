use leptos::prelude::*;

#[derive(Clone, serde::Deserialize, serde::Serialize, Debug)]
pub struct StopResult {
    pub stop_id: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

#[component]
pub fn StopSearch() -> impl IntoView {
    let (query, set_query) = signal(String::new());
    let (results, set_results) = signal(Vec::<StopResult>::new());
    let (loading, set_loading) = signal(false);

    // Search on input change — debounce handled server-side (min 2 chars)
    let on_input = move |ev: leptos::ev::Event| {
        let val = event_target_value(&ev);
        set_query.set(val.clone());

        if val.len() < 2 {
            set_results.set(vec![]);
            return;
        }

        set_loading.set(true);
        let val = val.clone();
        leptos::task::spawn_local(async move {
            // TODO: call server action / API endpoint
            // For now just clear
            let _ = val;
            set_results.set(vec![]);
            set_loading.set(false);
        });
    };

    view! {
        <div class="stop-search">
            <input
                type="text"
                placeholder="Megálló neve (pl. Győr, autóbusz-állomás)"
                prop:value=query
                on:input=on_input
                class="search-input"
            />
            {move || loading.get().then(|| view! { <p class="loading">"Keresés..."</p> })}
            <ul class="search-results">
                {move || results.get().into_iter().map(|stop| {
                    let stop_id = stop.stop_id.clone();
                    view! {
                        <li>
                            <a href=format!("/stop/{}", stop_id)>
                                {stop.name.clone()}
                            </a>
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
