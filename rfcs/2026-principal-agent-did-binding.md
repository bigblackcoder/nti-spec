# RFC (Draft): Verifiable Principal–Agent DID Binding Profile

**Status:** Draft for discussion; non-normative until accepted through the NTI RFC process  
**Scope:** NTI-1 Pillar 1, agent identity interoperability with ANP and ATP or other authorization systems  
**Proposer:** Larry Lewis (`bigblackcoder`)  
**Companion evidence:** `conformance/principal-binding/matrix.json`

## Summary

A DID and a valid signature establish control of a verification method, **not** a person's legal identity, agency for an organization, or authorization to perform a particular action. This draft proposes a method-neutral profile to connect an agent DID to a legally accountable human or organization using independently verifiable issuer evidence, then separately bind a bounded and revocable delegation to the exact requested action. It does not impose a universal root authority, require `did:atp`, or displace ANP transport and authentication.

## Motivation

NTI-1 §3.1 requires principal-bound identities but does not specify issuer trust bootstrap, provenance of legal-identity claims, a portable binding structure, issuer status checking, role/authority of the delegator, or negative verification controls. Implementations that accept self-asserted `principal_id` or equate an agent signature with a legal person will reach incompatible and potentially unsafe conclusions.

## Terminology

- **Agent DID:** DID whose verification method authenticates the acting runtime; identifying a key does not certify ownership.
- **Principal:** Identified human or legal organization represented in the accepted binding evidence.
- **Issuer:** Entity attesting a principal identity or principal–agent relationship; acceptance is determined by verifier-local trust policy and claim type.
- **Binding credential:** Signed, time-bounded attestation associating an agent DID with a principal identifier and relationship, with appropriate status.
- **Delegation:** Separate signed grant by a principal or authorized delegate conveying narrowly defined agent authority.
- **Relying party / enforcement point:** Resource accepting or refusing an action, independently of the agent process.

## Proposed evidence and verification profile

Proposed application-level envelope (illustrative schema, **not** a finished W3C VC JSON-LD context or approved normative format):

```json
{
  "profile": "nti-principal-binding-draft-01",
  "credential_format": "vc-or-compatible",
  "binding_credential": "<signed issuer credential>",
  "principal_ref": "<purpose-scoped principal identifier>",
  "agent_did": "did:example:agent123",
  "relationship": "operatedBy",
  "valid_from": "<RFC3339 timestamp>",
  "valid_until": "<RFC3339 timestamp>",
  "status_ref": "<authenticated status resource>",
  "delegation": "<separately signed, scoped authorization>",
  "requested_action": {
    "resource": "<audience/resource identifier>",
    "operation": "<normalized operation identifier>",
    "request_id": "<unique identifier>",
    "nonce": "<fresh challenge>",
    "timestamp": "<RFC3339 timestamp>"
  }
}
```

The `principal_ref` need not expose a legal name or national identifier. A verifier may accept a selectively disclosed claim or use a locally resolved pseudonym when its policy allows it. The *issuer's* legal-identity evidence and assurance process must be evaluated independently.

### Candidate verifier rules (for normative review)

1. The verifier MUST resolve the expected agent DID through an explicitly supported method and verify challenge-bound proof of control with the intended audience and freshness. **A DID resolution alone is insufficient.**
2. The verifier MUST validate the binding credential's signature/proof, subject and agent binding, schema/profile version, validity interval, and authenticated status.
3. The issuer MUST be independently trusted **for the particular claim and assurance requirement** under relying-party policy. The credential itself MUST NOT establish issuer trust; no untrusted self-certification.
4. The verifier MUST validate that the principal, or its properly authorized representative, actually issued or authorized a distinct delegation for this agent; mere principal binding confers zero capability.
5. Delegation MUST be checked against action, target resource/audience, purpose/limits where specified, delegation depth, issuance/expiry, cancellation/revocation, and freshness, using resource-side enforcement. No key or DID proof substitutes for the delegation.
6. For absent, invalid, expired, revoked, wrong-subject, wrong-audience, or untrusted **known** evidence: **DENY**, with no dispatch.
7. When a required **authoritative status cannot be determined**, emit `incomplete_check`, do not dispatch, and enter PAUSE according to NTI-1 §6.1; record a distinguishable reason (`tryLater`, `not_authoritative`, `internal_error`) as applicable. A PAUSE is not authorization. In a high-risk path, missing freshness must not be silently treated as good status.
8. Rotation of agent signing keys must be validated against a DID method's authorized rotation process. If an identifier changes or the binding references a key rather than the DID, require binding updates or reissuance according to policy. A change of principal requires a **new issuer-verified binding** and new delegation; historical records remain auditable.
9. Verified proofs or signed decision receipts MAY be retained for audit, but neither a signed agent request nor an authorization receipt proves that a resource actually completed an action. Actual execution evidence must originate from a trusted resource.
10. Verification MUST not depend on a single named commercial service or DID method. Verifiers MAY impose different trusted issuer sets and assurance thresholds by jurisdiction or action sensitivity.

