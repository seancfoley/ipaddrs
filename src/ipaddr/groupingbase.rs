//
// Copyright 2024 Sean C Foley
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//




use std::char::ToLowercase;
use crate::ipaddr::AddrType;
use crate::ipaddr::types::PrefixLen;

trait DivValues {
	fn get_byte_count(&self) -> u8;
}

pub struct AddressDivision {
	value: u64,
	upper_value: u64,
	prefix_len: PrefixLen,
}

pub trait DivArray {
	fn get_divisions(&self) -> [AddressDivision];
	fn get_division(&self, index: u32) -> &AddressDivision;
	fn get_division_count(&self) -> u32;
}

pub struct AddressDivisionGroupingBase {
	divisions: Box<dyn DivArray>,
}

pub struct StandardDivArray {
	divs: Box<[AddressDivision]>

}

pub struct AddressSegment {
	div: AddressDivision,
}

pub struct IPAddressSegment {
	seg: AddressSegment,
}

pub struct IPv4AddressSegment {
	seg: IPAddressSegment,
}

