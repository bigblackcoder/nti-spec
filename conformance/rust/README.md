# Rust enforcement regression matrix — issue #20

Candidate contribution by Larry Lewis (`bigblackcoder`), prepared with AI assistance.
These two cases exercise the Rust authorization and dispatch paths. They do not
establish LGD runtime halting, registry freshness, or signed PAUSE receipts.

The engine is pinned to `abisheakp197/Neutral-Trust-Infrastructure` commit
`308fd1c95f2c2f975e8b6b6c5e7ccba0e4d7b203`. The CI job checks out that exact
commit, verifies HEAD, copies this integration test into its crate, and uses
the engine's unchanged Cargo.lock. Rust/Cargo 1.90.0 is the declared test toolchain.

| Case | Control and stimulus | Required result |
| --- | --- | --- |
| unsigned capability | Valid signed request; no local grant; attach a token with an empty issuer signature | Deny; a request signature cannot supply missing delegated authority |
| quorum shortfall | Authorized signed request; one eligible module; required threshold two | Refusal and zero module invocations; threshold-one positive control executes once |

Only public fixture keys and an in-memory counter are used. The counter observes
module dispatch, not a real resource or completed external transaction.

From a separate pinned engine checkout, copy `nti_regressions.rs` to
`ube-foundation/tests/nti_conformance_regressions.rs`, then run:

```sh
cargo +1.90.0 test --locked --manifest-path ube-foundation/Cargo.toml --test nti_conformance_regressions -- --nocapture
```

CI retains compiler and Cargo output even on failure. There are no expected-panic,
ignored-test, or continue-on-error exceptions. Source review suggests this baseline
will fail the two assertions; execution must establish that. A build failure is
reported as blocked execution, not an assertion failure or successful conformance.
The local preparation workspace cannot currently download a compatible Rust
toolchain, so this contribution carries no claim that the tests compiled locally.
Changing the engine pin requires explicit review and a new result; never retarget
the pinned baseline silently. A failing baseline should lead to an attributed fix
and a separate run against the corrected engine revision.
