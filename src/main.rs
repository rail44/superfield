#![feature(proc_macro_non_items)]

extern crate squark;
extern crate squark_macros;
extern crate actix_web;

use actix_web::{http, server, App, Path, Responder, HttpResponse};
use actix_web::server::{HttpServer, HttpHandler};
use squark::App as SquarkApp;
use squark_macros::view;

#[derive(Clone, Debug, PartialEq)]
struct State {
    count: isize,
}

impl State {
    pub fn new() -> State {
        State { count: 0 }
    }
}

#[derive(Clone, Debug)]
enum Action {
    ChangeCount(isize),
}

#[derive(Clone, Debug)]
struct CounterApp;
impl squark::App for CounterApp {
    type State = State;
    type Action = Action;

    fn reducer(&self, mut state: State, action: Action) -> State {
        match action {
            Action::ChangeCount(c) => {
                state.count = c;
            }
        };
        state
    }

    fn view(&self, state: State) -> squark::View<Action> {
        let count = state.count;
        view! {
            <div>
                { count.to_string() }
                <button onclick={ move |_| Some(Action::ChangeCount(count.clone() + 1)) }>
                    increment
                </button>
                <button onclick={ move |_| Some(Action::ChangeCount(count - 1)) }>
                    decrement
                </button>
            </div>
        }
    }
}

impl Default for CounterApp {
    fn default() -> CounterApp {
        CounterApp
    }
}

fn index(info: Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

fn hoge() -> HttpServer<impl HttpHandler> {
    server::new(|| {
        App::new().resource("/", |r| {
            let app = CounterApp::default();
            let node = app.view(State::new()).node;
            r.f(move |_| {
                let mut res = HttpResponse::Ok();
                res.body(format!("<html><body>{}</body></html>", view_to_string(node.clone())))
            })
        })
    })
}

fn main() {
    hoge().bind("127.0.0.1:8080").unwrap()
        .run();
}

fn view_to_string(n: squark::Node) -> String {
    match n {
        squark::Node::Text(s) => s,
        squark::Node::Element(el) => {
            let children: Vec<String> = el.children.iter().map(|child| view_to_string(child.clone())).collect();
            format!("<{}>{}</{}>", el.name, children.join(""), el.name)
        },
        squark::Node::Null => "".to_string(),
    }
}
