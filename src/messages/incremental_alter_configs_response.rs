//! IncrementalAlterConfigsResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/IncrementalAlterConfigsResponse.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use crate::protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}, Builder
};


/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct AlterConfigsResourceResponse {
    /// The resource error code.
    /// 
    /// Supported API versions: 0-1
    pub error_code: i16,

    /// The resource error message, or null if there was no error.
    /// 
    /// Supported API versions: 0-1
    pub error_message: Option<StrBytes>,

    /// The resource type.
    /// 
    /// Supported API versions: 0-1
    pub resource_type: i8,

    /// The resource name.
    /// 
    /// Supported API versions: 0-1
    pub resource_name: StrBytes,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for AlterConfigsResourceResponse {
    type Builder = AlterConfigsResourceResponseBuilder;

    fn builder() -> Self::Builder{
        AlterConfigsResourceResponseBuilder::default()
    }
}

impl Encodable for AlterConfigsResourceResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        if version >= 1 {
            types::CompactString.encode(buf, &self.error_message)?;
        } else {
            types::String.encode(buf, &self.error_message)?;
        }
        types::Int8.encode(buf, &self.resource_type)?;
        if version >= 1 {
            types::CompactString.encode(buf, &self.resource_name)?;
        } else {
            types::String.encode(buf, &self.resource_name)?;
        }
        if version >= 1 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
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
        if version >= 1 {
            total_size += types::CompactString.compute_size(&self.error_message)?;
        } else {
            total_size += types::String.compute_size(&self.error_message)?;
        }
        total_size += types::Int8.compute_size(&self.resource_type)?;
        if version >= 1 {
            total_size += types::CompactString.compute_size(&self.resource_name)?;
        } else {
            total_size += types::String.compute_size(&self.resource_name)?;
        }
        if version >= 1 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for AlterConfigsResourceResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = types::Int16.decode(buf)?;
        let error_message = if version >= 1 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let resource_type = types::Int8.decode(buf)?;
        let resource_name = if version >= 1 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 1 {
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
            resource_type,
            resource_name,
            unknown_tagged_fields,
        })
    }
}

impl Default for AlterConfigsResourceResponse {
    fn default() -> Self {
        Self {
            error_code: 0,
            error_message: Some(Default::default()),
            resource_type: 0,
            resource_name: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AlterConfigsResourceResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct IncrementalAlterConfigsResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 0-1
    pub throttle_time_ms: i32,

    /// The responses for each resource.
    /// 
    /// Supported API versions: 0-1
    pub responses: Vec<AlterConfigsResourceResponse>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for IncrementalAlterConfigsResponse {
    type Builder = IncrementalAlterConfigsResponseBuilder;

    fn builder() -> Self::Builder{
        IncrementalAlterConfigsResponseBuilder::default()
    }
}

impl Encodable for IncrementalAlterConfigsResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.throttle_time_ms)?;
        if version >= 1 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.responses)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.responses)?;
        }
        if version >= 1 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
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
        if version >= 1 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.responses)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.responses)?;
        }
        if version >= 1 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for IncrementalAlterConfigsResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = types::Int32.decode(buf)?;
        let responses = if version >= 1 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 1 {
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
            responses,
            unknown_tagged_fields,
        })
    }
}

impl Default for IncrementalAlterConfigsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            responses: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for IncrementalAlterConfigsResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

impl HeaderVersion for IncrementalAlterConfigsResponse {
    fn header_version(version: i16) -> i16 {
        if version >= 1 {
            1
        } else {
            0
        }
    }
}

