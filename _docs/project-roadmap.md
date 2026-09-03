# Web Server Project Roadmap

## Goal

The book's final project stops at: single-threaded server → thread pool →
graceful shutdown, using only `std`. That's a great tour of ownership,
threads, `Arc`/`Mutex`/`mpsc`, and `Drop` — but the server itself is
deliberately minimal (one hardcoded route, string-matching instead of real
HTTP parsing, `.unwrap()` everywhere). This roadmap picks up from there with
one phase: deepen the synchronous version into something closer to a real
(if small) HTTP server, using only `std` throughout.

Staying synchronous is deliberate — same threading model as the book
(`std::thread`, `ThreadPool`), so the focus stays on HTTP handling and API
design, not a new concurrency paradigm at the same time.

**Scoping decision (2026-09-03):** not doing the full Phase 1 list. Doing
the concept-dense items only — #1 (real parsing), #2 (routing), #4
(structured errors), and probably #9 (keep-alive) — since those teach Rust
fundamentals (parsing, `Result`/`?`, custom error enums, protocol state)
worth having solid on their own merits. Deferring #5 (JSON/serde), #6
(config/clap), and #7 (logging) — those are mostly "learn a popular crate's
API" rather than "learn a Rust concept." #3 (static file serving w/
traversal protection), #8 (tests), and #10 (signal-based shutdown) are still
worth doing if time allows, lower priority than 1/2/4/9.

**No Phase 2 / async rewrite here (2026-09-03):** this repo stays `std`-only
and synchronous, full stop — Tokio/Axum learning happens in the separate
`rust-elt-api` project instead (a dedicated `elt-core` domain crate +
`api` Axum adapter, see its own `_docs/project-roadmap.md`), which also owns
the deployment work. Rewriting *this* server on Tokio would have been
redundant with that project's Part 2. This repo's scope ends once the
items above are done.

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

## Rust practice targets

- Real-world error handling (`thiserror`/`anyhow`, `Result` chains, `?`)
  instead of exercise-shaped `Result<T, MyEnum>`
- Trait objects vs. generics for interchangeable handlers/formats
- Ownership and borrowing across a request's lifecycle (headers, body,
  response — who owns what, and when it's safe to borrow vs. must own)
- `Arc`/`Mutex` for any real shared state (e.g. a request counter, an
  in-memory cache) beyond the book's channel-only example
- Integration testing a network service, not just pure functions

## Suggested dependencies

- `thiserror` — library-side error types
- `ctrlc` — signal-based graceful shutdown, if #10 gets picked up
- (JSON/`serde`, CLI config/`clap`, and logging/`log` deliberately not
  pulled in here — see scoping decision above)

## Showcase checklist

- `cargo fmt` passes
- `cargo clippy` passes
- `cargo test` passes (unit + integration)
- README includes runnable examples (`curl` commands against each route)
- Error behavior is documented (what each failure mode returns and why)
