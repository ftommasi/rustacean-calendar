use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use chrono::{self, Datelike};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn CounterButton(cx : Scope,  get_count_fn :ReadSignal<i32> , set_count_fn : WriteSignal<i32>) -> impl IntoView{
    let mut cur_count = 0 ; // use value to capture updated count from signal
    
    //use WriteSignals to update count value and capture for current view
    let inc_button = move |_| set_count_fn.update(
        |count| {
            *count += 1; 
        });

    let dec_button = move |_| set_count_fn.update(
        |count| {
            *count -= 1; 
        });
        let temp = get_count_fn();
    //take currently captured value and display in view
    view!{ cx,
        <button on:click=inc_button>" +1 "</button>
        <h2>{temp}</h2>
        <button on:click=dec_button>"- 1 "</button>
    }
}

#[component]
fn EnterValueField(cx : Scope) -> impl IntoView{
    view!{ cx,
        <form>
            <label for="fname">"Enter Value"</label>
            <input type="number" id="fname" name="fname"/>
        </form>
    }
}

#[component]
fn CalendarDay(cx: Scope, day_num : u32) -> impl IntoView{
    let mut  display_day_num = day_num.to_string();
    if day_num > 31 {
        display_day_num = "".to_string();
    }
    view! { cx,
            <div class="Row">
                <div class="Column">"SUN"</div>
                <div class="Column">"MON"</div>
                <div class="Column">"TUE"</div>
                <div class="Column">"WED"</div>
                <div class="Column">"THU"</div>
                <div class="Column">"FRI"</div>
                <div class="Column">"SAT"</div>
            </div>
            <div class="Row">
                <div class="Column">"1"</div>
                <div class="Column">"2"</div>
                <div class="Column">"3"</div>
                <div class="Column">"4"</div>
                <div class="Column">"5"</div>
                <div class="Column">"6"</div>
                <div class="Column">"7"</div>
            </div>
            <div class="Row">
                <div class="Column">"8"</div>
                <div class="Column">"9"</div>
                <div class="Column">"10"</div>
                <div class="Column">"11"</div>
                <div class="Column">"12"</div>
                <div class="Column">"13"</div>
                <div class="Column">"14"</div>
            </div>
            <div class="Row">
                <div class="Column">"15"</div>
                <div class="Column">"16"</div>
                <div class="Column">"17"</div>
                <div class="Column">"18"</div>
                <div class="Column">"19"</div>
                <div class="Column">"20"</div>
                <div class="Column">"21"</div>
            </div>
            <div class="Row">
                <div class="Column">"22"</div>
                <div class="Column">"23"</div>
                <div class="Column">"24"</div>
                <div class="Column">"25"</div>
                <div class="Column">"26"</div>
                <div class="Column">"27"</div>
                <div class="Column">"28"</div>
            </div>
            <div class="Row">
                <div class="Column">"29"</div>
                <div class="Column">"30"</div>
                <div class="Column">"31"</div>
                <div class="Column">"32"</div>
                <div class="Column">"33"</div>
                <div class="Column">"34"</div>
                <div class="Column">"35"</div>
            </div>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (today, set_today) = create_signal(cx,chrono::Utc::now());
    let (year, set_year)          = create_signal(cx,today.get().year());
    let (month, set_month) = create_signal(cx,today.get().month());
    let (day, set_day)   = create_signal(cx,today.get().day());
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
       
        <h1>"Today: " {year} " - " {month} " - "  {day}</h1>
        <CounterButton  get_count_fn=count set_count_fn= set_count/> //Add 1
        <EnterValueField /> //set arbitrary
        <CalendarDay day_num = day.get()/>
    }
}
