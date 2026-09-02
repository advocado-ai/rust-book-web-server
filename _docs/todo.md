# TODO

Small, concrete things noticed while working through the book chapter itself
— deferred on purpose, not forgotten. Bigger post-book plans live in
[project-roadmap.md](project-roadmap.md); this file is scoped to loose ends
from the current chapter (ch21, thread pool) plus general cleanup.

## Thread pool robustness

- [ ] Replace `thread::spawn` in `Worker::new` with `std::thread::Builder::new().spawn(...)`,
      which returns `Result<JoinHandle<T>, io::Error>` instead of panicking if
      the OS can't create a thread. Per the book's own note in ch21.2: fine to
      skip for the chapter's purposes, but a real thread pool should handle
      this. Would mean:
  - `Worker::new` returns `Result<Worker, io::Error>` (or a custom error type)
    instead of `Worker`
  - `ThreadPool::build`'s loop propagates that with `?` instead of assuming
    every `Worker::new` call succeeds
  - `main.rs` switches from `ThreadPool::new(...)` to
    `ThreadPool::build(...).expect(...)` (or real handling)
  - Docs: https://doc.rust-lang.org/std/thread/struct.Builder.html#method.spawn

## Cleanup / dead code

- [ ] `PoolCreationError` currently only has `#[derive(Debug)]`. Decide
      whether to implement `Display` + `std::error::Error` for it (already
      drafted, commented out at the top of `lib.rs`) or drop the draft —
      only worth it once something actually calls `.unwrap()`/`?` on a
      `Result<ThreadPool, PoolCreationError>` and needs it to compose with
      other error types.
- [ ] `WorkerError` enum (`IdNumberInvalid`) is defined but never
      constructed or used anywhere — decide whether `Worker::new`/`id`
      validation actually needs it, or remove it.
- [ ] `ThreadPool.workers` field and `Worker.id`/`Worker.handle` fields are
      currently unread outside of the `id` capture in the worker loop's
      `println!` — expected for now (graceful shutdown, which joins the
      handles, hasn't been implemented yet), revisit once that's built.
- [ ] Commented-out `ThreadPool::build` / `Display`/`Error` impl blocks in
      `lib.rs` — clean up once the design settles instead of leaving as
      comments indefinitely.

## Chapter reading notes

- [ ] Book flags: "in a production thread pool implementation, you'd likely
      want to use `std::thread::Builder`" — ch21.2, right after Listing
      21-21. Confirms the item above isn't just a nice-to-have, the book
      itself calls it out as the production-grade gap.
