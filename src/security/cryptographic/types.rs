use std::convert::From;

use bytes::Bytes;
use speedy::{Readable, Writable};

use crate::{rtps::Submessage, security::types::DataHolder};
/// CryptoToken: sections 7.2.4.2 and 8.5.1.1 of the Security specification (v.
/// 1.1)
#[derive(Clone)]
pub struct CryptoToken {
  pub data_holder: DataHolder,
}
impl From<DataHolder> for CryptoToken {
  fn from(value: DataHolder) -> Self {
    CryptoToken { data_holder: value }
  }
}
pub type ParticipantCryptoToken = CryptoToken;
pub type DatawriterCryptoToken = CryptoToken;
pub type DatareaderCryptoToken = CryptoToken;

/// CryptoHandles are supposed to be opaque references to key material that can
/// only be interpreted inside the plugin implementation (8.5.1.2–4).
pub struct CryptoHandle {
  pub key_material: Bytes,
}

impl From<Bytes> for CryptoHandle {
  fn from(key_material: Bytes) -> Self {
    Self { key_material }
  }
}
impl From<CryptoHandle> for Bytes {
  fn from(CryptoHandle { key_material }: CryptoHandle) -> Self {
    key_material
  }
}

/// ParticipantCryptoHandle: section 8.5.1.2 of the Security specification
/// (v. 1.1)
pub type ParticipantCryptoHandle = CryptoHandle;

/// DatawriterCryptoHandle: section 8.5.1.3 of the Security specification
/// (v. 1.1)
pub type DatawriterCryptoHandle = CryptoHandle;

/// DatareaderCryptoHandle: section 8.5.1.4 of the Security specification
/// (v. 1.1)
pub type DatareaderCryptoHandle = CryptoHandle;

/// CryptoTransformIdentifier: section 8.5.1.5 of the Security specification (v.
/// 1.1)
#[derive(Debug, PartialEq, Eq, Clone, Readable, Writable)]
pub struct CryptoTransformIdentifier {
  pub transformation_kind: CryptoTransformKind,
  pub transformation_key_id: CryptoTransformKeyId,
}
/// transformation_kind: section 8.5.1.5.1 of the Security specification (v.
/// 1.1)
pub type CryptoTransformKind = [u8; 4];
/// transformation_key_id: section 8.5.1.5.2 of the Security specification (v.
/// 1.1)
pub type CryptoTransformKeyId = [u8; 4];

/// SecureSubmessageCategory_t: section 8.5.1.6 of the Security specification
/// (v. 1.1)
///
/// Used as a return type by
///[super::cryptographic_plugin::CryptoTransform::preprocess_secure_submsg],
/// and therefore includes the crypto handles that would be returned in the
/// latter two cases.

#[allow(clippy::enum_variant_names)] // We are using variant names from the spec
pub enum SecureSubmessageCategory {
  InfoSubmessage,
  DatawriterSubmessage(DatawriterCryptoHandle, DatareaderCryptoHandle),
  DatareaderSubmessage(DatareaderCryptoHandle, DatawriterCryptoHandle),
}

/// [super::cryptographic_plugin::CryptoTransform::encode_datawriter_submessage]
/// and [super::cryptographic_plugin::CryptoTransform::encode_rtps_message] may
/// return different results for each receiver, or one result to be used for all
/// receivers.
pub enum EncodeResult<T> {
  One(T),
  Many(Vec<T>),
}

/// [super::cryptographic_plugin::CryptoTransform::encode_datawriter_submessage]
/// and [super::cryptographic_plugin::CryptoTransform::encode_datareader_submessage]
/// may return the unencoded input or an encoded message between a
/// `SecurePrefix` and `SecurePostfix`. See 7.3.6.4.4 and 8.5.1.9.2 in DDS
/// Security v1.1.

pub enum EncodedSubmessage {
  Unencoded(Submessage),
  Encoded(Submessage, Submessage, Submessage),
}
