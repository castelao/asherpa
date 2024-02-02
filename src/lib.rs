use reqwest;

type Error = Box<dyn std::error::Error>;

// Should this have a size limit?
async fn get<W: std::io::Write>(url: &str, wtr: &mut W) -> Result<usize, Error> {
    let mut res = reqwest::get(url).await?;
    let mut size = 0;
    while let Some(chunk) = res.chunk().await? {
        size += chunk.len();
        wtr.write_all(&chunk)?;
    }
    Ok(size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn demo() {
        let mut body = vec![];
        {
            let size = get(
                "https://raw.githubusercontent.com/castelao/asherpa/main/Cargo.toml",
                &mut body,
            )
            .await
            .unwrap();
            dbg!(size);
        }
        assert!(false);
    }
}
