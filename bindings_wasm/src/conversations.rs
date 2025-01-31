use std::collections::HashMap;
use std::sync::Arc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsError, JsValue};
use xmtp_mls::groups::{GroupMetadataOptions, HmacKey as XmtpHmacKey, PreconfiguredPolicies};
use xmtp_mls::storage::group::ConversationType as XmtpConversationType;
use xmtp_mls::storage::group::GroupMembershipState as XmtpGroupMembershipState;
use xmtp_mls::storage::group::GroupQueryArgs;

use crate::conversation::MessageDisappearingSettings;
use crate::messages::Message;
use crate::permissions::{GroupPermissionsOptions, PermissionPolicySet};
use crate::streams::{StreamCallback, StreamCloser};
use crate::{client::RustXmtpClient, conversation::Conversation};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum ConversationType {
  Dm = 0,
  Group = 1,
  Sync = 2,
}

impl From<XmtpConversationType> for ConversationType {
  fn from(ct: XmtpConversationType) -> Self {
    match ct {
      XmtpConversationType::Dm => ConversationType::Dm,
      XmtpConversationType::Group => ConversationType::Group,
      XmtpConversationType::Sync => ConversationType::Sync,
    }
  }
}

impl From<ConversationType> for XmtpConversationType {
  fn from(nct: ConversationType) -> Self {
    match nct {
      ConversationType::Dm => XmtpConversationType::Dm,
      ConversationType::Group => XmtpConversationType::Group,
      ConversationType::Sync => XmtpConversationType::Sync,
    }
  }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum GroupMembershipState {
  Allowed = 0,
  Rejected = 1,
  Pending = 2,
}

impl From<XmtpGroupMembershipState> for GroupMembershipState {
  fn from(gms: XmtpGroupMembershipState) -> Self {
    match gms {
      XmtpGroupMembershipState::Allowed => GroupMembershipState::Allowed,
      XmtpGroupMembershipState::Rejected => GroupMembershipState::Rejected,
      XmtpGroupMembershipState::Pending => GroupMembershipState::Pending,
    }
  }
}

impl From<GroupMembershipState> for XmtpGroupMembershipState {
  fn from(ngms: GroupMembershipState) -> Self {
    match ngms {
      GroupMembershipState::Allowed => XmtpGroupMembershipState::Allowed,
      GroupMembershipState::Rejected => XmtpGroupMembershipState::Rejected,
      GroupMembershipState::Pending => XmtpGroupMembershipState::Pending,
    }
  }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Default)]
pub struct ListConversationsOptions {
  #[wasm_bindgen(js_name = allowedStates)]
  pub allowed_states: Option<Vec<GroupMembershipState>>,
  #[wasm_bindgen(js_name = conversationType)]
  pub conversation_type: Option<ConversationType>,
  #[wasm_bindgen(js_name = createdAfterNs)]
  pub created_after_ns: Option<i64>,
  #[wasm_bindgen(js_name = createdBeforeNs)]
  pub created_before_ns: Option<i64>,
  pub limit: Option<i64>,
}

impl From<ListConversationsOptions> for GroupQueryArgs {
  fn from(opts: ListConversationsOptions) -> GroupQueryArgs {
    GroupQueryArgs::default()
      .maybe_allowed_states(
        opts
          .allowed_states
          .map(|states| states.into_iter().map(From::from).collect()),
      )
      .maybe_conversation_type(opts.conversation_type.map(Into::into))
      .maybe_created_after_ns(opts.created_after_ns)
      .maybe_created_before_ns(opts.created_before_ns)
      .maybe_limit(opts.limit)
  }
}

#[wasm_bindgen]
impl ListConversationsOptions {
  #[wasm_bindgen(constructor)]
  pub fn new(
    allowed_states: Option<Vec<GroupMembershipState>>,
    conversation_type: Option<ConversationType>,
    created_after_ns: Option<i64>,
    created_before_ns: Option<i64>,
    limit: Option<i64>,
  ) -> Self {
    Self {
      allowed_states,
      conversation_type,
      created_after_ns,
      created_before_ns,
      limit,
    }
  }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct CreateGroupOptions {
  pub permissions: Option<GroupPermissionsOptions>,
  #[wasm_bindgen(js_name = groupName)]
  pub group_name: Option<String>,
  #[wasm_bindgen(js_name = groupImageUrlSquare)]
  pub group_image_url_square: Option<String>,
  #[wasm_bindgen(js_name = groupDescription)]
  pub group_description: Option<String>,
  #[wasm_bindgen(js_name = groupPinnedFrameUrl)]
  pub group_pinned_frame_url: Option<String>,
  #[wasm_bindgen(js_name = customPermissionPolicySet)]
  pub custom_permission_policy_set: Option<PermissionPolicySet>,
  #[wasm_bindgen(js_name = messageDisappearingSettings)]
  pub message_disappearing_settings: Option<MessageDisappearingSettings>,
}

#[wasm_bindgen]
impl CreateGroupOptions {
  #[wasm_bindgen(constructor)]
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    permissions: Option<GroupPermissionsOptions>,
    group_name: Option<String>,
    group_image_url_square: Option<String>,
    group_description: Option<String>,
    group_pinned_frame_url: Option<String>,
    custom_permission_policy_set: Option<PermissionPolicySet>,
    message_disappearing_settings: Option<MessageDisappearingSettings>,
  ) -> Self {
    Self {
      permissions,
      group_name,
      group_image_url_square,
      group_description,
      group_pinned_frame_url,
      custom_permission_policy_set,
      message_disappearing_settings,
    }
  }
}

