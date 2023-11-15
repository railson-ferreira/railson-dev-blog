mod post_identification;
mod dynamo_db;

use std::process::Command;
use std::fs;
use std::env;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::PathBuf;
use chrono::{DateTime, SecondsFormat};
use crate::post_identification::PostIdentification;


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let posts_path = args.get(1);

    let posts_folder_path = match posts_path {
        Some(value) => value,
        None => panic!("Please inform the posts path as argument")
    };
    let git_revision = get_git_revision(posts_folder_path.to_string());
    let paths = fs::read_dir(posts_folder_path.to_string()).unwrap();

    let client = dynamo_db::get_client().await;

    for path_result in paths {
        let path = path_result.unwrap();
        if path.file_type().unwrap().is_dir() {
            continue;
        }
        if !path.file_name().to_str().unwrap().ends_with(".md") {
            continue;
        }
        println!("reading: {}", path.file_name().to_str().unwrap());
        let post_identification = get_post_identification(path.path());

        // TODO: download posts list and verify whether it's already uploaded
        // if posts_list.contains(post_identification)

        let uid = post_identification.uid;
        let updating_date_time = DateTime::from_timestamp(post_identification.timestamp, 0).unwrap().to_rfc3339_opts(SecondsFormat::Secs,true);
        let content_markdown = get_post_content(path.path());

        println!("uploading: {} | uid({}) | updating_date_time({})", path.file_name().to_str().unwrap(), uid, updating_date_time);
        dynamo_db::insert_post(
            &client,
            uid,
            updating_date_time,
            git_revision.to_string(),
            path.file_name().to_str().unwrap().to_string(), content_markdown,
        ).await.expect(format!("Fail to insert item '{}'", path.file_name().to_str().unwrap()).as_str());
    }
}


fn get_git_revision(posts_folder: String) -> String {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .current_dir(posts_folder)
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());

    let revision = String::from_utf8_lossy(&output.stdout).as_ref().to_string();

    assert!(revision.len() > 0);

    return revision;
}


fn get_post_identification(path: PathBuf) -> PostIdentification {

    // Read the first line of the file into `title`.
    let file = match fs::File::open(&path) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read file {:?}", &path),
    };
    let mut buffer = BufReader::new(file);

    // Skip initial line( must be exactly '---')
    buffer.seek(SeekFrom::Start(4)).expect("Fail to skip first characters '---'");

    let mut post_uid_prop = String::new();
    let mut post_timestamp_prop = String::new();

    buffer.read_line(&mut post_uid_prop).expect("Fail to read 'uid' line");
    buffer.read_line(&mut post_timestamp_prop).expect("Fail to read 'timestamp' line");

    assert!(post_uid_prop.starts_with("uid: "));
    assert!(post_timestamp_prop.starts_with("timestamp: "));


    let post_uid = post_uid_prop.replace("uid: ", "").replace("\"", "").replace("\n", "");
    let post_timestamp_iso = post_timestamp_prop.replace("timestamp: ", "").replace("\n", "Z");

    let timestamp = DateTime::parse_from_rfc3339(post_timestamp_iso.as_str()).unwrap();

    return PostIdentification {
        uid: post_uid,
        timestamp: timestamp.timestamp(),
    };
}

fn get_post_content(path: PathBuf) -> String {

    // Read the first line of the file into `title`.
    let file = match fs::File::open(&path) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read file {:?}", &path),
    };
    let mut buffer = BufReader::new(file);

    // Skip initial line( must be exactly '---')
    let mut content_markdown = String::new();
    buffer.read_to_string(&mut content_markdown).expect("Faill to read content");

    return content_markdown;
}