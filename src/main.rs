use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CounterComponentProps {
    pub count: i32,
}

#[function_component]
fn App() -> Html {
    let counter: UseStateHandle<i32> = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div class="p-2">
            <button class="bg-slate-500 text-white rounded-xl shadow-md p-2" {onclick}>{ "Increment the Counter" }</button>
            <Comp count={*counter}/>
        </div>
    }
}

#[function_component]
fn Comp(props: &CounterComponentProps) -> Html {
    html! {
        <div class="p-2 my-2 rounded-lg shadow-md bg-slate-300">
            <h2 class="text-slate-500">{ "I'm a component. And here is the count:" }{" "}{props.count}</h2>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