impl CreateGroupOptions {
  pub fn into_group_metadata_options(self) -> GroupMetadataOptions {
    GroupMetadataOptions {
      name: self.group_name,
      image_url_square: self.group_image_url_square,
      description: self.group_description,
      pinned_frame_url: self.group_pinned_frame_url,
      message_disappearing_settings: self
        .message_disappearing_settings
        .map(|settings| settings.into()),
    }
  }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(serde::Serialize)]
pub struct HmacKey {
  pub key: Vec<u8>,
  pub epoch: i64,
}

impl From<XmtpHmacKey> for HmacKey {
  fn from(value: XmtpHmacKey) -> Self {
    Self {
      epoch: value.epoch,
      key: value.key.to_vec(),
    }
  }
}

#[wasm_bindgen]
pub struct Conversations {
  inner_client: Arc<RustXmtpClient>,
}

impl Conversations {
  pub fn new(inner_client: Arc<RustXmtpClient>) -> Self {
    Self { inner_client }
  }
}

#[wasm_bindgen]
impl Conversations {
  #[wasm_bindgen(js_name = createGroup)]
  pub async fn create_group(
    &self,
    account_addresses: Vec<String>,
    options: Option<CreateGroupOptions>,
  ) -> Result<Conversation, JsError> {
    let options = options.unwrap_or(CreateGroupOptions {
      permissions: None,
      group_name: None,
      group_image_url_square: None,
      group_description: None,
      group_pinned_frame_url: None,
      custom_permission_policy_set: None,
      message_disappearing_settings: None,
    });

    if let Some(GroupPermissionsOptions::CustomPolicy) = options.permissions {
      if options.custom_permission_policy_set.is_none() {
        return Err(JsError::new("CustomPolicy must include policy set"));
      }
    } else if options.custom_permission_policy_set.is_some() {
      return Err(JsError::new("Only CustomPolicy may specify a policy set"));
    }

    let metadata_options = options.clone().into_group_metadata_options();

    let group_permissions = match options.permissions {
      Some(GroupPermissionsOptions::Default) => {
        Some(PreconfiguredPolicies::Default.to_policy_set())
      }
      Some(GroupPermissionsOptions::AdminOnly) => {
        Some(PreconfiguredPolicies::AdminsOnly.to_policy_set())
      }
      Some(GroupPermissionsOptions::CustomPolicy) => {
        if let Some(policy_set) = options.custom_permission_policy_set {
          Some(
            policy_set
              .try_into()
              .map_err(|e| JsError::new(format!("{}", e).as_str()))?,
          )
        } else {
          None
        }
      }
      _ => None,
    };

    let convo = if account_addresses.is_empty() {
      let group = self
        .inner_client
        .create_group(group_permissions, metadata_options)
        .map_err(|e| JsError::new(format!("{}", e).as_str()))?;
      group
        .sync()
        .await
        .map_err(|e| JsError::new(format!("{}", e).as_str()))?;
      group
    } else {
      self
        .inner_client
        .create_group_with_members(&account_addresses, group_permissions, metadata_options)
        .await
        .map_err(|e| JsError::new(format!("{}", e).as_str()))?
    };

    Ok(convo.into())
  }

  #[wasm_bindgen(js_name = createDm)]
  pub async fn find_or_create_dm(&self, account_address: String) -> Result<Conversation, JsError> {
    let convo = self
      .inner_client
      .find_or_create_dm(account_address)
      .await
      .map_err(|e| JsError::new(format!("{}", e).as_str()))?;

    Ok(convo.into())
  }

  #[wasm_bindgen(js_name = findGroupById)]
  pub fn find_group_by_id(&self, group_id: String) -> Result<Conversation, JsError> {
    let group_id = hex::decode(group_id).map_err(|e| JsError::new(format!("{}", e).as_str()))?;

    let group = self
      .inner_client
      .group(group_id)
      .map_err(|e| JsError::new(format!("{}", e).as_str()))?;

    Ok(group.into())
  }

  #[wasm_bindgen(js_name = findDmByTargetInboxId)]
  pub fn find_dm_by_target_inbox_id(
    &self,
    target_inbox_id: String,
  ) -> Result<Conversation, JsError> {
    let convo = self
      .inner_client
      .dm_group_from_target_inbox(target_inbox_id)
      .map_err(|e| JsError::new(format!("{}", e).as_str()))?;

    Ok(convo.into())
  }

  #[wasm_bindgen(js_name = findMessageById)]
  pub fn find_message_by_id(&self, message_id: String) -> Result<Message, JsError> {
    let message_id =
      hex::decode(message_id).map_err(|e| JsError::new(format!("{}", e).as_str()))?;

    let message = self
      .inner_client
      .message(message_id)
      .map_err(|e| JsError::new(format!("{}", e).as_str()))?;

    Ok(message.into())
  }

