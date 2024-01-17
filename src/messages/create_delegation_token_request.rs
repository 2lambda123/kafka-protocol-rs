//! CreateDelegationTokenRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/CreateDelegationTokenRequest.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use crate::protocol::{
    buf::{ByteBuf, ByteBufMut},
    compute_unknown_tagged_fields_size, types, write_unknown_tagged_fields, Builder, Decodable,
    DecodeError, Decoder, Encodable, EncodeError, Encoder, HeaderVersion, MapDecodable,
    MapEncodable, Message, StrBytes, VersionRange,
};

/// Valid versions: 0-3
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct CreatableRenewers {
    /// The type of the Kafka principal.
    ///
    /// Supported API versions: 0-3
    pub principal_type: StrBytes,

    /// The name of the Kafka principal.
    ///
    /// Supported API versions: 0-3
    pub principal_name: StrBytes,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for CreatableRenewers {
    type Builder = CreatableRenewersBuilder;

    fn builder() -> Self::Builder {
        CreatableRenewersBuilder::default()
    }
}

impl Encodable for CreatableRenewers {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 2 {
            types::CompactString.encode(buf, &self.principal_type)?;
        } else {
            types::String.encode(buf, &self.principal_type)?;
        }
        if version >= 2 {
            types::CompactString.encode(buf, &self.principal_name)?;
        } else {
            types::String.encode(buf, &self.principal_name)?;
        }
        if version >= 2 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 2 {
            total_size += types::CompactString.compute_size(&self.principal_type)?;
        } else {
            total_size += types::String.compute_size(&self.principal_type)?;
        }
        if version >= 2 {
            total_size += types::CompactString.compute_size(&self.principal_name)?;
        } else {
            total_size += types::String.compute_size(&self.principal_name)?;
        }
        if version >= 2 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for CreatableRenewers {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let principal_type = if version >= 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let principal_name = if version >= 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 2 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            principal_type,
            principal_name,
            unknown_tagged_fields,
        })
    }
}

impl Default for CreatableRenewers {
    fn default() -> Self {
        Self {
            principal_type: Default::default(),
            principal_name: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for CreatableRenewers {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

/// Valid versions: 0-3
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct CreateDelegationTokenRequest {
    /// The principal type of the owner of the token. If it's null it defaults to the token request principal.
    ///
    /// Supported API versions: 3
    pub owner_principal_type: Option<StrBytes>,

    /// The principal name of the owner of the token. If it's null it defaults to the token request principal.
    ///
    /// Supported API versions: 3
    pub owner_principal_name: Option<StrBytes>,

    /// A list of those who are allowed to renew this token before it expires.
    ///
    /// Supported API versions: 0-3
    pub renewers: Vec<CreatableRenewers>,

    /// The maximum lifetime of the token in milliseconds, or -1 to use the server side default.
    ///
    /// Supported API versions: 0-3
    pub max_lifetime_ms: i64,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for CreateDelegationTokenRequest {
    type Builder = CreateDelegationTokenRequestBuilder;

    fn builder() -> Self::Builder {
        CreateDelegationTokenRequestBuilder::default()
    }
}

impl Encodable for CreateDelegationTokenRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 3 {
            types::CompactString.encode(buf, &self.owner_principal_type)?;
        } else {
            if !self
                .owner_principal_type
                .as_ref()
                .map(|x| x.is_empty())
                .unwrap_or_default()
            {
                return Err(EncodeError);
            }
        }
        if version >= 3 {
            types::CompactString.encode(buf, &self.owner_principal_name)?;
        } else {
            if !self
                .owner_principal_name
                .as_ref()
                .map(|x| x.is_empty())
                .unwrap_or_default()
            {
                return Err(EncodeError);
            }
        }
        if version >= 2 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.renewers)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.renewers)?;
        }
        types::Int64.encode(buf, &self.max_lifetime_ms)?;
        if version >= 2 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 3 {
            total_size += types::CompactString.compute_size(&self.owner_principal_type)?;
        } else {
            if !self
                .owner_principal_type
                .as_ref()
                .map(|x| x.is_empty())
                .unwrap_or_default()
            {
                return Err(EncodeError);
            }
        }
        if version >= 3 {
            total_size += types::CompactString.compute_size(&self.owner_principal_name)?;
        } else {
            if !self
                .owner_principal_name
                .as_ref()
                .map(|x| x.is_empty())
                .unwrap_or_default()
            {
                return Err(EncodeError);
            }
        }
        if version >= 2 {
            total_size +=
                types::CompactArray(types::Struct { version }).compute_size(&self.renewers)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.renewers)?;
        }
        total_size += types::Int64.compute_size(&self.max_lifetime_ms)?;
        if version >= 2 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for CreateDelegationTokenRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let owner_principal_type = if version >= 3 {
            types::CompactString.decode(buf)?
        } else {
            Some(Default::default())
        };
        let owner_principal_name = if version >= 3 {
            types::CompactString.decode(buf)?
        } else {
            Some(Default::default())
        };
        let renewers = if version >= 2 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let max_lifetime_ms = types::Int64.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 2 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            owner_principal_type,
            owner_principal_name,
            renewers,
            max_lifetime_ms,
            unknown_tagged_fields,
        })
    }
}

impl Default for CreateDelegationTokenRequest {
    fn default() -> Self {
        Self {
            owner_principal_type: Some(Default::default()),
            owner_principal_name: Some(Default::default()),
            renewers: Default::default(),
            max_lifetime_ms: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for CreateDelegationTokenRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

impl HeaderVersion for CreateDelegationTokenRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 2 {
            2
        } else {
            1
        }
    }
}
