//! CreateAclsResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/CreateAclsResponse.json).
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
pub struct AclCreationResult {
    /// The result error, or zero if there was no error.
    ///
    /// Supported API versions: 0-3
    pub error_code: i16,

    /// The result message, or null if there was no error.
    ///
    /// Supported API versions: 0-3
    pub error_message: Option<StrBytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for AclCreationResult {
    type Builder = AclCreationResultBuilder;

    fn builder() -> Self::Builder {
        AclCreationResultBuilder::default()
    }
}

impl Encodable for AclCreationResult {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        if version >= 2 {
            types::CompactString.encode(buf, &self.error_message)?;
        } else {
            types::String.encode(buf, &self.error_message)?;
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
        total_size += types::Int16.compute_size(&self.error_code)?;
        if version >= 2 {
            total_size += types::CompactString.compute_size(&self.error_message)?;
        } else {
            total_size += types::String.compute_size(&self.error_message)?;
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

impl Decodable for AclCreationResult {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = types::Int16.decode(buf)?;
        let error_message = if version >= 2 {
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
            error_code,
            error_message,
            unknown_tagged_fields,
        })
    }
}

impl Default for AclCreationResult {
    fn default() -> Self {
        Self {
            error_code: 0,
            error_message: Some(Default::default()),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AclCreationResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

/// Valid versions: 0-3
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct CreateAclsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    ///
    /// Supported API versions: 0-3
    pub throttle_time_ms: i32,

    /// The results for each ACL creation.
    ///
    /// Supported API versions: 0-3
    pub results: Vec<AclCreationResult>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for CreateAclsResponse {
    type Builder = CreateAclsResponseBuilder;

    fn builder() -> Self::Builder {
        CreateAclsResponseBuilder::default()
    }
}

impl Encodable for CreateAclsResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.throttle_time_ms)?;
        if version >= 2 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.results)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.results)?;
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
        total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        if version >= 2 {
            total_size +=
                types::CompactArray(types::Struct { version }).compute_size(&self.results)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.results)?;
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

impl Decodable for CreateAclsResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = types::Int32.decode(buf)?;
        let results = if version >= 2 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
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
            throttle_time_ms,
            results,
            unknown_tagged_fields,
        })
    }
}

impl Default for CreateAclsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            results: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for CreateAclsResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

impl HeaderVersion for CreateAclsResponse {
    fn header_version(version: i16) -> i16 {
        if version >= 2 {
            1
        } else {
            0
        }
    }
}
