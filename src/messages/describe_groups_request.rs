//! DescribeGroupsRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/DescribeGroupsRequest.json).
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


/// Valid versions: 0-5
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct DescribeGroupsRequest {
    /// The names of the groups to describe
    /// 
    /// Supported API versions: 0-5
    pub groups: Vec<super::GroupId>,

    /// Whether to include authorized operations.
    /// 
    /// Supported API versions: 3-5
    pub include_authorized_operations: bool,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for DescribeGroupsRequest {
    type Builder = DescribeGroupsRequestBuilder;

    fn builder() -> Self::Builder{
        DescribeGroupsRequestBuilder::default()
    }
}

impl Encodable for DescribeGroupsRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 5 {
            types::CompactArray(types::CompactString).encode(buf, &self.groups)?;
        } else {
            types::Array(types::String).encode(buf, &self.groups)?;
        }
        if version >= 3 {
            types::Boolean.encode(buf, &self.include_authorized_operations)?;
        } else {
            if self.include_authorized_operations {
                return Err(EncodeError)
            }
        }
        if version >= 5 {
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
        if version >= 5 {
            total_size += types::CompactArray(types::CompactString).compute_size(&self.groups)?;
        } else {
            total_size += types::Array(types::String).compute_size(&self.groups)?;
        }
        if version >= 3 {
            total_size += types::Boolean.compute_size(&self.include_authorized_operations)?;
        } else {
            if self.include_authorized_operations {
                return Err(EncodeError)
            }
        }
        if version >= 5 {
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

impl Decodable for DescribeGroupsRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let groups = if version >= 5 {
            types::CompactArray(types::CompactString).decode(buf)?
        } else {
            types::Array(types::String).decode(buf)?
        };
        let include_authorized_operations = if version >= 3 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 5 {
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
            groups,
            include_authorized_operations,
            unknown_tagged_fields,
        })
    }
}

impl Default for DescribeGroupsRequest {
    fn default() -> Self {
        Self {
            groups: Default::default(),
            include_authorized_operations: false,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribeGroupsRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

impl HeaderVersion for DescribeGroupsRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 5 {
            2
        } else {
            1
        }
    }
}

