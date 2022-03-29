///////////////////////////////////////////////////////////////////////////////
// NAME:            service_root.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     ServiceRoot model
//
// CREATED:         03/28/2022
//
// LAST EDITED:     03/28/2022
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

use std::borrow::Cow;
use std::default::Default;
use std::path::{Path, PathBuf};

use serde::{Serialize, Serializer, ser::SerializeStruct};
use derive_builder::Builder;
use uuid::Uuid;

// use crate::redfish::ComputerSystemCollection;
// use crate::models::ComputerSystemCollection;
use crate::models::ServiceEndpoint;

const ODATA_TYPE: &'static str = "#ServiceRoot.v1_12_0.ServiceRoot";
const SERVICE_PATH: &'static str = "redfish/v1";
const SCHEMA_VERSION: &'static str = "1.6.0";
const DEFAULT_NAME: &'static str = "Root Service";
const DEFAULT_ID: &'static str = "RootService";

#[derive(Builder, Clone, Default)]
#[builder(setter(into))]
pub struct ServiceRoot<'a> {
    #[builder(default = "ODATA_TYPE.to_string()")]
    odata_type: String,

    #[builder(default = "DEFAULT_ID.to_string()")]
    id: String,

    #[builder(default = "DEFAULT_NAME.to_string()")]
    name: String,

    #[builder(default = "SCHEMA_VERSION.to_string()")]
    redfish_version: String,

    #[builder(default)]
    uuid: Uuid,

    #[builder(default = "Cow::Owned(PathBuf::from(SERVICE_PATH))")]
    odata_id: Cow<'a, PathBuf>,

    // #[builder(default, setter(custom))]
    // systems: ServiceId<'a>,
}

// impl ServiceRootBuilder<'_> {
//     pub fn systems(&mut self, systems: &ComputerSystemCollection) -> &Self {
//         self.systems = Some(systems.get_id().to_owned().into());
//         self
//     }
// }

impl ServiceEndpoint for ServiceRoot<'_> {
    fn get_id(&self) -> &Path { &self.odata_id }
    fn resolve(&self, path: PathBuf) -> Self {
        let mut result = self.clone();
        // result.systems = path.join(self.systems.as_ref().to_owned()).into();
        result.odata_id = Cow::Owned(path);
        result
    }
}

impl Serialize for ServiceRoot<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) ->
        Result<S::Ok, S::Error>
    {
        let mut state = serializer.serialize_struct("ServiceRoot", 6)?;
        state.serialize_field("Id", &self.id)?;
        state.serialize_field("Name", &self.name)?;
        state.serialize_field("RedfishVersion", &self.redfish_version)?;
        state.serialize_field("UUID", &self.uuid)?;
        state.serialize_field("@odata.id", &self.odata_id)?;
        state.serialize_field("@odata.type", &self.odata_type)?;
        // state.serialize_field("Systems", &self.systems)?;
        state.end()
    }
}

///////////////////////////////////////////////////////////////////////////////
