# aist-codex

Asynchronous source reviews through the Codex CLI, with structured verdicts and a persistent content-addressed cache. A verdict is an LLM judgment, not a formal proof.

`cargo_review_snapshot` resolves one Cargo package and its transitive dependency closure under explicit feature and target options. It captures their packaged sources and compilation context. `ReviewSnapshot::capture` also accepts an explicit source map and serializable context for other languages or a narrower independently established dependency closure. Callers must include every input on which their requirement depends.

`CodexReviewer::check` reviews one requirement. Each invocation reads a captured snapshot and returns `satisfied`, `violated`, or `inconclusive`, with a rationale and source citations. `ReviewVerdict::into_result` accepts only `satisfied`.

## Cache identity

The SHA-256 key covers:

- Captured source paths and bytes.
- Resolved package metadata, dependency identities, and enabled features.
- Cargo target and feature selection.
- Rust and Cargo versions.
- Cargo configuration fingerprints and compiler-related environment variables.
- Explicit extra source inputs and context.
- The requirement, review instructions, and output schema.
- Codex executable identity, CLI version, model, reasoning effort, review settings, and policy directory.
- A protocol version and caller-controlled cache epoch.

Changing dependency feature flags invalidates reviews. Git HEAD, timestamps, the output directory, concurrency, and invocation timeouts are not identity inputs. Unrelated workspace packages and their lockfile entries are omitted. `mise.toml`, `mise.lock`, and `.mise/` are omitted; changes to effective compiler configuration still invalidate reviews. Packaged documentation is included because it can declare semantic invariants.

Capture is deliberately conservative within a dependency closure. Editing an unrelated item in a captured package can invalidate several reviews. Narrowing this further requires a complete semantic dependency closure, including new trait implementations and macros.

Build scripts, external generators, untracked compiler inputs, and custom environment variables need explicit `extra_inputs` or `extra_context`. The Cargo helper does not claim to infer those dependencies. Generated code that is absent from the snapshot must produce an inconclusive review. Paths and resolved metadata remain visible to the reviewer; Cargo configuration contents are hashed rather than copied.

Model aliases can change without a name change. Use a pinned model where available, a finite maximum age, and advance `cache_epoch` whenever external model behavior or managed Codex instructions change. Failed invocations and inconclusive judgments are never cached. Cache entries are validated on load, atomically replaced, and protected by per-key process locks. A corrupt entry is reported rather than silently triggering another paid review.

## Harness

The caller selects the Codex executable, policy directory, model, reasoning effort, cache directory, cache epoch, maximum age, and timeout. The executable must support `exec --ephemeral --output-schema --output-last-message`. Authenticate the CLI before use.

The runner inherits user configuration, project instructions, permissions, approval rules, and shell environment from the explicit policy directory. It does not override host guards. It verifies snapshot contents before and after a review. Advance `cache_epoch` when user configuration or project instructions change; those inputs are not inferred from Codex configuration. Review instructions prohibit building or executing project code. See the official [noninteractive usage](https://developers.openai.com/codex/noninteractive) and [permissions](https://developers.openai.com/codex/permissions) documentation.
