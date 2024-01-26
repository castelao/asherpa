use reqwest::get;

type Error = Box<dyn std::error::Error>;

async fn download<W: std::io::Write>(url: &str, mut wtr: W) -> Result<usize, Error> {
    //let url = "https://www.ngdc.noaa.gov/thredds/fileServer/global/ETOPO2022/60s/60s_bed_elev_netcdf/ETOPO_2022_v1_60s_N90W180_bed.nc";
    //let hash = "e7e7efb75230280126bc96e910f71010";
    // 491284376
    //let url = "https://www.ngdc.noaa.gov/mgg/global/relief/ETOPO2/ETOPO2v2-2006/ETOPO2v2g/netCDF/ETOPO2v2g_f4_netCDF.zip";
    // https://raw.githubusercontent.com/castelao/asherpa/main/Cargo.toml

    let body = reqwest::get(url).await?.bytes().await?;
    wtr.write_all(&body)?;
    /*
        dbg!("filename", filename);
        let mut fp = std::io::BufWriter::new(
            std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(filename)?,
        );

        let mut res = ureq::get(url).call()?.into_reader().take(500_000_000);
        let mut buffer = vec![];
        res.read_to_end(&mut buffer)?;
        let s = fp.write(buffer.as_slice())?;
        dbg!(s);
        //while let Some(chunk) = res.chunk().await? {
    */
    Ok(body.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn demo() {
        let mut body = vec![];
        {
            let size = download(
                "https://raw.githubusercontent.com/castelao/asherpa/main/Cargo.toml",
                &mut body,
            )
            .await
            .unwrap();
            dbg!(size);
        }
        dbg!(&body);
        assert!(false);
    }
}
