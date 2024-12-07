use actix_web::get;

#[get("/")]
pub async fn day_start() -> &'static str {
    "Hello bird!"
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
