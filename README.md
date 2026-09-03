# rust-book-web-server

A single-threaded, then multithreaded, HTTP web server built from scratch in
Rust — the final project of
[*The Rust Programming Language*](https://doc.rust-lang.org/book/) book
(chapter 21), extended a bit further as a personal learning exercise.

Built entirely on `std` — no web framework, no async runtime. The point is
to see what `TcpListener`, thread pools, `Arc`/`Mutex`/`mpsc`, and `Drop`-based
graceful shutdown actually look like underneath the frameworks that normally
hide them.

## What it does

- Listens on `127.0.0.1:7878` and serves a static HTML page for `GET /`, or a
  404 page for anything else.
- Handles requests concurrently via a hand-rolled `ThreadPool` (fixed-size
  worker threads pulling jobs off a shared `mpsc` channel, guarded by
  `Arc<Mutex<Receiver<Job>>>`).
- Shuts down gracefully: dropping the `ThreadPool` closes the channel, which
  signals every worker to finish its current job and exit, then joins each
  worker thread before the process exits.

## Running it

```sh
cd hello
cargo run
```

Then, in another terminal:

```sh
curl http://127.0.0.1:7878/
curl http://127.0.0.1:7878/anything-else   # 404
```

## Layout

- [hello/](hello/) — the server itself (a `cargo new hello` crate).
  - `src/main.rs` — the listener loop, request routing, response building.
  - `src/lib.rs` — `ThreadPool` and `Worker`.
- [_docs/](_docs/) — project notes:
  - [project-roadmap.md](_docs/project-roadmap.md) — where this goes after
    the book: real HTTP parsing, routing, structured errors, tests. Stays
    synchronous and `std`-only on purpose — Tokio/Axum work happens in the
    separate `rust-elt-api` project instead.
  - [todo.md](_docs/todo.md) — smaller loose ends noticed along the way.

## Status

Chapter 21 (single-threaded → thread pool → graceful shutdown) is complete.
See the roadmap for what's next.
