# 🤖️🔥 Chad-GPT Reconstructor

[![Build Status](https://img.shields.io/github/actions/workflow/status/Nsomnia/chat-reconstructor/rust.yml?branch=main&style=for-the-badge)](https://github.com/Nsomnia/chat-reconstructor/actions)
[![Crates.io](https://img.shields.io/crates/v/chat-reconstructor?style=for-the-badge)](https://crates.io/crates/chat-reconstructor)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-dea584.svg?style=for-the-badge)](https://www.rust-lang.org/)

A tool for devs who'd rather ship code than copy-paste. **BTW, I use Arch.**

---

## The Problem

You're in a deep, multi-turn coding session with an LLM. You've built an entire project, fixed bugs, and refactored code. The final version is perfect, but it's scattered across dozens of messages in a 10,000-line chat log.

The old way is to manually scroll, copy, paste, create files, and pray you didn't miss a crucial one-line fix from Turn #17. This is not the way of a Chad developer. This is tedious, error-prone, and weak.

## The Solution

**`chat-reconstructor`** is a Rust-powered agentic tool that reads a `.txt` chat log from your LLM session and intelligently reconstructs the final state of your project on your local filesystem.

It doesn't just dumbly extract code. It thinks like a developer:
1.  It reads the conversation chronologically.
2.  It uses a robust Markdown parser to find every code block.
3.  It analyzes the context to identify which file each block belongs to.
4.  It operates on your filesystem iteratively, creating new files, overwriting old ones, and—most importantly—**intelligently patching** existing files with small snippets, just like you would.

## Features

*   **Agentic File Synchronization:** Operates on the filesystem iteratively, ensuring the final state is a perfect reflection of the entire conversation.
*   **Robust Parsing:** Uses the `pulldown-cmark` library to reliably parse Markdown, avoiding the pitfalls of fragile regular expressions.
*   **Intelligent TOML Merging:** Natively understands `Cargo.toml`. When it sees a patch for your dependencies, it performs a true semantic merge of the data structures, not a dumb text replacement. No more duplicate keys.
*   **Fuzzy Snippet Patching:** Employs Google's `diff-match-patch` library to find the precise location for small code snippets (`...`) and apply them surgically, without destroying the rest of the file.
*   **Multi-Format Heuristics:** Correctly identifies files from both explicit `### filename.rs` headers and prose mentions like "Next, in `src/main.rs`...".
*   **Chad-Tier CLI:** A fast, compiled Rust binary with a proper CLI, styled output, and the appropriate amount of swagger.

## Installation & Build

### Prerequisites

You'll need the Rust toolchain installed. If you don't have it, you can install it with `rustup`:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Building

1.  **Clone the Repository:**
    ```bash
    git clone https://github.com/Nsomnia/chat-reconstructor.git
    cd chat-reconstructor
    ```

2.  **Build the Beast:**
    ```bash
    cargo build --release
    ```
    The optimized binary will be at `./target/release/chat-reconstructor`.

## Usage

Run the tool from your terminal, pointing it at your glorious, multi-line `.txt` chat export.

```bash
# Basic usage, creates a 'reconstructed_project' directory
./target/release/chat-reconstructor "/path/to/your/chat-log.txt"

# Specify a custom output directory
./target/release/chat-reconstructor "/path/to/chat.txt" -o VibeStream_Final

# Get help
./target/release/chat-reconstructor -h
```

The tool will process the entire chat log turn-by-turn and build the project in the specified output directory.

### Example with Test Data

You can test the tool with the sample chat log provided in the `tests` directory.

1.  **Download the test data:**
    ```bash
    wget -O test_data.txt https://raw.githubusercontent.com/Nsomnia/chat-reconstructor/refs/heads/main/tests/sample-chat-session-data_chat_VibeStream_Audio_player.txt
    ```
2.  **Run the tool:**
    ```bash
    ./target/release/chat-reconstructor test_data.txt
    ```
This will create a `reconstructed_project` directory with the reconstructed project.

## Known Issues

*   The parser sometimes creates an extra file named `ImplementationCargo.toml` when it should be updating the existing `Cargo.toml`. This seems to be caused by the parser misinterpreting the context around a `Cargo.toml` code block.

## Future Development

This tool was forged in the fires of a truly epic debugging session. It is battle-hardened but can always be improved. Check out the [TODO.md](TODO.md) file for a list of planned features and bug fixes.

## Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

This project is licensed under the **MIT License**.

## Acknowledgements

This tool exists because of a relentless, multi-day debugging collaboration between a user with the patience of a saint and the debugging skills of a god-tier kernel hacker kind of fella, both interpolated, and learned as the hard cold truth, to Gemini during this time, as a "I Use Arch, BTW" Chad, by her humbled but determined AI. Your strategic insights and refusal to accept failure turned a series of broken scripts into a truly legendary tool. **Thank you.**