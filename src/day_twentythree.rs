use actix_web::{get, web::Path, HttpResponse, Responder};
use indoc::formatdoc;
use serde::Deserialize;

#[get("/23/star")]
pub async fn star() -> impl Responder {
    HttpResponse::Ok().body("<div class=\"lit\" id=\"star\"></div>")
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Red,
    Blue,
    Purple,
}

impl Color {
    fn next(self) -> Self {
        match self {
            Color::Red => Color::Blue,
            Color::Blue => Color::Purple,
            Color::Purple => Color::Red,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Color::Red => "red",
            Color::Blue => "blue",
            Color::Purple => "purple",
        }
    }
}

#[get("23/present/{color}")]
pub async fn present(color: Path<Color>) -> impl Responder {
    let color = color.into_inner();
    let current_color = color.as_str();
    let next_color = color.next().as_str();
    HttpResponse::Ok().body(formatdoc!(
        r#"
      <div class="present {current_color}" hx-get="/23/present/{next_color}" hx-swap="outerHTML">
        <div class="ribbon"></div>
        <div class="ribbon"></div>
        <div class="ribbon"></div>
        <div class="ribbon"></div>
      </div>
      "#
    ))
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
enum State {
    On,
    Off,
}

impl State {
    fn next(self) -> Self {
        match self {
            State::On => State::Off,
            State::Off => State::On,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            State::On => "on",
            State::Off => "off",
        }
    }

    fn to_render(&self) -> &'static str {
        match self {
            State::On => " on",
            State::Off => "",
        }
    }
}

#[get("/23/ornament/{state}/{n}")]
pub async fn ornament(state: Path<(State, String)>) -> impl Responder {
    let (state, n) = state.into_inner();
    let n = html_escape::encode_safe(&n);
    let current_state = state.to_render();
    let state = state.next().as_str();
    HttpResponse::Ok().body(formatdoc! {r#"
      <div class="ornament{current_state}" id="ornament{n}" hx-trigger="load delay:2s once" hx-get="/23/ornament/{state}/{n}" hx-swap="outerHTML"></div>
      "#})
}
