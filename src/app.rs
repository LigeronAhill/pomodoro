use gloo_timers::callback::Timeout;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::{timer_controls::TimerControls, timer_display::TimerDisplay};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}
#[derive(PartialEq, Clone, Debug, Copy)]
pub enum TimerState {
    Paused,
    Running,
    Break,
}
#[derive(Serialize, Deserialize)]
struct SetTitleArgs<'a> {
    title: &'a str,
}

fn get_tray_title(timer_state: TimerState, timer_duration: u32, session_length: u32) -> String {
    match timer_state {
        TimerState::Paused => "Paused".to_string(),
        TimerState::Running => {
            if timer_duration > session_length {
                format!("Finished session: {}", format_time(timer_duration))
            } else {
                format!(
                    "In session: {}",
                    format_time(session_length - timer_duration)
                )
            }
        }
        TimerState::Break => {
            if timer_duration > session_length {
                format!("Finished break: {}", format_time(timer_duration))
            } else {
                format!("In break: {}", format_time(session_length - timer_duration))
            }
        }
    }
}
pub fn format_time(seconds: u32) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;

    format!("{:02}:{:02}", minutes, seconds)
}

#[function_component(App)]
pub fn app() -> Html {
    let session_length = use_state(|| 45 * 60);
    let timer_duration = use_state(|| 0);
    let timer_state = use_state(|| TimerState::Paused);
    use_effect_with(
        (
            timer_duration.clone(),
            timer_state.clone(),
            session_length.clone(),
        ),
        move |props| {
            let (timer_duration, timer_state, _) = props.to_owned();
            let timeout = Timeout::new(1_000, move || {
                if *timer_state != TimerState::Paused {
                    timer_duration.set(*timer_duration + 1);
                }
            });
            let (timer_duration, timer_state, session_length) = props.to_owned();
            spawn_local(async move {
                let (timer_duration, timer_state, session_length) =
                    (*timer_duration, *timer_state, *session_length);
                let args = to_value(&SetTitleArgs {
                    title: &get_tray_title(
                        timer_state.clone(),
                        timer_duration.clone(),
                        session_length.clone(),
                    ),
                })
                .unwrap();
                invoke("set_title", args).await;
            });
            move || {
                timeout.cancel();
            }
        },
    );
    html! {
        <div class={classes!("flex", "flex-col", "items-center", "justify-center", "h-screen", "bg-gray-900", "text-slate-100")}>
            <TimerDisplay session_length={session_length.clone()} timer_state={timer_state.clone()} timer_duration={timer_duration.clone()} />
            <TimerControls session_length={session_length.clone()} timer_state={timer_state.clone()} timer_duration={timer_duration.clone()} />
        </div>
    }
}
