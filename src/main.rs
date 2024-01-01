use serde;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Clone, PartialEq, serde::Deserialize, Debug)]
struct Weather {
    coord: Coord,
    weather: Vec<WeatherInfo>,
    base: String,
    main: Main,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
}

#[derive(Clone, PartialEq, serde::Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Clone, PartialEq, serde::Deserialize, Debug)]
struct WeatherInfo {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Clone, PartialEq, serde::Deserialize, Debug)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: u32,
    humidity: u32,
}

#[derive(Clone, PartialEq, serde::Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: u32,
}

#[derive(Clone, PartialEq, serde::Deserialize, Debug)]
struct Clouds {
    all: u32,
}

#[derive(Clone, PartialEq, serde::Deserialize, Debug)]
struct Sys {
    r#type: u32,
    id: u32,
    country: String,
    sunrise: u32,
    sunset: u32,
}

#[function_component]
fn App() -> Html {
    let weather = use_state(|| Weather {
        coord: Coord { lon: 0.0, lat: 0.0 },
        weather: vec![WeatherInfo {
            id: 0,
            main: "".to_string(),
            description: "".to_string(),
            icon: "".to_string(),
        }],
        base: "".to_string(),
        main: Main {
            temp: 0.0,
            feels_like: 0.0,
            temp_min: 0.0,
            temp_max: 0.0,
            pressure: 0,
            humidity: 0,
        },
        visibility: 0,
        wind: Wind { speed: 0.0, deg: 0 },
        clouds: Clouds { all: 0 },
        dt: 0,
        sys: Sys {
            r#type: 0,
            id: 0,
            country: "".to_string(),
            sunrise: 0,
            sunset: 0,
        },
        timezone: 0,
        id: 0,
        name: "".to_string(),
        cod: 0,
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
    let has_data = use_state(|| false);
    {
        let is_submit = is_submit.clone();
        let has_data = has_data.clone();
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
                    has_data.set(true);
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

    let has_data_value = (*has_data).clone();
    html! {
        <div class="flex flex-col gap-12 p-2">
            <div class="mb-2">
                <h1 class="text-8xl text-slate-300"><span class="text-amber-600">{"Rust"}</span>{"_Weather"}</h1>
            </div>
            <SearchWeather {on_cautious_change} input_value={input_value_string} submit_weather_query={on_submit.clone()}/>
            if has_data_value {<CurrentWeather weather={weather_prop.clone()}/>}
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
                <input required={true} class="text-slate-900 bg-slate-300 font-bold p-2 rounded" type="text" name="weather_query" value={props.input_value.clone()} onchange={props.on_cautious_change.clone()}/>
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
        <div class="flex flex-row gap-12 justify-between">
            <div class="w-full p-2 rounded-lg shadow-2xl bg-slate-300">
                <h1 class="text-slate-600 text-3xl">{props.weather.name.clone()}{","}{props.weather.sys.country.clone()}</h1>
            </div>
            <div class="w-full p-2 rounded-lg shadow-2xl bg-slate-300">
                <h1 class="text-slate-600 text-3xl">{props.weather.main.temp}{"\u{00b0}"}</h1>
                <h4 class="text-slate-600 text-3xl">{"H:"}{props.weather.main.temp_max}{"\u{00b0}"}</h4>
                <h4 class="text-slate-600 text-3xl">{"L:"}{props.weather.main.temp_min}{"\u{00b0}"}</h4>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
