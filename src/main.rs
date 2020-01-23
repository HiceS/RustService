// progress bar
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};

// requests
use reqwest::Url;

use futures::future::join_all;

// async handler integration
use tokio;
use tokio::task::JoinHandle;

use bytes::Buf;

// IO and File Access
use std::io::prelude::*;
use std::fs::File;
use std::fs::create_dir_all;
use std::io;

// Globals for download and install
// this should really be a json file

const SOURCES: [&'static str; 2] = ["https://github.com/Autodesk/synthesis/releases/download/v4.3.3/SynthesisSampleAssets.zip", "https://github.com/Autodesk/synthesis/releases/download/v4.3.3/SynthesisSampleAssets.zip"];

const DESTINATIONS: [&'static str; 2] = ["./", "./"];

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

/*
    let paths = vec![
        "https://github.com/Autodesk/synthesis/releases/download/v4.3.3/SynthesisSampleAssets.zip".to_string(),
        "https://github.com/Autodesk/synthesis/releases/download/v4.3.3/SynthesisSampleAssets.zip".to_string(),
        "https://github.com/Autodesk/synthesis/releases/download/v4.3.3/SynthesisSampleAssets.zip".to_string(),
    ];

    let mut tasks: Vec<JoinHandle<Result<(), reqwest::Error>>>= vec![];

    for path in paths{
        
        let path = path.clone();
        tasks.push(tokio::spawn(
            async move {
                get_file(&path).await;
            Ok(())    
        }));
    }

    println!("Started {} tasks. Waiting...", tasks.len());

    join_all(tasks).await;
*/

    for x in &SOURCES {
        get_file(&x.to_string()).await.expect("Could not get File");
    }

    Ok(())
}

// This isn't a thing on windows apparently - maybe i could copy all the bytes individuall but this isn't worth it at all
fn unzip (file: &File) -> i32 {

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = file.sanitized_name();

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.as_path().display());
            create_dir_all(&outpath).unwrap();
        } else {
            println!("File {} extracted to \"{}\" ({} bytes)", i, outpath.as_path().display(), file.size());
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
    return 0;
}

async fn get_file(index: &String) -> Result<(), reqwest::Error> {
    
    let url = Url::parse(index).expect("Not a valid URL");
    let mut resp = reqwest::get(url).await?;

    if resp.status().to_string() != "200 OK" {
        eprintln!("Cannot recieve connection to download file: {}", index);
        assert_eq!(resp.status().to_string(), "200 OK");
        std::process::exit(1);
    }

    let content_length = resp.headers().get("content-length").unwrap().to_str().unwrap(); // I know the response was okay so hopefully they send nice headers yo

    let content = content_length.parse::<u64>().unwrap();
    
    let bar = ProgressBar::new(content);
    bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.green/white}] {bytes}/{total_bytes} (eta: {eta}) data: {msg}")
        .progress_chars("#>_"));

    let _content_info = resp.headers().get("content-disposition").unwrap();

    bar.set_message(&format!("{}", _content_info.to_str().unwrap().to_string()));

    let mut file = File::create("test.zip").expect("Failed to create new file");

    
    while let Some(chunk) = resp.chunk().await? {

        bar.inc(chunk.len() as u64);

        if chunk.len() == (content as usize) {
            //bar.finish_with_message("Finished Downloading and Writing");
        }

        file.write(chunk.bytes()).expect("Failed to write some data to file");
    }

    //unzip(&file);

    bar.finish_with_message("Finished");
    
    Ok(())
}