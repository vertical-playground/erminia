# Diagnostics API — Planned Improvements

This document tracks identified bugs and design improvements for the diagnostics subsystem
(`crates/erminia/src/diagnostics/`).

---

## Bugs

### 1. `lexer_diag!` no-help variant uses `PARSER_PASS`
**File:** `crates/erminia/src/lexer/mod.rs:19`

The two-argument (no-help) variant of `lexer_diag!` hardcodes `PARSER_PASS` instead of `LEXER_PASS`:

```rust
// Bug: should be LEXER_PASS
($code:ident, $note:ident, $args:expr, $tokens:expr, $diag:expr, $span:expr) => {{
    if let Some(dgn) = DB::build(PARSER_PASS, Code::$code)  // ← wrong
```

Every lexer diagnostic emitted without a help message gets silently tagged as a Parser error,
which corrupts `Accumulator::get(pass)` filtering and `is_blocking` checks.

---

### 2. `sort()` and `is_blocking()` use different orderings
**File:** `crates/erminia/src/diagnostics/diagnostic.rs:110–128`

`sort()` uses a hand-written integer key:
```rust
DiagnosticLevel::Internal => -1,
DiagnosticLevel::Error    =>  0,
// ...
```

`is_blocking()` calls `self.diagnostics.sort()`, which uses the **derived `Ord`** (declaration order).
These happen to produce the same result today, but will silently diverge if a new variant is inserted
or the enum is reordered. Both should use the same mechanism.

---

### 3. `with_args(MessageKind::Help, args)` silently drops `args`
**File:** `crates/erminia/src/diagnostics/builder.rs:80–86`

```rust
MessageKind::Help => {
    self.help_str = self.help.stringify(); // args is just dropped
}
```

`Help::stringify()` takes no arguments, so any args passed under `MessageKind::Help` are silently
discarded. The `MessageKind` abstraction implies a symmetric API but delivers an asymmetric one,
making call sites misleading.

---

## API Design

### 4. `Note` variants should carry their args instead of accepting `Vec<String>` at runtime
**File:** `crates/erminia/src/diagnostics/messages.rs`

Currently, note messages accept a runtime `Vec<String>` and index into it without compile-time
safety:
```rust
Note::ExpectedIdentifier => format!("Expected an identifier, but found '{}'.", args[0])
```

Passing the wrong number of args causes a runtime index panic. `args_count()` guards against it,
but only by panicking too. The fix is to embed args directly into each variant:

```rust
pub enum Note {
    ExpectedIdentifier(String),
    ExpectedSomethingElse(String, String),
    // ...
}
```

`stringify()` then takes only `&self`. This eliminates:
- `args_count()` and `args_required()` methods
- The `MessageKind` enum
- The `with_args` builder step
- All runtime panics on mismatched arg counts

Call sites become self-documenting and compiler-enforced:
```rust
.with_note(Note::ExpectedIdentifier(found.to_string()))
```

---

### 5. `Note::None` and `Help::None` should be `Option<Note>` / `Option<Help>`
**File:** `crates/erminia/src/diagnostics/messages.rs`

Using a `None` variant inside the enum forces every `match` arm to handle it explicitly and makes
`args_required()` awkward. `Option<Note>` expresses optional presence correctly, removes the
default-variant pattern, and eliminates the need for the `#[default] None` workaround.

---

### 6. `MessageKind` enum is vestigial
**File:** `crates/erminia/src/diagnostics/messages.rs:3–6`

Once `Note` carries its args (see point 4), there is no need to route through
`with_args(MessageKind::Note, ...)`. The entire `MessageKind` + `with_args` step collapses into
a direct `.with_note(Note::ExpectedIdentifier(found))` call. `MessageKind` can be removed entirely.

---

### 7. `Accumulator.diagnostics` is fully public
**File:** `crates/erminia/src/diagnostics/diagnostic.rs:22`

```rust
pub diagnostics: Vec<Diagnostic>,
```

Any caller can push to or mutate the `Vec` directly, bypassing `add_diag`, `sort`, and
`is_blocking`. The field should be private, exposing only the intended interface (`add_diag`,
`get`, `is_blocking`, iteration).

---

### 8. `DiagnosticLevel` severity ordering is inverted relative to meaning
**File:** `crates/erminia/src/diagnostics/code.rs:21–29`

The derived `Ord` gives `Internal < Error < Warning < Note < Help`, meaning `Note` and `Help` are
considered *more severe* than `Warning`. This is semantically backwards. Any future comparison like
`diag.level >= DiagnosticLevel::Warning` would silently include `Note` and `Help` as well.
The ordering should reflect actual severity: `Error > Warning > Note ≥ Help`.

---

## Display / UX

### 9. Missing space between error code and level in the header line
**File:** `crates/erminia/src/diagnostics/diagnostic.rs:57`

```rust
writeln!(f, " [{}]{} {}", ...)?;
//              ^^^ no space between ] and level
```

Produces ` [E0001]Error  message` instead of ` [E0001] Error  message`.
Should be `" [{}] {} {}"`.

---

### 10. Pass is printed with `{:?}` (debug repr) instead of `{}`
**File:** `crates/erminia/src/diagnostics/diagnostic.rs:62`

```rust
writeln!(f, "  {} {:?}", "pass:".dimmed(), self.pass)?;
//                  ^^^
```

`CompilerPass` has `derive(Display)`, so `{}` should be used to get the clean variant name.
`{:?}` prints the debug representation (e.g. `"Parser"` with quotes in some contexts).

---

### 11. Multi-line snippets always show the same line number
**File:** `crates/erminia/src/diagnostics/diagnostic.rs:76–83`

```rust
for line in self.window.snippet.lines() {
    writeln!(f, "{} │   {}", self.window.span.start.get_line(), line)?;
    //                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ never incremented
}
```

Every line in a multi-line snippet is annotated with the span's start line number. The line
counter should increment per iteration.

---

### 12. `emmit` typo throughout the public API
**File:** `crates/erminia/src/diagnostics/builder.rs:97`, macros in `lexer/mod.rs` and `syntax/mod.rs`

The method is named `emmit` instead of `emit`. This typo appears in the public `DiagnosticBuilder`
API and in every macro that calls it, requiring a coordinated rename across all call sites.

---

## Priority Summary

| # | Priority | Category | Change |
|---|----------|----------|--------|
| 1 | High — Bug | Bug | Fix `PARSER_PASS` → `LEXER_PASS` in `lexer_diag!` no-help variant |
| 2 | High — Bug | Bug | Unify `sort()` and `is_blocking()` to use the same ordering |
| 4 | High — API | API | Embed args into `Note` variants; remove `Vec<String>`, `args_count`, `args_required`, `MessageKind`, `with_args` |
| 5 | Medium | API | Use `Option<Note>` / `Option<Help>` instead of `::None` variants |
| 7 | Medium | API | Make `Accumulator.diagnostics` private |
| 8 | Medium | API | Fix `DiagnosticLevel` severity ordering |
| 3 | Medium | Bug | Remove or fix `with_args(MessageKind::Help, ...)` dead-arg path |
| 6 | Low | API | Remove `MessageKind` (follows from point 4) |
| 9 | Low | Display | Add space between `[code]` and level in header |
| 10 | Low | Display | Use `{}` instead of `{:?}` for pass |
| 11 | Low | Display | Increment line number per line in multi-line snippet loop |
| 12 | Low | Typo | Rename `emmit` → `emit` everywhere |
