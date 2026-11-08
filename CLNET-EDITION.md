# EGCL, common-lisp.net edition

This branch is Evergreen Common Lisp's `main` with a few correctness
fixes merged in, built and published by common-lisp.net while they wait
for upstream.  It is not an official EGCL release: report problems with
these changes here, and everything else to
[atgreen/evergreen](https://github.com/atgreen/evergreen).

`(lisp-implementation-version)` reports `0.0.12-clnet.N`.  The base is
`main` at 39800079 (2026-11-04), the commit prepared as v0.0.12 but not
yet tagged.  Each change is a branch off that commit, merged here
unchanged; when upstream fixes or refuses one, it is dropped from the
next edition.

## Changes

Each fixes an issue filed upstream; the fixes themselves have not been
submitted yet.

| Change | Upstream |
|---|---|
| `CLOSE :ABORT T` deletes a file the stream created and restores the file it replaced (it left an empty file, and the old contents were already gone at `OPEN`); `:IF-EXISTS :RENAME` keeps the old file as `NAME.bak`; the interpreter's `WITH-OPEN-FILE` aborts on a non-local exit | issue #1003 |
| `FORMAT`'s `~&` at the start of a control string starts a fresh line relative to the stream's column; broadcast, two-way and echo streams report their output's column | issue #941 |
| A `#` rebuilt with `MAKE-DISPATCH-MACRO-CHARACTER` reads the standard sub-characters restored into it, with the current readtable (named-readtables' `(:merge :standard)` readtables read `#:foo` again); `#` given constituent syntax is no longer a dispatching macro character | issue #940 |
| `DEFMETHOD` can specialize on `RESTART` before anything has called `FIND-CLASS` | issue #917 |

Each change has its own test in `crates/egcl/tests/`
(`open_if_exists_cli.rs`, `format_fresh_line_cli.rs`,
`reader_sharp_redispatch_cli.rs`, `restart_specializer_cli.rs`).

## Building

As upstream: `cargo build --release -p egcl`.  Images built from this
branch are to be published as `clfoundation/egcl:0.0.12-clnet-*`
beside the upstream releases; see
[cl-docker-images/egcl](https://gitlab.common-lisp.net/cl-docker-images/egcl).
