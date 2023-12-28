use serde;
use std::env;
use yew::prelude::*;

#[derive(Clone, PartialEq, serde::Deserialize, Debug)]
#[allow(non_snake_case)]
struct WeatherValues {
    cloudBase: f32,
    cloudCeiling: f32,
    dewPoint: f32,
    freezingRainIntensity: i32,
    humidity: i32,
    precipitationProbability: i32,
    pressureSurfaceLevel: f32,
    rainIntensity: i32,
    sleetIntensity: i32,
    snowIntensity: i32,
    temperature: f32,
    temperatureApparent: f32,
    uvHealthConcern: i32,
    uvIndex: i32,
    visibility: i32,
    weatherCode: i32,
    windDirection: f32,
    windGust: f32,
    windSpeed: f32,
}
#[derive(Clone, PartialEq, serde::Deserialize, Debug)]
struct WeatherData {
    time: String,
    values: WeatherValues,
}

#[derive(Clone, PartialEq, serde::Deserialize, Debug)]
struct WeatherLocation {
    name: String,
    lat: f64,
    lon: f64,
    #[serde(rename = "type")]
    location_type: String,
}

#[derive(Clone, PartialEq, serde::Deserialize, Debug)]
struct Weather {
    location: WeatherLocation,
    data: WeatherData,
}
#[derive(Properties, PartialEq, Debug)]
pub struct CurrentWeatherProps {
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
        data: WeatherData {
            time: "".to_string(),
            values: WeatherValues {
                cloudBase: 0.0,
                cloudCeiling: 0.0,
                dewPoint: 0.0,
                freezingRainIntensity: 0,
                humidity: 0,
                precipitationProbability: 0,
                pressureSurfaceLevel: 0.0,
                rainIntensity: 0,
                sleetIntensity: 0,
                snowIntensity: 0,
                temperature: 0.0,
                temperatureApparent: 0.0,
                uvHealthConcern: 0,
                uvIndex: 0,
                visibility: 0,
                weatherCode: 0,
                windDirection: 0.0,
                windGust: 0.0,
                windSpeed: 0.0,
            },
        },
    });
    {
        let weather = weather.clone();
        use_effect_with((), move |_| {
            // let weather = weather.clone();
            // let request_url = format!(
            //     "https://api.tomorrow.io/v4/weather/realtime?location={}&apikey={}",
            //     "Portland", api_key
            // );
            // wasm_bindgen_futures::spawn_local(async move {
            //     let fetched_weather: Weather = gloo_net::http::Request::get(&request_url)
            //         .send()
            //         .await
            //         .unwrap()
            //         .json()
            //         .await
            //         .unwrap();
            //     weather.set(fetched_weather);
            // });
            // || ()
        });
    }
    html! {
        <div class="p-2">
            <h1 class="text-8xl text-slate-100"><span class="text-amber-600">{"Rust_"}</span>{"Weather"}</h1>
            <h1 class="text-4xl text-slate-100">{weather.location.name.clone()}</h1>
            <CurrentWeather weather={(*weather).clone()}/>
        </div>
    }
}

#[function_component]
fn CurrentWeather(props: &CurrentWeatherProps) -> Html {
    html! {
        <div class="p-2 my-2 rounded-lg shadow-lg bg-slate-100">
            <h1 class="text-slate-600 text-3xl">{props.weather.data.values.temperature * 9.0 / 5.0 + 32.0}{"\u{00b0}"}</h1>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
