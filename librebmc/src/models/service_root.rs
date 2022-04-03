///////////////////////////////////////////////////////////////////////////////
// NAME:            service_root.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     ServiceRoot model
//
// CREATED:         03/28/2022
//
// LAST EDITED:     04/03/2022
//
// Copyright 2022, Ethan D. Twardy
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
////

use std::default::Default;

use odata;
use derive_builder::Builder;
use uuid::Uuid;

use crate::models::ComputerSystemCollection;

const SCHEMA_VERSION: &'static str = "1.6.0";
const DEFAULT_NAME: &'static str = "Root Service";
const DEFAULT_ID: &'static str = "RootService";

#[derive(Builder, Clone, Default)]
#[builder(setter(into))]
pub struct ServiceRoot {
    #[builder(default = "DEFAULT_ID.to_string()")]
    id: String,

    #[builder(default = "DEFAULT_NAME.to_string()")]
    name: String,

    #[builder(default = "SCHEMA_VERSION.to_string()")]
    redfish_version: String,

    #[builder(default)]
    uuid: Uuid,

    #[builder(default, setter(custom))]
    systems: Option<odata::Link>,
}

impl odata::ResourceMetadata for ServiceRoot {
    const ODATA_TYPE: &'static str = "#ServiceRoot.v1_12_0.ServiceRoot";
}

impl ServiceRootBuilder {
    pub fn systems(
        &mut self, systems: &odata::Resource<ComputerSystemCollection>
    ) -> &Self {
        self.systems = Some(Some(systems.get_id()));
        self
    }
}

impl odata::Serialize for ServiceRoot {
    const CARDINALITY: usize = 4;
    fn serialize<S: serde::ser::SerializeStruct>(&self, serializer: &mut S) ->
        Result<(), S::Error>
    {
        serializer.serialize_field("Id", &self.id)?;
        serializer.serialize_field("Name", &self.name)?;
        serializer.serialize_field("RedfishVersion", &self.redfish_version)?;
        serializer.serialize_field("UUID", &self.uuid)?;
        serializer.serialize_field("Systems", &self.systems)
    }
}

///////////////////////////////////////////////////////////////////////////////
