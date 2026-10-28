//! Rune GPU UI for srcthread.
//!
//! Entered from the `srcthread` CLI via `srcthread desktop` — same product
//! binary, no separate desktop executable. See
//! `research/08-herdr-single-binary.md`.
//!
//! Hello-world: shows engine greeting + thread list (query) and a button that
//! creates a thread (mutation) via the in-process [`srcthread`] lib.

use std::process::ExitCode;

use rune::tok::{self, motion, type_};
use rune::{AppSpec, Cx, El, col, el, text};
use srcthread::Srcthread;

struct State {
    engine: Srcthread,
    greeting: String,
    threads: Vec<String>,
    status: String,
}

impl Default for State {
    fn default() -> Self {
        let engine = Srcthread::open_memory();
        let greeting = engine.hello().to_string();
        let threads = engine.list_threads();
        Self {
            engine,
            greeting,
            threads,
            status: "query: list_threads".into(),
        }
    }
}

impl State {
    fn refresh(&mut self) {
        self.threads = self.engine.list_threads();
    }

    fn add_thread(&mut self) {
        let id = format!("desktop-{}", self.threads.len() + 1);
        match self.engine.create_thread(id.clone()) {
            Ok(()) => {
                self.refresh();
                self.status = format!("mutation: create_thread {id}");
            }
            Err(err) => self.status = format!("error: {err}"),
        }
    }
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = tok::theme(true);
    let list = s.threads.join(", ");
    let add = el()
        .key("add")
        .role("button")
        .px(10.0)
        .py(8.0)
        .radius(8.0)
        .fill(tok::accent(0, true))
        .motion(motion::SNAPPY)
        .on_press(|s: &mut State| s.add_thread())
        .child(text("create thread").font(type_::BODY).color(t.text));

    col()
        .w_full()
        .h_full()
        .center()
        .gap(12.0)
        .fill(t.bg)
        .child(text("srcthread").font(type_::TITLE).color(t.text))
        .child(
            text(format!("{} — {}", s.greeting, list))
                .font(type_::BODY)
                .color(t.text_secondary),
        )
        .child(add)
        .child(
            text(s.status.as_str())
                .font(type_::BODY)
                .color(t.text_secondary),
        )
}

/// Open the hello-world desktop window (blocks until quit).
pub fn run() -> ExitCode {
    rune::run(AppSpec::<State> {
        ..AppSpec::new("srcthread", (480.0, 280.0), State::default, view)
    });
    ExitCode::SUCCESS
}
