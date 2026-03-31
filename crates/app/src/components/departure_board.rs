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
        <div class="departure-board">
            <h3>"Következő járatok"</h3>

            {move || error.get().map(|e| view! {
                <p class="error">"Hiba: " {e}</p>
            })}

            {move || loading.get().then(|| view! {
                <p class="loading">"Betöltés..."</p>
            })}

            {move || (!loading.get()).then(|| {
                let deps = departures.get();
                if deps.is_empty() {
                    view! {
                        <p class="empty">"Nincs közelgő járat."</p>
                    }.into_any()
                } else {
                    view! {
                        <table class="departures">
                            <thead>
                                <tr>
                                    <th>"Járat"</th>
                                    <th>"Irány"</th>
                                    <th>"Indulás"</th>
                                    <th>"Késés"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {deps.into_iter().map(|d| {
                                    let delay_str = match d.delay_seconds {
                                        Some(s) if s > 0 => format!("+{}p", s / 60),
                                        Some(_) | None => "—".to_string(),
                                    };
                                    view! {
                                        <tr>
                                            <td class="route-badge">{d.route_short_name}</td>
                                            <td>{d.headsign}</td>
                                            <td>{d.departure_time}</td>
                                            <td class="delay">{delay_str}</td>
                                        </tr>
                                    }
                                }).collect::<Vec<_>>()}
                            </tbody>
                        </table>
                    }.into_any()
                }
            })}
        </div>
    }
}
