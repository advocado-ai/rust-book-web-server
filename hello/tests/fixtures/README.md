# Raw request fixtures

Each `.txt` file is a raw HTTP request to send straight at the server, to
prove the parser handles malformed/unusual input without panicking a worker
thread. These are inputs, not routes — no corresponding HTML file needed for
most of them; the point is what status code/behavior comes back, not content.

Send one with `nc` (netcat) or `curl --raw`-ish equivalents. Easiest with `nc`:

```sh
# terminal 1
cd hello && cargo run

# terminal 2, from the hello/ directory
nc 127.0.0.1:7878 < tests/fixtures/empty_request_line.txt
```

(`\r\n` line endings matter for real HTTP — these files use them
deliberately; make sure your editor doesn't silently convert them to `\n`.)

## Fixtures and what each should prove

- `happy_path.txt` — `GET / HTTP/1.1` + normal headers. Sanity check: still
  returns 200 after the parser rewrite.
- `empty_request_line.txt` — a completely empty first line (client connects
  and sends nothing, or just `\r\n`). Old code: `.next().unwrap().unwrap()`
  panics here (`None` from `.next()`) — this is the concrete case that
  proves you removed the panic.
- `missing_http_version.txt` — `GET /` with no ` HTTP/1.1` at all. Tests
  that your parser doesn't assume there are always exactly 3
  space-separated pieces.
- `unknown_method.txt` — `POST / HTTP/1.1`. This server only serves GET;
  prove your router returns a real error status (405, not a panic or a
  silent 200).
- `path_traversal.txt` — `GET /../../etc/passwd HTTP/1.1`. Ties to roadmap
  item #3 — even before you build general static file serving, worth
  proving a `..` segment can't currently do anything (right now it'd just
  404 since there's no real file lookup yet, but keep this fixture for when
  #3 lands and it becomes a real test of the traversal guard).
- `weird_whitespace.txt` — extra spaces between method/path/version
  (`GET  /  HTTP/1.1`). Real HTTP is picky about single-space separators;
  decide (and document) whether you reject this or tolerate it.
- `no_trailing_blank_line.txt` — headers with no blank-line terminator
  before the connection is closed by the client. Tests what happens when
  `take_while(|line| !line.is_empty())`-style header reading never finds
  its stop condition.

## How to use these

Don't need to automate all of these into `cargo test` right away — that's
roadmap item #8, later. For now, manually running a few by hand while you
build the parser is enough to keep you honest about "doesn't panic," which
is the actual acceptance criterion on roadmap item #1.
