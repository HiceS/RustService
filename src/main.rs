// progress bar
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

// requests
use futures::{stream, StreamExt}; // 0.3.1
use reqwest::Client; // 0.10.0

// async handler integration
use tokio;
use bytes::Bytes;
use bytes::Buf;

// for json parsing easily - future maintanance
// use serde::{Deserialize, Serialize};

// IO and File Access
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

/*
#[derive(Serialize, Deseialize)]
struct Links{
    links: Vec<&Link>,
}
*/

// Globals for download and install
// this should really be a json file

const SOURCES: [&'static str; 2] = ["S_1", "D_1"];

const DESTINATIONS: [&'static str; 2] = ["S_1", "D_1"];

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let mut resp = reqwest::get("https://github.com/Autodesk/synthesis/releases/download/v4.3.3/SynthesisSampleAssets.zip").await?;


    let mut content_length = resp.headers().get("content-length").unwrap().to_str().unwrap();
    // println!("{:#?}", content_length);

    let mut content = content_length.parse::<u64>().unwrap();
    
    let bar = ProgressBar::new(content);
    bar.set_style(ProgressStyle::default_bar()
    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
    .progress_chars("##-"));

    let content_info = resp.headers().get("content-disposition").unwrap();
    // println!("{:#?}", content_info);

    //let mut buffer = [0; content];

    // https://stackoverflow.com/questions/25225346/how-do-you-copy-between-arrays-of-different-sizes-in-rust
    
    while let Some(mut chunk) = resp.chunk().await? {
        //println!("{:#?}", chunk.get_u64());
        //chunk.copy_to_slice(&mut buffer);
        
        println!("{:#?}", chunk.bytes_vectored());
        bar.inc(chunk.len() as u64);

        if (chunk.len() == (content as usize)){
            bar.finish();
        }
    }

    //let result = resp.text().await?;


    let mut out = File::create("final.zip");
    //io::copy(&mut )
    //println!("{:#?}", result);

    Ok(())
}