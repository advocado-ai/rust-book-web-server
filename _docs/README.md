# rust-book-web-server Documentation

This directory contains project planning and learning notes for the standalone
web server project, started from *The Rust Programming Language* book's final
project (chapter 21: building a single-threaded, then multithreaded, then
gracefully-shutting-down web server).

- [Project roadmap](project-roadmap.md)

The book's version lives in the main learning repo at
`rust-programming-language-book/capstone_webserver/` and is left as-is as a
reference. This project starts over from scratch, reusing what that version
taught (`TcpListener`, `ThreadPool`, `Arc`/`Mutex`/`mpsc`, `Drop`-based
shutdown) as the foundation for real HTTP handling, routing, and — eventually
— an async rewrite. Standalone so it can have its own Cargo manifest, tests,
README, and Git history, matching the `rust-cli-elt` project's convention.
