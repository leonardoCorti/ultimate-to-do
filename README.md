# Ultimate Todo

WiP!

Ultimate Todo is an open-source, self-hostable task manager that aims to fill the gap for an open source to-do list application that's easy to sync with other platforms and services while providing uniform interfaces across various platforms. Inspired by Todoist and Taskwarrior, this project strives to offer a fast, ubiquitous, and easily synchronizable task management solution without the limitations of proprietary services.

## current status
There is nothing ready. I just started, this readme is just a roadmap and declaration of what this project aims to be.

## Features

### Ubiquitous
Ultimate Todo is designed to be accessible from any device, ensuring maximum reachability. The planned platforms include:

- Library to implement your own client
- CLI Program
- Desktop Application
- Mobile Application
- Web Application

### Fast
To ensure the best performance and efficiency, Ultimate Todo is built with:

- Rust: A fast, memory-safe programming language
- Local-First Approach: Works offline by default
- Natural Language Processing: Quickly add and parse tasks with intuitive text-based inputs

### Simple

- SQLite or YAML for project managment to provide a human-readable format 
- simple interface with sensible defaults with powerful configuration options

### Easily Synchronizable
Synchronization is a key aspect of Ultimate Todo, featuring:

- Optional Self-Hostable Server: Use your own infrastructure for complete data control
- Single-File Database: Sync your tasks however you prefer
- Integration with Existing Tools: Compatibility with CalDAV, Todoist, Taskwarrior, and potentially other task managers

## Roadmap
1. **Core library and CLI** - develop a minimal cli using a YAML-based backend.
2. **Server and synchronization** - implement a self-hostable server and introduce SQLite as an alternative backend.
3. **Desktop gui** - build a cross-platform desktop application.
4. **Stabilization phase** – Before expanding to external synchronization, refine core features and ensure robust CLI and GUI interaction.
5. **External synchronization** - Enable syncing with third-party services.
6. **Web app** - develop a web-based interface mirroring the desktop experience.
7. **Android app** - create a dedicated android app.
