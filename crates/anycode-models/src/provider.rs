//! The `ModelProvider` trait (PRD §79) — every adapter implements this and nothing else
//! in the application talks to a vendor SDK or API directly.

use crate::types::{ModelDefinition, ModelRequest, ProviderError, ProviderManifest, StreamEvent};
use async_trait::async_trait;
use futures_core::Stream;
use std::pin::Pin;

pub type ModelStream = Pin<Box<dyn Stream<Item = Result<StreamEvent, ProviderError>> + Send>>;

#[async_trait]
pub trait ModelProvider: Send + Sync {
    fn manifest(&self) -> ProviderManifest;

    /// The models actually available to this account — a live API call, never a
    /// hardcoded catalog (see the provider-adapter skill's contract requirements).
    async fn models(&self) -> Result<Vec<ModelDefinition>, ProviderError>;

    async fn stream(&self, request: ModelRequest) -> Result<ModelStream, ProviderError>;
}
