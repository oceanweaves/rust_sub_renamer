# rust_sub_renamer

## Goal
To rename the subtitle files to match the corresponding video files, so the media player will automatically load the subtitles.

## Description
This script, developed with the help of ChatGPT, scans the current directory for video and subtitle files. It then extracts the episode numbers from the filenames and matches the subtitle files with the corresponding video files. If the number of video files and subtitle files match, the script renames the subtitle files to have the same name as the video files, while keeping their original file extensions.

Note: Only video files with a file size greater than 200 MB will be considered.

## Supported File Formats
Video formats: `.mkv, .mp4`
Subtitle formats: `.ass, .ssa, .srt, .sub`

## Usage
Build the executable with
`cargo build`

Put the exe in the directory containing the video and subtitle files and then run it.

The script will display the renaming operations performed. If there are no matching video and subtitle files, or if the numbers of video and subtitle files do not match, the script will print an appropriate message and exit.
