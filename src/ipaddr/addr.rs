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

use super::section::AddressSection;

struct Address {
	section: AddressSection,
}

struct IPAddress {
	address: Address,
}

impl IPAddress {
	fn to_ipv4(self) -> IPv4Address {
		IPv4Address { address: self }
	}
}

struct IPv4Address {
	address: IPAddress,
}

impl IPv4Address {
	fn to_ip(&self) -> &IPAddress {
		&self.address
	}
}

impl IPv4Address {
	fn to_address_base(&self) -> &Address {
		&self.address.address
	}
}