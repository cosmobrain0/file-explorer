# File Explorer > Prototype 1
> A file explorer written in Rust for the command line

This project uses `crossterm`, so it should run on everything (I think), but I only intend
to test it on Windows (sorry).

## Purpose
This project has multiple purposes, listed below in no particular order:
- Efficiency: I want to be able to do as many things in the terminal as possible, but
  navigating the file system and zipping/extracting/moving/editing/deleting/creating files
  is currently *waaay* too inconvenient. This will make things more convenient and less
  error prone, and therefore faster.
- Practice: I want to get better at Rust, and make my own good TUI library, and this
  project is good for both of those things.
- Fun: it'll be fun to make this

## Features
- [ ] view files and folders in current directory
- [ ] navigate file system
- [ ] change current working directory
- [ ] view UTF-8 files
- [ ] view files with basic syntax highlighting for common languages
- [ ] scroll in file view
- [ ] view file metadata
- [ ] search for files / folders
- [ ] be able to create / delete files
- [ ] be able to select files to zip
- [ ] be able to extract files
- [ ] be able to run shell commands without exiting the program

As shown by the empty check boxes, none of these features have been implemented yet.

## Feature Requests
This project is currently only intended for personal use so I might not program a feature
which someone else requests, but if you want to see a feature in this project and it sounds
like fun to program or it sounds like it would be really useful then I'll probably do it.

## Contributing
If you would like to contribute, feel free to do that! If you want me to merge it with this
then you can make a request and if your code is clear enough that I can understand it and
it's been tested then I'll merge it. If you would like to discuss this project then
you can DM me on Discord: `cosmobrain`

## TODO > Prototype 1
this section is more of a note-to-self kinda thing
- Make it update itself properly and allow for navigating the file system
- show files in a `DirFilesView` window
- Refactor `DirectoryView` thing to use `State` to some extent
- give `DirectoryView` and `DirFilesView` a max size and scrolling capabilities
- fix `DirFilesView` so it doesn't redraw every frame
