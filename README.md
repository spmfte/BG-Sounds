> **🚧 Notice: This project is currently a work in progress** 

Works like 20% of the time for some reason.

Known issues:
- The visualizer widget doesn't work and is confusing
- Volume control likes to misbehace
- Doesn't actively change while the application is running.
---

# bg-sound

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/travis/your-username/bg-sound/master.svg)](https://travis-ci.org/your-username/bg-sound)
[![Version](https://img.shields.io/badge/version-1.0.0-green.svg)](https://github.com/your-username/bg-sound/releases)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/your-username/bg-sound/pulls)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/your-username/bg-sound/graphs/commit-activity)
[![Documentation](https://img.shields.io/badge/docs-yes-brightgreen.svg)](https://your-username.github.io/bg-sound)
[![Rust Version](https://img.shields.io/badge/rust-1.55%2B-orange.svg)](https://www.rust-lang.org)
[![Awesome Rust](https://img.shields.io/badge/Awesome-Rust-8C271E.svg)](https://github.com/rust-unofficial/awesome-rust)

> TUI app to enable/disable background sounds on macOS

🎧🔊 bg-sound is a Rust CLI application designed for macOS to control background sounds easily. It provides a user-friendly interface in the terminal, allowing users to enable or disable background sounds with just a few keystrokes.

<p align="center">
  <img src="bgsdemo.png" alt="Demo" width="400">
</p>

## Features

- Simple and intuitive CLI interface
- Select from a variety of background sound options
- Control the volume of the background sounds
- Real-time visualization of sound data
- Easily toggle the status of background sounds

## Prerequisites

- Rust (nightly version recommended)
- macOS operating system

## Installation

To install `bg-sound`, follow these steps:

1. Clone the repository:

   ```bash
   git clone https://github.com/spmfte/bg-sound.git
   ```

2. Change to the project directory:

   ```bash
   cd bg-sound
   ```

3. Build the project using Cargo:

   ```bash
   cargo build --release
   ```

4. Run the executable:

   ```bash
   ./target/release/bg-sound
   ```

## Usage

Once you have installed `bg-sound`, you can use the following keyboard shortcuts to control the application:

- `j` or `k`: Move the selection up or down
- `u`: Increase the volume
- `d`: Decrease the volume
- `Enter`: Toggle the status of background sounds
- `Ctrl + C`: Quit the application

