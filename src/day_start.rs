use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn day_start() -> &'static str {
    "Hello, bird!"
}

#[get("/-1/seek")]
pub async fn seek() -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", "https://www.youtube.com/watch?v=9Gc4QTqslN4"))
        .finish()
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use xshell::{cmd, Shell};

    #[test]
    fn test_day_start() -> Result<()> {
        let shell = Shell::new()?;
        cmd!(shell, "/home/carltonrp/.cargo/bin/cch24-validator -1").run()?;

        Ok(())
    }
}
