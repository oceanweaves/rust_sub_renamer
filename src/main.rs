use std::env;
use std::fs;
use regex::Regex;
use std::path::Path;


fn extract_episode_number(filename: &str) -> Option<u32> {
    let patterns = vec![
        Regex::new(r"(?i)(?:S\d{1,2}E|E)(\d{1,2})").unwrap(),
        Regex::new(r"(?i)^(\d{1,2})\.").unwrap(),
        Regex::new(r"(?i)(?:EP|E)(\d{1,2})").unwrap(),
        Regex::new(r"(?i)第(\d{1,2})話").unwrap(),
        Regex::new(r"(\d{1,2})\s*\[").unwrap(),
        Regex::new(r"(?i)\[.*?\]\[(\d{1,2})\]").unwrap(),
        Regex::new(r"(?i)\[x_x\].*?\[Ep(\d{1,2})\]").unwrap(),
    ];

    for pattern in patterns {
        let mut captures = pattern.captures_iter(filename).filter(|cap| {
            let episode_number = cap.get(1).unwrap().as_str();
            let resolution = format!("{}P", episode_number);
            !filename.contains(&resolution)
        });

        if let Some(capture) = captures.next() {
            return capture.get(1).map(|m| m.as_str().parse::<u32>().unwrap());
        }
    }

    None
}



fn main() {
    let video_extensions = [".mkv", ".mp4", ".MKV", ".MP4"];
    let subtitle_extensions = [".ass", ".ssa", ".srt", ".ASS", ".SRT", ".SSA", ".sub", ".SUB"];

    // Get the path of the executable
    let exe_path = env::current_exe().unwrap_or_else(|e| {
        println!("Error getting the executable path: {}", e);
        std::process::exit(1);
    });

    // Use the parent directory of the executable as the current directory
    let current_dir = exe_path.parent().unwrap_or_else(|| {
        println!("Error getting the parent directory of the executable");
        std::process::exit(1);
    });

    // Print the current directory
    println!("Current directory: {:?}", current_dir);

    let entries = fs::read_dir(&current_dir).unwrap_or_else(|e| {
        println!("Error reading the current directory: {}", e);
        std::process::exit(1);
    });

    let mut video_files = Vec::new();
    let mut subtitle_files = Vec::new();
    
    for entry in fs::read_dir(current_dir).expect("Failed to read directory") {
    let entry = entry.expect("Failed to read entry");
    let file_name = entry.file_name().into_string().expect("Failed to convert OsString to String");
    let path = entry.path();
    let file_ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    if video_extensions.contains(&file_ext) {
        println!("Video file: {}, Episode number: {:?}", file_name, extract_episode_number(&file_name));
        video_files.push(file_name);
    } else if subtitle_extensions.contains(&file_ext) {
        println!("Subtitle file: {}, Episode number: {:?}", file_name, extract_episode_number(&file_name));
        subtitle_files.push(file_name);
    }
    }
        


    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if path.is_file() && path.metadata().unwrap().len() > 200 * 1024 * 1024 {
            if video_extensions.iter().any(|ext| file_name.ends_with(ext)) {
                video_files.push(file_name.to_string());
            }
        }

        if path.is_file() && subtitle_extensions.iter().any(|ext| file_name.ends_with(ext)) {
            subtitle_files.push(file_name.to_string());
        }
    }

    if video_files.len() == subtitle_files.len() {
        let mut rename_operations = Vec::new();

        let video_files_dict: std::collections::HashMap<u32, String> = video_files.into_iter().filter_map(|f| {
            extract_episode_number(&f).map(|n| (n, f))
        }).collect();

        for subtitle_file in subtitle_files {
            if let Some(episode_number) = extract_episode_number(&subtitle_file) {
                if let Some(video_file) = video_files_dict.get(&episode_number) {
                    let video_name = Path::new(video_file).file_stem().unwrap().to_str().unwrap();
                    let subtitle_ext = Path::new(&subtitle_file).extension().unwrap().to_str().unwrap();
                    let new_subtitle_name = format!("{}.{}", video_name, subtitle_ext);
                    rename_operations.push((subtitle_file, new_subtitle_name));
                }
            }
        }

        // Sort the rename_operations list by episode number
        rename_operations.sort_by_key(|x| extract_episode_number(&x.0).unwrap());

        if rename_operations.is_empty() {
            println!("No matching video and subtitle files found. Exiting.");
            std::process::exit(1);
        }

        // Print the sorted preview and perform the renaming
        for (old_name, new_name) in rename_operations {
            let old_path = current_dir.join(&old_name);
            let new_path = current_dir.join(&new_name);
            if let Err(e) = fs::rename(&old_path, &new_path) {
                println!("Failed to rename: {} -> {}. Error: {}", old_name, new_name, e);
            } else {
                println!("Renamed: {} -> {}", old_name, new_name);
            }
        }

    } else {
        println!("Numbers of video files and subtitle files do not match. Video files found: {}, subtitle files found: {}", video_files.len(), subtitle_files.len());
    }
}
