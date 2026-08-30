# Web Server Project Roadmap

## Goal

The book's final project stops at: single-threaded server → thread pool →
graceful shutdown, using only `std`. That's a great tour of ownership,
threads, `Arc`/`Mutex`/`mpsc`, and `Drop` — but the server itself is
deliberately minimal (one hardcoded route, string-matching instead of real
HTTP parsing, `.unwrap()` everywhere). This roadmap picks up from there in
two phases: first deepen the synchronous version into something closer to a
real (if small) HTTP server, then do a second pass rewriting it on async/Tokio.

Phase 1 stays synchronous on purpose — same threading model as the book
(`std::thread`, `ThreadPool`), so the jump in complexity is in HTTP handling
and API design, not a new concurrency paradigm at the same time.

## Phase 1 — Sync server, closer to real HTTP

1. **Real request parsing.** Replace `request_line == "GET / HTTP/1.1"`
   string comparison with actually parsing the method, path, HTTP version,
   and headers out of the request. Handle a request line that doesn't match
   the expected shape without panicking.
2. **Routing.** Support more than one path. A small router — even just a
   `match` on `(method, path)` — that dispatches to different handler
   functions, plus a real fallback 404 for unmatched routes (not just one
   hardcoded alternate path).
3. **Static file serving, generalized.** Serve arbitrary files from a
   `public/` directory based on the request path, with path traversal
   protection (reject `..` in the requested path — a real security concern,
   not just a toy detail).
4. **Structured errors.** Replace `.unwrap()` calls in request handling with
   real error handling — a request that fails to parse, or a file that
   doesn't exist, should produce a proper HTTP error response instead of
   crashing the worker thread. This is the same `Result`/custom-error-enum
   muscle from Rustlings' `13_error_handling`, applied to a real I/O path.
5. **JSON request/response bodies.** Add at least one route that accepts or
   returns JSON (`serde` + `serde_json`), rather than only serving static
   HTML. Good opportunity to practice `Deserialize`/`Serialize` derives.
6. **Configuration.** Move the bind address, port, thread pool size, and
   static file root out of hardcoded constants and into a config source
   (CLI args via `clap`, and/or a config file, and/or env vars — `rust-cli-elt`
   already has env var and arg-parsing precedent to build on).
7. **Logging.** Replace `println!`/`eprintln!` with the `log` crate + a
   backend (`env_logger` is the simplest). Log each request's method, path,
   status, and duration.
8. **Tests.** Unit tests for the request parser and router logic (pure
   functions, no sockets needed). Integration tests that spin up the server
   on a test port and make real requests against it (`std::net` client side,
   or a lightweight HTTP client crate).
9. **Keep-alive / connection reuse.** The book's version closes the
   connection after one response. HTTP/1.1 keep-alive (reading multiple
   requests off the same `TcpStream`) is a meaningful step up in protocol
   correctness and a good forcing function for cleaner loop/state handling
   in `handle_connection`.
10. **Graceful shutdown, revisited.** The book's `Drop`-based shutdown
    doesn't handle an OS signal (Ctrl+C) — add a `ctrlc`-crate or
    signal-based trigger that stops accepting new connections and lets
    in-flight ones finish before exiting.

## Phase 2 — Async rewrite

Once Phase 1 feels solid, redo the server on `tokio` (+ likely `axum` or
`hyper` directly, depending on how much you want the framework to do for
you vs. hand-roll). This ties directly into the fast-track plan's Step 4
(async/await, futures, Tokio internals) and the existing
`17_async_await_futures_streams/` work in the main learning repo.

1. Swap `std::net::TcpListener`/`TcpStream` for `tokio::net` equivalents;
   understand why `.await` replaces the thread-per-connection model.
2. Replace the hand-rolled `ThreadPool` with Tokio's task scheduler — notice
   what disappears (no more manual `Worker`/`mpsc`/`Arc<Mutex<Receiver>>`)
   and think through *why* it disappears (cooperative task scheduling on a
   thread pool Tokio manages, vs. OS threads you manage yourself).
3. Revisit graceful shutdown using `tokio::signal` and cancellation
   (`tokio_util::sync::CancellationToken` or a broadcast channel) instead of
   `Drop`.
4. If using `axum` or `hyper`: compare how much of Phase 1's hand-rolled
   parsing/routing/error-handling work a real framework replaces, and be
   able to explain *why* each piece is no longer needed — same "intermediate
   before idiomatic" habit from Rustlings, now at the framework level.
5. Load-test the async version against the Phase 1 sync version (e.g. with
   `wrk` or `oha`) to see the concurrency difference concretely, not just
   read about it.

## Rust practice targets

- Real-world error handling (`thiserror`/`anyhow`, `Result` chains, `?`)
  instead of exercise-shaped `Result<T, MyEnum>`
- Trait objects vs. generics for interchangeable handlers/formats
- Ownership and borrowing across a request's lifecycle (headers, body,
  response — who owns what, and when it's safe to borrow vs. must own)
- `Arc`/`Mutex` for any real shared state (e.g. a request counter, an
  in-memory cache) beyond the book's channel-only example
- Async/await, `Future`, `Pin` basics (Phase 2)
- Integration testing a network service, not just pure functions

## Suggested dependencies

- `serde` / `serde_json` — JSON bodies
- `clap` — CLI configuration
- `log` + `env_logger` — logging
- `thiserror` — library-side error types
- `ctrlc` — signal-based graceful shutdown (Phase 1)
- `tokio`, `axum` or `hyper` — Phase 2 async rewrite
- `wrk` or `oha` (external tools, not crates) — load testing to compare
  Phase 1 vs. Phase 2

## Showcase checklist

- `cargo fmt` passes
- `cargo clippy` passes
- `cargo test` passes (unit + integration)
- README includes runnable examples (`curl` commands against each route)
- Error behavior is documented (what each failure mode returns and why)
- Phase 2 section notes what changed vs. Phase 1 and why, not just that it
  was rewritten
