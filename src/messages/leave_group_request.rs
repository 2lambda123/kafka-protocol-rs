//! LeaveGroupRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/LeaveGroupRequest.json).
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

/// Valid versions: 0-5
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct MemberIdentity {
    /// The member ID to remove from the group.
    ///
    /// Supported API versions: 3-5
    pub member_id: StrBytes,

    /// The group instance ID to remove from the group.
    ///
    /// Supported API versions: 3-5
    pub group_instance_id: Option<StrBytes>,

    /// The reason why the member left the group.
    ///
    /// Supported API versions: 5
    pub reason: Option<StrBytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for MemberIdentity {
    type Builder = MemberIdentityBuilder;

    fn builder() -> Self::Builder {
        MemberIdentityBuilder::default()
    }
}

impl Encodable for MemberIdentity {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 3 {
            if version >= 4 {
                types::CompactString.encode(buf, &self.member_id)?;
            } else {
                types::String.encode(buf, &self.member_id)?;
            }
        } else {
            if !self.member_id.is_empty() {
                return Err(EncodeError);
            }
        }
        if version >= 3 {
            if version >= 4 {
                types::CompactString.encode(buf, &self.group_instance_id)?;
            } else {
                types::String.encode(buf, &self.group_instance_id)?;
            }
        } else {
            if !self.group_instance_id.is_none() {
                return Err(EncodeError);
            }
        }
        if version >= 5 {
            types::CompactString.encode(buf, &self.reason)?;
        }
        if version >= 4 {
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
            if version >= 4 {
                total_size += types::CompactString.compute_size(&self.member_id)?;
            } else {
                total_size += types::String.compute_size(&self.member_id)?;
            }
        } else {
            if !self.member_id.is_empty() {
                return Err(EncodeError);
            }
        }
        if version >= 3 {
            if version >= 4 {
                total_size += types::CompactString.compute_size(&self.group_instance_id)?;
            } else {
                total_size += types::String.compute_size(&self.group_instance_id)?;
            }
        } else {
            if !self.group_instance_id.is_none() {
                return Err(EncodeError);
            }
        }
        if version >= 5 {
            total_size += types::CompactString.compute_size(&self.reason)?;
        }
        if version >= 4 {
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

impl Decodable for MemberIdentity {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let member_id = if version >= 3 {
            if version >= 4 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Default::default()
        };
        let group_instance_id = if version >= 3 {
            if version >= 4 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            None
        };
        let reason = if version >= 5 {
            types::CompactString.decode(buf)?
        } else {
            None
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 4 {
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
            member_id,
            group_instance_id,
            reason,
            unknown_tagged_fields,
        })
    }
}

impl Default for MemberIdentity {
    fn default() -> Self {
        Self {
            member_id: Default::default(),
            group_instance_id: None,
            reason: None,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for MemberIdentity {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

/// Valid versions: 0-5
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct LeaveGroupRequest {
    /// The ID of the group to leave.
    ///
    /// Supported API versions: 0-5
    pub group_id: super::GroupId,

    /// The member ID to remove from the group.
    ///
    /// Supported API versions: 0-2
    pub member_id: StrBytes,

    /// List of leaving member identities.
    ///
    /// Supported API versions: 3-5
    pub members: Vec<MemberIdentity>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for LeaveGroupRequest {
    type Builder = LeaveGroupRequestBuilder;

    fn builder() -> Self::Builder {
        LeaveGroupRequestBuilder::default()
    }
}

impl Encodable for LeaveGroupRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 4 {
            types::CompactString.encode(buf, &self.group_id)?;
        } else {
            types::String.encode(buf, &self.group_id)?;
        }
        if version <= 2 {
            types::String.encode(buf, &self.member_id)?;
        } else {
            if !self.member_id.is_empty() {
                return Err(EncodeError);
            }
        }
        if version >= 3 {
            if version >= 4 {
                types::CompactArray(types::Struct { version }).encode(buf, &self.members)?;
            } else {
                types::Array(types::Struct { version }).encode(buf, &self.members)?;
            }
        } else {
            if !self.members.is_empty() {
                return Err(EncodeError);
            }
        }
        if version >= 4 {
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
        if version >= 4 {
            total_size += types::CompactString.compute_size(&self.group_id)?;
        } else {
            total_size += types::String.compute_size(&self.group_id)?;
        }
        if version <= 2 {
            total_size += types::String.compute_size(&self.member_id)?;
        } else {
            if !self.member_id.is_empty() {
                return Err(EncodeError);
            }
        }
        if version >= 3 {
            if version >= 4 {
                total_size +=
                    types::CompactArray(types::Struct { version }).compute_size(&self.members)?;
            } else {
                total_size +=
                    types::Array(types::Struct { version }).compute_size(&self.members)?;
            }
        } else {
            if !self.members.is_empty() {
                return Err(EncodeError);
            }
        }
        if version >= 4 {
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

impl Decodable for LeaveGroupRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let group_id = if version >= 4 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let member_id = if version <= 2 {
            types::String.decode(buf)?
        } else {
            Default::default()
        };
        let members = if version >= 3 {
            if version >= 4 {
                types::CompactArray(types::Struct { version }).decode(buf)?
            } else {
                types::Array(types::Struct { version }).decode(buf)?
            }
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 4 {
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
            group_id,
            member_id,
            members,
            unknown_tagged_fields,
        })
    }
}

impl Default for LeaveGroupRequest {
    fn default() -> Self {
        Self {
            group_id: Default::default(),
            member_id: Default::default(),
            members: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for LeaveGroupRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

impl HeaderVersion for LeaveGroupRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 4 {
            2
        } else {
            1
        }
    }
}
