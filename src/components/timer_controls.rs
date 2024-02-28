use yew::prelude::*;

use crate::app::TimerState;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub timer_state: UseStateHandle<TimerState>,
    pub timer_duration: UseStateHandle<u32>,
    pub session_length: UseStateHandle<u32>,
}

#[function_component]
pub fn TimerControls(props: &Props) -> Html {
    let Props {
        timer_state,
        timer_duration,
        session_length,
    } = props;

    let start_timer: Callback<()> = {
        let timer_state = timer_state.clone();

        Callback::from(move |_| {
            timer_state.set(TimerState::Running);
        })
    };

    let pause_timer: Callback<()> = {
        let timer_state = timer_state.clone();

        Callback::from(move |_| {
            timer_state.set(TimerState::Paused);
        })
    };

    let reset_timer: Callback<()> = {
        let timer_state = timer_state.clone();
        let timer_duration = timer_duration.clone();
        let session_length = session_length.clone();

        Callback::from(move |_| {
            timer_state.set(TimerState::Paused);
            timer_duration.set(0);
            session_length.set(45 * 60); // Reset to 25 minute session time
        })
    };

    let take_break: Callback<()> = {
        let timer_state = timer_state.clone();
        let timer_duration = timer_duration.clone();
        let session_length = session_length.clone();

        Callback::from(move |_| {
            timer_state.set(TimerState::Break);
            timer_duration.set(0);
            session_length.set(5 * 60); // 5 minute break time
        })
    };

    let finish_break: Callback<()> = {
        let timer_state = timer_state.clone();
        let timer_duration = timer_duration.clone();
        let session_length = session_length.clone();

        Callback::from(move |_| {
            timer_state.set(TimerState::Running);
            timer_duration.set(0);
            session_length.set(45 * 60); // Reset state to 25 minutes
        })
    };

    match **timer_state {
        TimerState::Running => {
            html!(
                h1>a>
          
              <div class={classes!("flex", "flex-row", "space-x-2")}>
                <button class={classes!("p-3")} onclick={move |_| {
                  take_break.emit(());
                }}>
                <svg width="46" height="46" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                  <path d="M18 8h1a4 4 0 0 1 0 8h-1"></path>
                  <path d="M2 8h16v9a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V8z"></path>
                  <path d="M6 1v3"></path>
                  <path d="M10 1v3"></path>
                  <path d="M14 1v3"></path>
                </svg>
                </button>
                <button class={classes!("p-3")} onclick={move |_| {
                  pause_timer.emit(());
                }}>
                <svg width="46" height="46" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                  <path d="M6 4h4v16H6z"></path>
                  <path d="M14 4h4v16h-4z"></path>
                </svg>
                </button>
                <button class={classes!("p-3")} onclick={move |_| {
                  reset_timer.emit(());
                }}>
                <svg width="46" height="46" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                  <path d="M1 4v6h6"></path>
                  <path d="M23 20v-6h-6"></path>
                  <path d="M20.49 9A9 9 0 0 0 5.64 5.64L1 10"></path>
                  <path d="m23 14-4.64 4.36A9 9 0 0 1 3.51 15"></path>
                </svg>
                </button>
              </div>
            )
        }
        TimerState::Paused => {
            html!(
              <div class={classes!("flex", "flex-row", "space-x-2")}>
                <button class={classes!("p-3")} onclick={move |_| {
                  take_break.emit(());
                }}>
                <svg width="46" height="46" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                  <path d="M18 8h1a4 4 0 0 1 0 8h-1"></path>
                  <path d="M2 8h16v9a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V8z"></path>
                  <path d="M6 1v3"></path>
                  <path d="M10 1v3"></path>
                  <path d="M14 1v3"></path>
                </svg>
                </button>
                <button class={classes!("p-3")} onclick={move |_| {
                  start_timer.emit(());
                }}>
                <svg width="46" height="46" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                  <path d="m5 3 14 9-14 9V3z"></path>
                </svg>
                </button>
                <button class={classes!("p-3")} onclick={move |_| {
                  reset_timer.emit(());
                }}>
                <svg width="46" height="46" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                  <path d="M1 4v6h6"></path>
                  <path d="M23 20v-6h-6"></path>
                  <path d="M20.49 9A9 9 0 0 0 5.64 5.64L1 10"></path>
                  <path d="m23 14-4.64 4.36A9 9 0 0 1 3.51 15"></path>
                </svg>
                </button>
              </div>
            )
        }
        TimerState::Break => {
            html!(
              <div class={classes!("flex", "flex-row", "space-x-2")}>
                <button class={classes!("p-3")} onclick={move |_| {
                  finish_break.emit(());
                }}>
                <svg width="46" height="46" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                  <path d="m5 3 14 9-14 9V3z"></path>
                </svg>
                </button>
              </div>
            )
        }
    }
}
