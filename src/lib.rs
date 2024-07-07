use reqwest;
use tokio;

type Error = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct Artifact {
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

#[derive(Debug)]
pub struct Resource {
    url: String,
    filename: String,
    size: usize,
    hash: String,
}

// Should this have a size limit?
/// Read from a URL and write to a writer
///
/// # Parameters
///
/// - `url`: The URL to read from.
/// - `wtr`: The writer to write to.
async fn write<W: std::io::Write>(url: &str, mut wtr: W) -> Result<usize, Error> {
    tracing::debug!("Getting from: {}", url);
    let mut res = reqwest::get(url).await?;
    let mut size = 0;
    while let Some(chunk) = res.chunk().await? {
        tracing::trace!("Chunk size: {}", chunk.len());
        size += chunk.len();
        wtr.write_all(&chunk)?;
        // wtr.write_all(&chunk).await?;
    }
    tracing::debug!("Done writing {} bytes", &size);
    Ok(size)
}

// Would a stream be more efficient here? As it is, it would first get
// all the content before try to save it. Wouldn't be better to alternate
// with chunks? Download a block and save it?
/// Download one single artifact into a file
async fn get<P: AsRef<Path>>(artifact: &Artifact, filename: P) -> Result<Resource, Error> {
    //let url = "https://www.ngdc.noaa.gov/thredds/fileServer/global/ETOPO2022/60s/60s_bed_elev_netcdf/ETOPO_2022_v1_60s_N90W180_bed.nc";
    //let hash = "e7e7efb75230280126bc96e910f71010";
    // 491284376
    //let url = "https://www.ngdc.noaa.gov/mgg/global/relief/ETOPO2/ETOPO2v2-2006/ETOPO2v2g/netCDF/ETOPO2v2g_f4_netCDF.zip";
    tracing::debug!("Downloading: {:?}", artifact);
    // let mut fp = tokio::fs::File::create(&filename).await?;
    let mut fp = std::io::BufWriter::new(
        std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(filename)?,
    );
    //tracing::debug!("Will save artifact as: {:}", &filename);

    let size = write(artifact.url(), &mut fp).await?;
    tracing::trace!("Downloaded {} bytes", size);
    let resource = Resource {
        url: artifact.url.clone(),
        filename: filename.as_ref().as_os_str().to_str().unwrap().into(),
        size,
        hash: "te enganei".to_string(),
    };
    fp.flush()?;
    tracing::trace!("Flushed");
    Ok(resource)
}

// Maybe we should target on having one single public function that
// downloads a list of files asyncronously and return when all is done.
pub fn download(artifacts: Vec<Artifact>) -> Result<String, Error> {
    let path = dirs::cache_dir().unwrap();
    //path.push(filename);
    //dbg!(reqwest::Url::parse(url).unwrap().to_file_path());
    //use std::hash::{DefaultHasher, Hash, Hasher};

    // Download all artifacts asyncronously
    let tasks: Vec<_> = artifacts
        .iter()
        .map(|a| {
            let mut p = path.clone();
            p.push(a.filename.as_ref().unwrap());
            get(a, p)
        })
        .collect();
    //let mut hasher = DefaultHasher::new();
    //url.hash(&mut hasher);
    //let filename = format!("{:x}", hasher.finish());
    dbg!(&path);
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Unable to create a runtime");
    let resources = rt.block_on(async {
        // Use match and better error handling
        futures::future::join_all(tasks)
            .await
            .into_iter()
            .collect::<Vec<_>>()
    });
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
            let size = write(
                "https://raw.githubusercontent.com/castelao/asherpa/main/Cargo.toml",
                &mut body,
            )
            .await
            .unwrap();
            dbg!(size);
        }
    }
}
