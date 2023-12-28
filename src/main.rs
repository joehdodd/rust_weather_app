use serde;
use std::env;
use yew::prelude::*;

#[derive(Clone, PartialEq, serde::Deserialize)]
struct WeatherLocation {
    name: String,
    lat: f64,
    lon: f64,
    #[serde(rename = "type")]
    location_type: String,
}

#[derive(Clone, PartialEq, serde::Deserialize)]
struct Weather {
    location: WeatherLocation,
}
#[derive(Properties, PartialEq)]
pub struct CounterComponentProps {
    weather: Weather,
}

#[function_component]
fn App() -> Html {
    let api_key = env!("WEATHER_API_KEY");
    let weather = use_state(|| Weather {
        location: WeatherLocation {
            name: "".to_string(),
            lat: 0.0,
            lon: 0.0,
            location_type: "".to_string(),
        },
    });
    {
        let weather = weather.clone();
        use_effect_with((), move |_| {
            let weather = weather.clone();
            let request_url = format!(
                "https://api.tomorrow.io/v4/weather/forecast?location={}&apikey={}",
                "Portland", api_key
            );
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_weather: Weather = gloo_net::http::Request::get(&request_url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                weather.set(fetched_weather);
            });
            || ()
        });
    }
    html! {
        <div class="p-2">
            <h1 class="text-4xl text-slate-500">{weather.location.name.clone()}</h1>
            <Comp weather={(*weather).clone()}/>
        </div>
    }
}

#[function_component]
fn Comp(props: &CounterComponentProps) -> Html {
    html! {
        <div class="p-2 my-2 rounded-lg shadow-md bg-slate-300">
            <h2 class="text-slate-500">{props.weather.location.name.clone()}</h2>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
