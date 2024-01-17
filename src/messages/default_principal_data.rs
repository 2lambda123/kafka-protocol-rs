//! DefaultPrincipalData
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/DefaultPrincipalData.json).
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


/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct DefaultPrincipalData {
    /// The principal type
    /// 
    /// Supported API versions: 0
    pub _type: StrBytes,

    /// The principal name
    /// 
    /// Supported API versions: 0
    pub name: StrBytes,

    /// Whether the principal was authenticated by a delegation token on the forwarding broker.
    /// 
    /// Supported API versions: 0
    pub token_authenticated: bool,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for DefaultPrincipalData {
    type Builder = DefaultPrincipalDataBuilder;

    fn builder() -> Self::Builder{
        DefaultPrincipalDataBuilder::default()
    }
}

impl Encodable for DefaultPrincipalData {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::CompactString.encode(buf, &self._type)?;
        types::CompactString.encode(buf, &self.name)?;
        types::Boolean.encode(buf, &self.token_authenticated)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::CompactString.compute_size(&self._type)?;
        total_size += types::CompactString.compute_size(&self.name)?;
        total_size += types::Boolean.compute_size(&self.token_authenticated)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for DefaultPrincipalData {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let _type = types::CompactString.decode(buf)?;
        let name = types::CompactString.decode(buf)?;
        let token_authenticated = types::Boolean.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let mut unknown_value = vec![0; size as usize];
            buf.try_copy_to_slice(&mut unknown_value)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            _type,
            name,
            token_authenticated,
            unknown_tagged_fields,
        })
    }
}

impl Default for DefaultPrincipalData {
    fn default() -> Self {
        Self {
            _type: Default::default(),
            name: Default::default(),
            token_authenticated: false,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DefaultPrincipalData {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

