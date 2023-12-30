use serde;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
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

#[function_component]
fn App() -> Html {

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

    let input_value = use_state(|| "".to_string());
    let input_value_to_query = input_value.clone();
    let input_value_to_string = input_value.clone();
    let input_value_string = (*input_value_to_string).clone();
    let on_cautious_change = Callback::from(move |e: Event| {
        let input_value = input_value.clone();
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input) = input {
            input_value.set(input.value());
        }
    });

    let is_submit = use_state(|| false);
    let weather = weather.clone();
    let weather_prop = (*weather).clone();

    {
        let is_submit = is_submit.clone();
        let is_submit_value = (*is_submit).clone();
        let input_value_string = input_value_to_query.clone();
        use_effect_with(is_submit_value, move |_| {
            let is_submit = is_submit.clone();
            let is_submit_value = (*is_submit).clone();
            if is_submit_value {
                wasm_bindgen_futures::spawn_local(async move {
                    let api_key = env!("WEATHER_API_KEY");
                    let query = (*input_value_string).clone();
                    let request_url = format!(
                        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=imperial",
                        query, api_key
                    );

                    let fetched_weather: Weather = gloo_net::http::Request::get(&request_url)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    weather.set(fetched_weather);
                });
                is_submit.set(false);
            }
            || ()
        });
    }

    let on_submit = Callback::from(move |_e: MouseEvent| {
        let is_submit = is_submit.clone();
        is_submit.set(true);
    });

    html! {
        <div class="p-2">
            <div class="mb-2">
                <h1 class="text-8xl text-slate-100"><span class="text-amber-600">{"Rust_"}</span>{"Weather"}</h1>
                <h1 class="text-4xl text-slate-100">{&weather_prop.location.name}</h1>
            </div>
            <SearchWeather {on_cautious_change} input_value={input_value_string} submit_weather_query={on_submit.clone()}/>
            <CurrentWeather weather={weather_prop}/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SearchWeatherProps {
    pub on_cautious_change: Callback<Event>,
    pub input_value: String,
    pub submit_weather_query: Callback<MouseEvent>,
}

#[function_component]
fn SearchWeather(props: &SearchWeatherProps) -> Html {
    html! {
        <div>
            <div class="flex flex-row gap-2 justify-start content-center items-center">
                <input required={true} class="text-slate-900 font-bold p-2 rounded" type="text" name="weather_query" value={props.input_value.clone()} onchange={props.on_cautious_change.clone()}/>
                <button class="bg-amber-600 hover:bg-amber-700 text-slate-100 font-bold py-2 px-4 rounded" onclick={props.submit_weather_query.clone()}>{"Submit"}</button>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Debug)]
pub struct CurrentWeatherProps {
    weather: Weather,
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
