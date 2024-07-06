use reqwest;
use tokio;

type Error = Box<dyn std::error::Error>;

#[derive(Debug)]
struct Artifact {
    url: String,
    filename: Option<String>,
    size: Option<usize>,
    hash: Option<String>,
}

impl Artifact {
    fn url(&self) -> &str {
        &self.url
    }
}

// Should this have a size limit?
async fn get<W: std::io::Write>(url: &str, mut wtr: W) -> Result<usize, Error> {
    tracing::debug!("Getting from: {}", url);
    let mut res = reqwest::get(url).await?;
    let mut size = 0;
    while let Some(chunk) = res.chunk().await? {
        tracing::trace!("Chunk size: {}", chunk.len());
        size += chunk.len();
        wtr.write_all(&chunk)?;
    }
    Ok(size)
}

async fn download<P: AsRef<Path>>(url: &str, filename: P) -> Result<usize, Error> {
    tracing::debug!("Downloading from: {}", url);
    let mut fp = std::io::BufWriter::new(
        std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(filename)?,
    );

    let size = get(artifact.url(), &mut fp).await?;
    tracing::trace!("Downloaded {} bytes", size);
    fp.flush()?;
    tracing::trace!("Flushed");
    Ok(size)
}

// Maybe we should target on having one single public function that
// downloads a list of files asyncronously and return when all is done.
fn demo(artifact: &Artifact, filename: &str) -> Result<String, Error> {
    let mut path = dirs::cache_dir().unwrap();
    path.push(filename);
    //dbg!(reqwest::Url::parse(url).unwrap().to_file_path());
    //use std::hash::{DefaultHasher, Hash, Hasher};

    //let mut hasher = DefaultHasher::new();
    //url.hash(&mut hasher);
    //let filename = format!("{:x}", hasher.finish());
    dbg!(&path);
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Unable to create a runtime");
    let _size = rt.block_on(download(artifact, &path))?;
    /*
    block_on(download(
        "https://raw.githubusercontent.com/castelao/asherpa/main/Cargo.toml",
        &path,
    ));
    */

    let path = path.into_os_string().into_string().unwrap();
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    // Just testing a concept
    fn demo_block() {
        let filename = demo(
            &Artifact {
                url: "https://raw.githubusercontent.com/castelao/asherpa/main/Cargo.toml"
                    .to_string(),
                filename: None,
                size: None,
                hash: None,
            },
            "testing.txt",
        )
        .unwrap();
        assert!(std::path::Path::new(&filename).exists());
        std::fs::remove_file(filename).unwrap();
    }

    #[tokio::test]
    async fn demo_async() {
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
    }
}
