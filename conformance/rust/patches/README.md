# Candidate core fix for issue #20

Apply `issuer-and-quorum.patch` to Neutral-Trust-Infrastructure commit
`308fd1c95f2c2f975e8b6b6c5e7ccba0e4d7b203` with `git apply --check` then `git apply`.
The two original regression assertions are preserved unchanged in the patch.

Capability tokens now require a trusted local issuer key, a valid strict Ed25519
signature, and a local grant authorizing that issuer for the token capability.
Configure `register_issuer_key(issuer, public_key_bytes)` and `grant(issuer, capability)`
through the administrative API. A request's own public key cannot establish issuer
trust. Existing persisted states default to no trusted issuers until configured.
Unsigned tokens are intentionally no longer accepted. Local explicit grants remain
available. The Python wrapper exposes the same issuer-key registration method.

Quorum shortfalls now return an error before module dispatch. Module errors and
result disagreement return errors without falling back to another execution.
This fixes local module threshold enforcement; it does not establish Byzantine
validator independence or implement consensus-before-effect. Selected modules still
execute before their results can be compared. No runtime halt or rollback claim
is made by this patch.

CI keeps the pinned failing baseline and separately applies and tests this patch,
including existing core tests, positive controls, issuer authentication, tampering,
persistence, eligible-module counts, and error/disagreement paths. Build evidence
is retained by GitHub Actions. The patch is a candidate until CI has completed.
