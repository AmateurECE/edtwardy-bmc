///////////////////////////////////////////////////////////////////////////////
// NAME:            service_root.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Service Root
//
// CREATED:         03/16/2022
//
// LAST EDITED:     03/16/2022
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

use std::{convert::{From, Infallible}, default::Default};
use std::path::PathBuf;

use hyper::{Body, Request, Response};
use routerify::prelude::*;
use serde_json;
use serde::{Serialize, Serializer, ser::SerializeStruct};
use derive_builder::Builder;
use uuid::Uuid;

// Define an app state to share it across the route handlers and middlewares.
#[derive(Default, Builder)]
#[builder(setter(into))]
pub struct ServiceRoot {
    #[builder(default = "\"#ServiceRoot.v1_12_0.ServiceRoot\".to_string()")]
    odata_type: String,

    #[builder(default = "\"RootService\".to_string()")]
    id: String,

    #[builder(default = "\"Root Service\".to_string()")]
    name: String,

    #[builder(default = "\"1.6.0\".to_string()")]
    redfish_version: String,

    #[builder(default)]
    uuid: Uuid,

    #[builder(default)]
    odata_id: PathBuf,
}

impl Serialize for ServiceRoot {
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
        state.end()
    }
}

pub async fn v1_service_root(request: Request<Body>) ->
    Result<Response<Body>, Infallible>
{
    let service = request.data::<ServiceRoot>().unwrap();
    Ok(Response::new(Body::from(
        serde_json::to_string::<ServiceRoot>(&service).unwrap())))
}

///////////////////////////////////////////////////////////////////////////////
