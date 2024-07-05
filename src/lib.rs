use reqwest;
use tokio;

type Error = Box<dyn std::error::Error>;

struct Artifact {
    url: String,
    filename: Option<String>,
    size: Option<usize>,
    hash: Option<String>,
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

// Would a stream be more efficient here? As it is, it would first get
// all the content before try to save it. Wouldn't be better to alternate
// with chunks? Download a block and save it?
async fn download<P: AsRef<Path>>(url: &str, filename: P) -> Result<usize, Error> {
    //let url = "https://www.ngdc.noaa.gov/thredds/fileServer/global/ETOPO2022/60s/60s_bed_elev_netcdf/ETOPO_2022_v1_60s_N90W180_bed.nc";
    //let hash = "e7e7efb75230280126bc96e910f71010";
    // 491284376
    //let url = "https://www.ngdc.noaa.gov/mgg/global/relief/ETOPO2/ETOPO2v2-2006/ETOPO2v2g/netCDF/ETOPO2v2g_f4_netCDF.zip";
    // https://raw.githubusercontent.com/castelao/asherpa/main/Cargo.toml

    dbg!("Inside download");
    let mut fp = std::io::BufWriter::new(
        std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(filename)?,
    );

    let size = get(url, &mut fp).await?;
    tracing::trace!("Downloaded {} bytes", size);
    fp.flush()?;
    tracing::trace!("Flushed");
    Ok(size)
}

// Maybe we should target on having one single public function that
// downloads a list of files asyncronously and return when all is done.
fn demo(url: &str, filename: &str) -> Result<String, Error> {
    let mut path = dirs::cache_dir().unwrap();
    path.push(filename);
    //dbg!(reqwest::Url::parse(url).unwrap().to_file_path());
    //use std::hash::{DefaultHasher, Hash, Hasher};

    //let mut hasher = DefaultHasher::new();
    //url.hash(&mut hasher);
    //let filename = format!("{:x}", hasher.finish());
    dbg!("Inside demo");
    dbg!(&path);
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Unable to create a runtime");
    let size = rt.block_on(download(url, &path))?;
    dbg!(size);
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
            "https://raw.githubusercontent.com/castelao/asherpa/main/Cargo.toml",
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
