// Copyright lowRISC contributors.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

//! `DeviceInfo` request and response.
//!
//! This module provides a Cerberus command that allows the querying of
//! Cerberus and vendor-specified information about the device.

use crate::io::Read;
use crate::io::Write;
use crate::protocol::Command;
use crate::protocol::CommandType;
use crate::protocol::FromWire;
use crate::protocol::FromWireError;
use crate::protocol::Request;
use crate::protocol::Response;
use crate::protocol::ToWire;
use crate::protocol::ToWireError;

#[cfg(feature = "arbitrary-derive")]
use libfuzzer_sys::arbitrary::{self, Arbitrary};

/// A command for requesting device information.
///
/// Corresponds to [`CommandType::DeviceInfo`].
///
/// [`CommandType::DeviceInfo`]:
///     ../enum.CommandType.html#variant.DeviceInfo
pub enum DeviceInfo {}

impl<'a> Command<'a> for DeviceInfo {
    type Req = DeviceInfoRequest;
    type Resp = DeviceInfoResponse<'a>;
}

wire_enum! {
    /// A type of "device information" that can be requested.
    #[cfg_attr(feature = "arbitrary-derive", derive(Arbitrary))]
    pub enum InfoIndex: u8 {
        /// Represents getting the Unique Chip Identifier for the device.
        UniqueChipIndex = 0x00,
    }
}

/// The [`DeviceInfo`] request.
///
/// [`DeviceInfo`]: enum.DeviceInfo.html
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "arbitrary-derive", derive(Arbitrary))]
pub struct DeviceInfoRequest {
    /// Which device information to look up.
    pub index: InfoIndex,
}

impl Request<'_> for DeviceInfoRequest {
    const TYPE: CommandType = CommandType::DeviceInfo;
}

impl<'a> FromWire<'a> for DeviceInfoRequest {
    fn from_wire<R: Read<'a>>(r: &mut R) -> Result<Self, FromWireError> {
        let index = InfoIndex::from_wire(r)?;
        Ok(Self { index })
    }
}

impl<'a> ToWire for DeviceInfoRequest {
    fn to_wire<W: Write>(&self, w: &mut W) -> Result<(), ToWireError> {
        self.index.to_wire(w)?;
        Ok(())
    }
}

make_fuzz_safe! {
    /// The [`DeviceInfo`] response.
    ///
    /// [`DeviceInfo`]: enum.DeviceInfo.html
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct DeviceInfoResponse<'a> as DIRWrap {
        /// The requested information, in some binary format.
        ///
        /// The format of the response depends on which information index was sent.
        /// Only `0x00` is specified by Cerberus, which is reqired to produce the
        /// "Unique Chip Identifier".
        pub info: (&'a [u8]),
    }
}

impl<'a> Response<'a> for DeviceInfoResponse<'a> {
    const TYPE: CommandType = CommandType::DeviceInfo;
}

impl<'a> FromWire<'a> for DeviceInfoResponse<'a> {
    fn from_wire<R: Read<'a>>(r: &mut R) -> Result<Self, FromWireError> {
        let len = r.remaining_data();
        let info = r.read_bytes(len)?;
        Ok(Self { info })
    }
}

impl ToWire for DeviceInfoResponse<'_> {
    fn to_wire<W: Write>(&self, w: &mut W) -> Result<(), ToWireError> {
        w.write_bytes(self.info)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    round_trip_test! {
        request_round_trip: {
            bytes: &[0x0],
            value: DeviceInfoRequest {
                index: InfoIndex::UniqueChipIndex,
            },
        },
        response_round_trip: {
            bytes: b"some unstructured data of no particular length",
            value: DeviceInfoResponse {
                info: b"some unstructured data of no particular length",
            },
        },
    }
}