  #[wasm_bindgen]
  pub async fn sync(&self) -> Result<(), JsError> {
    let provider = self
      .inner_client
      .mls_provider()
      .map_err(|e| JsError::new(format!("{}", e).as_str()))?;
    self
      .inner_client
      .sync_welcomes(&provider)
      .await
      .map_err(|e| JsError::new(format!("{}", e).as_str()))?;

    Ok(())
  }

  #[wasm_bindgen(js_name = syncAllConversations)]
  pub async fn sync_all_conversations(&self) -> Result<usize, JsError> {
    let provider = self
      .inner_client
      .mls_provider()
      .map_err(|e| JsError::new(format!("{}", e).as_str()))?;

    let num_groups_synced = self
      .inner_client
      .sync_all_welcomes_and_groups(&provider, None)
      .await
      .map_err(|e| JsError::new(format!("{}", e).as_str()))?;

    Ok(num_groups_synced)
  }

  #[wasm_bindgen]
  pub fn list(&self, opts: Option<ListConversationsOptions>) -> Result<js_sys::Array, JsError> {
    let convo_list: js_sys::Array = self
      .inner_client
      .find_groups(opts.unwrap_or_default().into())
      .map_err(|e| JsError::new(format!("{}", e).as_str()))?
      .into_iter()
      .map(|group| {
        JsValue::from(Conversation::new(
          self.inner_client.clone(),
          group.group_id,
          group.created_at_ns,
        ))
      })
      .collect();

    Ok(convo_list)
  }

  #[wasm_bindgen(js_name = listGroups)]
  pub fn list_groups(
    &self,
    opts: Option<ListConversationsOptions>,
  ) -> Result<js_sys::Array, JsError> {
    self.list(Some(ListConversationsOptions {
      conversation_type: Some(ConversationType::Group),
      ..opts.unwrap_or_default()
    }))
  }

  #[wasm_bindgen(js_name = listDms)]
  pub fn list_dms(&self, opts: Option<ListConversationsOptions>) -> Result<js_sys::Array, JsError> {
    self.list(Some(ListConversationsOptions {
      conversation_type: Some(ConversationType::Dm),
      ..opts.unwrap_or_default()
    }))
  }

  #[wasm_bindgen(js_name = getHmacKeys)]
  pub fn get_hmac_keys(&self) -> Result<JsValue, JsError> {
    let inner = self.inner_client.as_ref();
    let conversations = inner
      .find_groups(GroupQueryArgs {
        include_duplicate_dms: true,
        ..Default::default()
      })
      .map_err(|e| JsError::new(format!("{}", e).as_str()))?;

    let mut hmac_map: HashMap<String, Vec<HmacKey>> = HashMap::new();
    for conversation in conversations {
      let id = hex::encode(&conversation.group_id);
      let keys = conversation
        .hmac_keys(-1..=1)
        .map_err(|e| JsError::new(format!("{}", e).as_str()))?
        .into_iter()
        .map(Into::into)
        .collect::<Vec<_>>();
      hmac_map.insert(id, keys);
    }

    Ok(crate::to_value(&hmac_map)?)
  }

  #[wasm_bindgen(js_name = stream)]
  pub fn stream(
    &self,
    callback: StreamCallback,
    conversation_type: Option<ConversationType>,
  ) -> Result<StreamCloser, JsError> {
    let stream_closer = RustXmtpClient::stream_conversations_with_callback(
      self.inner_client.clone(),
      conversation_type.map(Into::into),
      move |message| match message {
        Ok(item) => {
          let conversation = Conversation::from(item);
          callback.on_item(JsValue::from(conversation))
        }
        Err(e) => callback.on_error(JsError::from(e)),
      },
    );

    Ok(StreamCloser::new(stream_closer))
  }

  #[wasm_bindgen(js_name = "streamGroups")]
  pub fn stream_groups(&self, callback: StreamCallback) -> Result<StreamCloser, JsError> {
    self.stream(callback, Some(ConversationType::Group))
  }

  #[wasm_bindgen(js_name = "streamDms")]
  pub fn stream_dms(&self, callback: StreamCallback) -> Result<StreamCloser, JsError> {
    self.stream(callback, Some(ConversationType::Dm))
  }

  #[wasm_bindgen(js_name = "streamAllMessages")]
  pub fn stream_all_messages(
    &self,
    callback: StreamCallback,
    conversation_type: Option<ConversationType>,
  ) -> Result<StreamCloser, JsError> {
    let stream_closer = RustXmtpClient::stream_all_messages_with_callback(
      self.inner_client.clone(),
      conversation_type.map(Into::into),
      move |message| match message {
        Ok(m) => {
          let serialized = crate::to_value(&m);
          if let Err(e) = serialized {
            callback.on_error(JsError::from(e));
          } else {
            callback.on_item(serialized.expect("checked for err"))
          }
        }
        Err(e) => callback.on_error(JsError::from(e)),
      },
    );
    Ok(StreamCloser::new(stream_closer))
  }
}
