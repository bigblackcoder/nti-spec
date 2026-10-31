//! Issue #20: candidate enforcement regressions against the pinned Rust engine.
use ed25519_dalek::{Signer, SigningKey};
use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};
use ube_foundation::*;

fn request(token: Option<CapabilityToken>) -> ActionRequest {
    let key = SigningKey::from_bytes(&[7; 32]); // Public fixture material only.
    let mut request = ActionRequest {
        id: "nti-rust-regression".into(),
        actor: "recipient".into(),
        capability: "transfer".into(),
        action: "transfer".into(),
        input: serde_json::json!({"amount": 1}),
        signature: None,
        pqc_signature: None,
        public_key: Some(key.verifying_key().to_bytes().to_vec()),
        pqc_public_key: None,
        token,
        identity_claim: None,
    };
    request.signature = Some(key.sign(&request.message_to_sign()).to_bytes().to_vec());
    request
}

#[test]
fn unsigned_capability_cannot_grant_authority() {
    let engine = TrustEngine::default();
    // A valid request signature does not substitute for an issuer-signed grant.
    assert_eq!(engine.evaluate(&request(None)).decision, Decision::Deny);
    let unsigned = CapabilityToken {
        id: "unsigned-fixture".into(),
        root_actor: "issuer".into(),
        capability: "transfer".into(),
        caveats: vec![],
        signature: vec![],
    };
    assert_eq!(engine.evaluate(&request(Some(unsigned))).decision, Decision::Deny,
        "an unsigned capability must not create authority for this recipient");
    // Positive control for the engine's explicit local grant path.
    let mut granted = TrustEngine::default();
    granted.grant("recipient", "transfer");
    assert_eq!(granted.evaluate(&request(None)).decision, Decision::Allow);
}

struct EffectModule(Arc<AtomicUsize>);
#[async_trait::async_trait]
impl Module for EffectModule {
    fn name(&self) -> &str { "effect-fixture" }
    fn capabilities(&self) -> Vec<String> { vec!["transfer".into()] }
    async fn execute(&self, _: &str, _: serde_json::Value) -> Result<serde_json::Value, String> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(serde_json::json!({"executed": true}))
    }
}

fn orchestrator(threshold: usize, effects: Arc<AtomicUsize>) -> Orchestrator {
    let mut engine = TrustEngine::default();
    engine.grant("recipient", "transfer");
    let mut orchestrator = Orchestrator::new(engine);
    orchestrator.set_consensus_threshold(threshold);
    orchestrator.register_module(Box::new(EffectModule(effects)));
    orchestrator
}

#[tokio::test]
async fn required_quorum_blocks_dispatch_when_one_module_is_available() {
    // Positive control: the same authorized operation executes at threshold one.
    let allowed_effects = Arc::new(AtomicUsize::new(0));
    let mut allowed = orchestrator(1, allowed_effects.clone());
    assert!(allowed.run_task(request(None)).await.is_ok());
    assert_eq!(allowed_effects.load(Ordering::SeqCst), 1);

    let blocked_effects = Arc::new(AtomicUsize::new(0));
    let mut blocked = orchestrator(2, blocked_effects.clone());
    let result = blocked.run_task(request(None)).await;
    assert_eq!(blocked_effects.load(Ordering::SeqCst), 0,
        "a required quorum of two must block dispatch with only one eligible module");
    assert!(result.is_err(), "quorum shortfall must return refusal: {result:?}");
}
