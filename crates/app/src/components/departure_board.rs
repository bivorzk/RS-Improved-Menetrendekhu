use leptos::prelude::*;

#[derive(Clone, serde::Deserialize, serde::Serialize, Debug)]
pub struct Departure {
    pub trip_id: String,
    pub route_short_name: String,
    pub headsign: String,
    pub departure_time: String,
    pub delay_seconds: Option<i32>,
}

#[component]
pub fn DepartureBoard(stop_id: Signal<String>) -> impl IntoView {
    let (departures, set_departures) = signal(Vec::<Departure>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(Option::<String>::None);

    // Fetch departures whenever stop_id changes
    Effect::new(move |_| {
        let id = stop_id.get();
        if id.is_empty() {
            return;
        }

        set_loading.set(true);
        set_error.set(None);

        leptos::task::spawn_local(async move {
            // TODO: call /api/stops/{id}/departures
            let _ = id;
            set_departures.set(vec![]);
            set_loading.set(false);
        });
    });

    view! {
        <div class="departure-board rounded-[28px] border border-slate-800/90 bg-slate-900/80 p-6">
            <h3 class="text-xl font-semibold text-white mb-4">"Következő járatok"</h3>

            {move || error.get().map(|e| view! {
                <p class="text-sm text-rose-300">"Hiba: " {e}</p>
            })}

            {move || loading.get().then(|| view! {
                <p class="text-sm text-slate-400">"Betöltés..."</p>
            })}

            {move || (!loading.get()).then(|| {
                let deps = departures.get();
                if deps.is_empty() {
                    view! {
                        <p class="text-sm text-slate-400">"Nincs közelgő járat."</p>
                    }.into_any()
                } else {
                    view! {
                        <table class="w-full border-collapse text-left text-slate-200">
                            <thead>
                                <tr class="border-b border-slate-800/80 text-slate-400">
                                    <th class="py-3">"Járat"</th>
                                    <th class="py-3">"Irány"</th>
                                    <th class="py-3">"Indulás"</th>
                                    <th class="py-3">"Késés"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {deps.into_iter().map(|d| {
                                    let delay_str = match d.delay_seconds {
                                        Some(s) if s > 0 => format!("+{}p", s / 60),
                                        Some(_) | None => "—".to_string(),
                                    };
                                    view! {
                                        <tr class="border-b border-slate-800/80 last:border-none">
                                            <td class="py-4 font-semibold text-white">{d.route_short_name}</td>
                                            <td class="py-4">{d.headsign}</td>
                                            <td class="py-4">{d.departure_time}</td>
                                            <td class="py-4 text-slate-300">{delay_str}</td>
                                        </tr>
                                    }
                                }).collect::<Vec<_>>() }
                            </tbody>
                        </table>
                    }.into_any()
                }
            })}
        </div>
    }
}
