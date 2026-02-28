# Suggestions for Lamina

These are suggestions for the lamina project itself, based on experience using lamina 0.0.8 as a backend for nukleus. They focus on documentation, API clarity, and features that would help embedders.

---

## 1. Clarify print vs writebyte in Documentation

**Problem:** It is unclear from docs that `print` is intended for debugging lamina itself, not for user program output. Embedders (e.g. nukleus) may assume `print` is the standard output primitive and use it for compiled programs.

**Suggestion:** Add explicit documentation stating:

- `print` is for debugging the lamina compiler/toolchain. Its behavior (e.g. newlines, format) is implementation-defined and may change.
- `writebyte` is the canonical primitive for user program output. All portable output should be built on `writebyte`.

**Where:** README, docs.rs, or a "Output semantics" section in the spec.

---

## 2. Document writebyte Semantics

**Problem:** Embedders need to know exactly what `writebyte` does: which fd (stdout), whether it blocks, and whether it accepts literal immediates vs SSA values only.

**Suggestion:** Document:

- `writebyte <i64>` writes the low 8 bits to stdout (fd 1).
- Whether `writebyte %var` is supported (SSA value).
- Any platform differences (e.g. Windows vs Unix).

---

## 3. Provide a Standard Integer-to-Decimal Helper (Optional)

**Problem:** To print integers via `writebyte`, embedders must emit a recursive helper (e.g. `@print_number` / `@print_digits`) in every module that prints integers. This is boilerplate that every embedder will need.

**Suggestion:** Consider one of:

- **Option A:** Ship a prelude/stdlib IR snippet (e.g. `@lamina_print_i64`) that embedders can prepend. Document it in the repo.
- **Option B:** Add a builtin `print_i64(i64)` that compiles to the same digit-by-digit writebyte logic, but is part of lamina's runtime/prelude.
- **Option C:** Keep the current minimal design; document the pattern (zero/negative/digits recursion) in a "Common patterns" or "Output" section so embedders can copy it.

Option C is lowest-friction for lamina; Option A is helpful without changing the IR.

---

## 4. IR Parser: print and Literals

**Problem:** From plan.md: "Text IR parser expects `print <identifier>`; `print \"literal\"` fails (Invalid identifier)." This makes it harder to test or hand-write IR with string output.

**Suggestion:** Either:

- Document that `print` accepts only SSA identifiers (no string literals), and recommend `writebyte` for strings; or
- Extend the parser to accept `print "literal"` and lower it to a writebyte sequence internally.

The former is simpler and aligns with "writebyte for output."

---

## 5. IRBuilder API Consistency

**Problem:** IRBuilder has `.print(string("..."))` and `.print(var("x"))`, but MIR codegen may not support string print. This creates a mismatch between API surface and actual behavior.

**Suggestion:** Either:

- Document that `.print(string(...))` is unsupported or deprecated, and that output should use writebyte; or
- Implement string print in MIR codegen as a writebyte loop.

---

## 6. Naming: Consider Renaming or Deprecating print

**Problem:** The name `print` suggests a general-purpose output primitive. If it is debug-only, the name is misleading.

**Suggestion:** Consider:

- Renaming to `debug_print` or `dbg_print` to signal its purpose; or
- Adding a deprecation note in docs: "Prefer writebyte for user output."

---

## 7. Testcase as Reference Implementation

**Problem:** Lamina's own test (e.g. `simple_const.lamina`) uses `@print_number` + `@print_digits` + `writebyte` for integer output. This is the correct pattern, but it is not documented as the reference.

**Suggestion:** Add a "Reference: integer output" section that points to this test and explains the pattern (zero, negative, recursive digits). This gives embedders a canonical example to follow.

---

## 8. Module / Prelude Story

**Problem:** If lamina gains a prelude or stdlib, the integer-print helper could live there. Currently there is no standard way to "include" shared IR.

**Suggestion:** If/when a prelude mechanism exists, document how embedders can opt in (e.g. `--prelude output` or similar). Until then, documenting the helper pattern suffices.

---

## Summary Table

| Suggestion | Effort | Impact |
|------------|--------|--------|
| Clarify print vs writebyte in docs | Low | High |
| Document writebyte semantics | Low | High |
| Document integer-print pattern | Low | Medium |
| Standard helper (prelude/snippet) | Medium | Medium |
| IR parser: print literal handling | Low | Low |
| IRBuilder consistency | Medium | Low |
| Rename/deprecate print | Medium | Medium |
| Prelude/module mechanism | High | Medium |

---

## References

- nukleus uses lamina 0.0.8 as AOT backend.
- nukleus implements print/println via writebyte for strings and a custom `@nk_print_i64` helper for integers, avoiding lamina `print` entirely.
