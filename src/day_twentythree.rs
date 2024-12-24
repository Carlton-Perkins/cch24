use std::io::Read;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, post, web::Path, HttpResponse, Responder};
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

#[derive(MultipartForm)]
struct Form {
    #[multipart(rename = "lockfile")]
    lockfile: TempFile,
}

#[derive(Debug, Deserialize)]
struct Lockfile {
    package: Vec<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    checksum: Option<String>,
}

#[post("/23/lockfile")]
pub async fn lockfile(MultipartForm(form): MultipartForm<Form>) -> impl Responder {
    let body = form
        .lockfile
        .file
        .bytes()
        .filter(|b| b.is_ok())
        .map(|b| b.unwrap())
        .collect::<Vec<u8>>();
    let file: Lockfile = match toml::from_slice(&body) {
        Ok(file) => file,
        Err(err) => return HttpResponse::BadRequest().body(format!("invalid toml: {err:?}")),
    };

    let mut output = vec![];
    for package in file.package {
        let Some(sum) = package.checksum else {
            continue;
        };

        let color = sum.get(0..6);
        let top = sum.get(6..8);
        let left = sum.get(8..10);
        let (Some(color), Some(top), Some(left)) = (color, top, left) else {
            return HttpResponse::UnprocessableEntity().body("invalid checksum");
        };

        if !color.chars().all(|c| c.is_ascii_hexdigit()) {
            return HttpResponse::UnprocessableEntity().body("invalid hex color");
        }

        let top = hex::decode(top).unwrap()[0];
        let left = hex::decode(left).unwrap()[0];
        output.push(formatdoc! {
          r#"<div style="background-color:#{color};top:{top}px;left:{left}px;"></div>"#,
        });
    }
    let output = output.join("\n");
    println!("{}", output);
    HttpResponse::Ok().body(output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_toml() {
        let file = include_str!("../test-Cargo.lock");
        let file: Lockfile = toml::from_str(file).unwrap();
        dbg!(file);
    }
}
