use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};
use xmtp_mls::storage::consent_record::{
  ConsentState as XmtpConsentState, ConsentType as XmtpConsentType, StoredConsentRecord,
};

use crate::{client::Client, conversation::Conversation};

#[derive(Tsify, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum ConsentState {
  Unknown = 0,
  Allowed = 1,
  Denied = 2,
}

impl From<XmtpConsentState> for ConsentState {
  fn from(state: XmtpConsentState) -> Self {
    match state {
      XmtpConsentState::Unknown => ConsentState::Unknown,
      XmtpConsentState::Allowed => ConsentState::Allowed,
      XmtpConsentState::Denied => ConsentState::Denied,
    }
  }
}

impl From<ConsentState> for XmtpConsentState {
  fn from(state: ConsentState) -> Self {
    match state {
      ConsentState::Unknown => XmtpConsentState::Unknown,
      ConsentState::Allowed => XmtpConsentState::Allowed,
      ConsentState::Denied => XmtpConsentState::Denied,
    }
  }
}

#[derive(Tsify, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum ConsentEntityType {
  GroupId = 0,
  InboxId = 1,
}

impl From<ConsentEntityType> for XmtpConsentType {
  fn from(entity_type: ConsentEntityType) -> Self {
    match entity_type {
      ConsentEntityType::GroupId => XmtpConsentType::ConversationId,
      ConsentEntityType::InboxId => XmtpConsentType::InboxId,
    }
  }
}

#[derive(Tsify, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Consent {
  #[serde(rename = "entityType")]
  pub entity_type: ConsentEntityType,
  pub state: ConsentState,
  pub entity: String,
}

impl From<Consent> for StoredConsentRecord {
  fn from(consent: Consent) -> Self {
    Self {
      entity_type: consent.entity_type.into(),
      state: consent.state.into(),
      entity: consent.entity,
    }
  }
}

impl From<StoredConsentRecord> for Consent {
  fn from(value: StoredConsentRecord) -> Self {
    Self {
      entity: value.entity,
      entity_type: match value.entity_type {
        XmtpConsentType::ConversationId => ConsentEntityType::GroupId,
        XmtpConsentType::InboxId => ConsentEntityType::InboxId,
      },
      state: value.state.into(),
    }
  }
}

#[wasm_bindgen]
impl Client {
  #[wasm_bindgen(js_name = setConsentStates)]
  pub async fn set_consent_states(&self, records: Vec<Consent>) -> Result<(), JsError> {
    let stored_records: Vec<StoredConsentRecord> =
      records.into_iter().map(StoredConsentRecord::from).collect();

    self
      .inner_client()
      .set_consent_states(&stored_records)
      .await
      .map_err(|e| JsError::new(format!("{}", e).as_str()))?;
    Ok(())
  }

  #[wasm_bindgen(js_name = getConsentState)]
  pub async fn get_consent_state(
    &self,
    entity_type: ConsentEntityType,
    entity: String,
  ) -> Result<ConsentState, JsError> {
    let result = self
      .inner_client()
      .get_consent_state(entity_type.into(), entity)
      .await
      .map_err(|e| JsError::new(format!("{}", e).as_str()))?;

    Ok(result.into())
  }
}

#[wasm_bindgen]
impl Conversation {
  #[wasm_bindgen(js_name = consentState)]
  pub fn consent_state(&self) -> Result<ConsentState, JsError> {
    let group = self.to_mls_group();
    let state = group
      .consent_state()
      .map_err(|e| JsError::new(&format!("{e}")))?;

    Ok(state.into())
  }

  #[wasm_bindgen(js_name = updateConsentState)]
  pub fn update_consent_state(&self, state: ConsentState) -> Result<(), JsError> {
    let group = self.to_mls_group();

    group
      .update_consent_state(state.into())
      .map_err(|e| JsError::new(&format!("{e}")))?;

    Ok(())
  }
}
