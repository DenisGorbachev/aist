# aist-codex

Asynchronous source reviews through the Codex CLI, with structured verdicts and a persistent content-addressed cache. A verdict is an LLM judgment, not a formal proof.

`cargo_review_snapshot` resolves one Cargo package and its transitive dependency closure under explicit feature and target options. It captures their packaged sources and compilation context. `ReviewSnapshot::capture` also accepts an explicit source map and serializable context for other languages or a narrower independently established dependency closure. Callers must include every input on which their requirement depends.

`CodexReviewer::check` reviews one requirement. `check_many` streams independent results with bounded concurrency. Each invocation reads an isolated snapshot and returns `satisfied`, `violated`, or `inconclusive`, with a rationale and source citations. `ReviewVerdict::into_result` accepts only `satisfied`.

## Cache identity

The SHA-256 key covers:

- Captured source paths and bytes.
- Resolved package metadata, dependency identities, and enabled features.
- Cargo target and feature selection.
- Rust and Cargo versions.
- Cargo configuration fingerprints and compiler-related environment variables.
- Explicit extra source inputs and context.
- The requirement, review instructions, and output schema.
- Codex executable identity, CLI version, model, reasoning effort, and effective review settings.
- A protocol version and caller-controlled cache epoch.

Changing dependency feature flags invalidates reviews. Git HEAD, timestamps, the output directory, concurrency, and invocation timeouts are not identity inputs. Unrelated workspace packages and their lockfile entries are omitted. `mise.toml`, `mise.lock`, and `.mise/` are omitted; changes to effective compiler configuration still invalidate reviews. Packaged documentation is included because it can declare semantic invariants.

Capture is deliberately conservative within a dependency closure. Editing an unrelated item in a captured package can invalidate several reviews. Narrowing this further requires a complete semantic dependency closure, including new trait implementations and macros.

Build scripts, external generators, untracked compiler inputs, and custom environment variables need explicit `extra_inputs` or `extra_context`. The Cargo helper does not claim to infer those dependencies. Generated code that is absent from the snapshot must produce an inconclusive review. Paths and resolved metadata remain visible to the reviewer; Cargo configuration contents are hashed rather than copied.

Model aliases can change without a name change. Use a pinned model where available, a finite maximum age, and advance `cache_epoch` whenever external model behavior or managed Codex instructions change. Failed invocations and inconclusive judgments are never cached. Cache entries are validated on load, atomically replaced, and protected by per-key process locks. A corrupt entry is reported rather than silently triggering another paid review.

## Harness

The caller selects the Codex executable, model, reasoning effort, cache directory, cache epoch, maximum age, timeout, and concurrency. The executable must support `exec --ignore-user-config --ignore-rules --ephemeral --output-schema --output-last-message` and named permission profiles. Authenticate the CLI before use.

The runner ignores user configuration and rule files, disables skills and subagents, and supplies a read-only snapshot with command network access disabled. It verifies snapshot contents before and after a review. Managed host policy still applies. Review instructions prohibit building or executing project code. See the official [noninteractive usage](https://developers.openai.com/codex/noninteractive) and [permissions](https://developers.openai.com/codex/permissions) documentation.