### Normative boundary

This RFC proposes *interoperability behavior*, not a legal standard for identity proofing. The profile does not establish universal legal liability or an unqualified non-repudiation claim. Post-quantum algorithm requirements in NTI-1 are a separate cryptographic-policy question; declaring an algorithm does not authenticate the real-world principal or prevent prompt injection.

## Example interoperability sequence

1. ANP peers establish a secure connection and authenticate the expected agent DID (transport and identity proof).
2. Agent presents issuer-attested principal binding (agent DID -> principal) and separately signed delegation.
3. Resource-side verifier validates accepted issuer, principal, binding status, delegation chain, action context and live authority.
4. Resource enforcement point returns ALLOW, DENY, or `incomplete_check`/PAUSE. No consequential action occurs for DENY or PAUSE.
5. If ALLOW, resource executes within bounds; independent execution receipt distinguishes attempted/authorized/completed.

## Conformance evidence plan

Each case must have a matched positive and negative control where possible, **different expected decisions**, and a resource-dispatch counter. The companion matrix describes expected results but does not itself prove an implementation passes. Run identical fixtures through at least two independent verifier implementations. Capture signed proof verification results, issuer trust configuration, authority status source/freshness, normalized decision, dispatched side-effect count and reproducible logs. Test with both a method-neutral test DID resolver and optionally `did:atp` or ANP `did:wba`; no mandatory vendor coupling.

## Security and privacy considerations

- Compromised issuer, malicious enterprise admin, false identity proofing, ambiguous legal entity jurisdiction, DID controller takeover and stolen key are independent threats. Revocation/rotation/issuer trust policy are required.
- Do not put passport numbers, legal names, raw KYC material or other sensitive personal information in public DID documents or logs.
- Selective disclosure and pairwise IDs should be supported where the required assurance can still be checked. Issuer correlation and status-list availability require threat assessment.
- Freshness, replay, nonce binding, clock skew and time-of-check/time-of-use races need explicit tests.
- A valid signature on a prompt-injected operation does not establish that the action is authorized or safe. Policy must be enforced at the protected resource.
- Be precise about DENY vs PAUSE: known-invalid authority vs required-unknown status.

## Drawbacks

Issuer governance, legal identity proofing and status availability introduce operational and jurisdictional complexity. Additional proof verification increases latency and may expose correlatable metadata. Requiring real-world identity for every agent risks excluding privacy-preserving or low-risk autonomous workflows.

## Alternatives

1. **Self-declared DID-to-person binding:** simpler, but cannot serve as independent legal-identity evidence.
2. **Single global authoritative institution:** consistent trust anchor, but centralizes authority and may not work across jurisdictions.
3. **Organization-issued credentials only:** useful for internal agent inventories, not sufficient for claims about unrelated legal identities.
4. **Existing PKI/X.509 or OAuth claims:** acceptable evidence carriers when profiled and trusted, but do not by themselves replace narrowly bounded delegated authority.

## References and review questions

- NTI-1 §3.1, §6.1, §10
- W3C DID Core: https://www.w3.org/TR/did-core/
- W3C Verifiable Credentials Data Model 2.0: https://www.w3.org/TR/vc-data-model-2.0/
- NIST SP 800-63A-4 identity proofing: https://pages.nist.gov/800-63-4/sp800-63a.html
- RFC 9396 (OAuth Rich Authorization Requests): https://www.rfc-editor.org/rfc/rfc9396.html

**Review questions:** What issuer trust roots are accepted per claim type? Is the binding directly issued to the agent DID or a separate organization-authorized agent-registration credential? Which verifier makes the ultimate decision? What is the status freshness contract? What is the required proof profile, including algorithm negotiation? How do two independent implementations attest the same result?

## Process

This is a candidate RFC only. Open a PR, solicit feedback from Gaowei, Abisheak and independent implementers, and follow NTI §10 public comment / 30-day review procedure before any normative adoption. Do not conflate acceptance of this proposal with acceptance of ATP or NTI production integrations.
