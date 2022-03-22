///////////////////////////////////////////////////////////////////////////////
// NAME:            computer_system_collection.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     ComputerSystemCollection endpoint.
//
// CREATED:         03/20/2022
//
// LAST EDITED:     03/21/2022
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
use std::convert::{From, Infallible};
use std::{default::Default, path::{Path, PathBuf}};

use hyper::{Body, Request, Response};
use routerify::prelude::*;
use serde_json;
use serde::{Serialize, Serializer, ser::SerializeStruct};
use derive_builder::Builder;

use crate::redfish::{ServiceEndpoint, ServiceId};
use crate::redfish::ComputerSystem;

const ODATA_TYPE: &'static str =
    "#ComputerSystemCollection.ComputerSystemCollection";
const DEFAULT_NAME: &'static str = "Computer System Collection";
const SERVICE_PATH: &'static str = "Systems";

#[derive(Builder, Clone, Default)]
#[builder(setter(into))]
pub struct ComputerSystemCollection<'a> {
    #[builder(default = "ODATA_TYPE.to_string()")]
    odata_type: String,

    #[builder(default = "Cow::Owned(PathBuf::from(SERVICE_PATH))")]
    odata_id: Cow<'a, PathBuf>,

    #[builder(default = "DEFAULT_NAME.to_string()")]
    name: String,

    #[builder(default)]
    members: Vec<ServiceId<'a>>,
}

impl ComputerSystemCollection<'_> {
    pub fn add_system(&mut self, system: &ComputerSystem) {
        self.members.push(system.get_id().to_owned().into());
    }
}

impl ServiceEndpoint for ComputerSystemCollection<'_> {
    fn get_id(&self) -> &Path { &self.odata_id }
    fn resolve(&self, path: PathBuf) -> Self {
        let mut result = self.clone();
        result.members = self.members.iter()
            .map(|s| ServiceId::from(path.join(s.as_ref())))
            .collect::<Vec<ServiceId<'_>>>();
        result.odata_id = Cow::Owned(path);
        result
    }
}

impl Serialize for ComputerSystemCollection<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) ->
        Result<S::Ok, S::Error>
    {
        let mut state = serializer.serialize_struct(
            "ComputerSystemCollection", 5)?;
        state.serialize_field("Name", &self.name)?;
        state.serialize_field("Members@odata.count", &self.members.len())?;
        state.serialize_field("Members", &self.members)?;
        state.serialize_field("@odata.id", &self.odata_id)?;
        state.serialize_field("@odata.type", &self.odata_type)?;
        state.end()
    }
}

pub async fn get(request: Request<Body>) -> Result<Response<Body>, Infallible>
{
    let service = request.data::<ComputerSystemCollection>().unwrap()
        .resolve(PathBuf::from(request.uri().path()));
    Ok(Response::new(Body::from(
        serde_json::to_string::<ComputerSystemCollection>(&service).unwrap())))
}

///////////////////////////////////////////////////////////////////////////////
