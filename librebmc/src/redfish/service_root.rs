///////////////////////////////////////////////////////////////////////////////
// NAME:            service_root.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Service Root
//
// CREATED:         03/16/2022
//
// LAST EDITED:     03/18/2022
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

use std::borrow::{Borrow, Cow};
use std::{convert::{From, Infallible}, default::Default};
use std::path::{Path, PathBuf};

use hyper::{Body, Request, Response};
use routerify::prelude::*;
use routerify::Router;
use serde_json;
use serde::{Serialize, Serializer, ser::SerializeStruct};
use derive_builder::Builder;
use uuid::Uuid;

// use crate::redfish::ComputerSystemCollection;
use crate::redfish::{ServiceEndpoint, ServiceId};

const ODATA_TYPE: &'static str = "#ServiceRoot.v1_12_0.ServiceRoot";
const SERVICE_PATH: &'static str = "/redfish/v1";
const SCHEMA_VERSION: &'static str = "1.6.0";
const DEFAULT_NAME: &'static str = "Root Service";
const DEFAULT_ID: &'static str = "RootService";

// Define an app state to share it across the route handlers and middlewares.
#[derive(Builder, Default)]
#[builder(setter(into))]
pub struct ServiceRoot<'a> {
    #[builder(default = "Cow::from(ODATA_TYPE)")]
    odata_type: Cow<'a, str>,

    #[builder(default = "Cow::from(DEFAULT_ID)")]
    id: Cow<'a, str>,

    #[builder(default = "Cow::from(DEFAULT_NAME)")]
    name: Cow<'a, str>,

    #[builder(default = "Cow::from(SCHEMA_VERSION)")]
    redfish_version: Cow<'a, str>,

    #[builder(default)]
    uuid: Uuid,

    #[builder(default = "Cow::from(PathBuf::from(SERVICE_PATH))")]
    odata_id: Cow<'a, PathBuf>,

    // #[builder(setter(custom))]
    // systems: ServiceId,
}

// impl ServiceRootBuilder {
//     pub fn systems(&mut self, collection: &mut ComputerSystemCollection) ->
//         &mut ServiceRootBuilder
//     {
//         self.systems = Some(ServiceId::from(collection.get_id().to_owned()));
//         self
//     }
// }

impl ServiceEndpoint for ServiceRoot {
    fn get_id(&self) -> &Path { &self.odata_id }
}

impl ServiceRoot {
    pub fn resolve(&self, path: PathBuf) -> Self {
        let mut result = *self.to_owned();
        result.odata_id = path.join(result.odata_id);
        result.systems = result.odata_id.join(result.systems).into();
        result
    }
}

impl Serialize for ServiceRoot {
    fn serialize<S: Serializer>(&self, serializer: S) ->
        Result<S::Ok, S::Error>
    {
        let mut state = serializer.serialize_struct("ServiceRoot", 7)?;
        state.serialize_field("Id", &self.id)?;
        state.serialize_field("Name", &self.name)?;
        state.serialize_field("RedfishVersion", &self.redfish_version)?;
        state.serialize_field("UUID", &self.uuid)?;
        state.serialize_field("@odata.id", &self.odata_id)?;
        state.serialize_field("@odata.type", &self.odata_type)?;
        state.serialize_field("Systems", &self.systems)?;
        state.end()
    }
}

pub async fn get(request: Request<Body>) ->
    Result<Response<Body>, Infallible>
{
    let service = request.data::<ServiceRoot>().unwrap()
        .resolve(PathBuf::from(request.uri().path()));
    Ok(Response::new(Body::from(
        serde_json::to_string::<ServiceRoot>(&service).unwrap())))
}

// Create a `Router<Body, Infallible>` for response body type `hyper::Body`
// and for handler error type `Infallible`.
pub fn route(service: ServiceRoot) -> Router<Body, Infallible> {
    let mountpoint = service.get_id().to_owned().into_os_string()
        .into_string().unwrap();
    Router::builder()
        // Specify the state data which will be available to every route
        // handlers, error handler and middlewares.
        .data(service)
        .get(mountpoint, get)
        .build()
        .unwrap()
}

///////////////////////////////////////////////////////////////////////////////
