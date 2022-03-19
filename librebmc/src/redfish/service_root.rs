///////////////////////////////////////////////////////////////////////////////
// NAME:            service_root.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Service Root
//
// CREATED:         03/16/2022
//
// LAST EDITED:     03/19/2022
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
use crate::redfish::ComputerSystemCollection;

const ODATA_TYPE: &'static str = "#ServiceRoot.v1_12_0.ServiceRoot";
const SERVICE_PATH: &'static str = "redfish/v1";
const SCHEMA_VERSION: &'static str = "1.6.0";
const DEFAULT_NAME: &'static str = "Root Service";
const DEFAULT_ID: &'static str = "RootService";

// Define an app state to share it across the route handlers and middlewares.
#[derive(Builder, Clone, Default)]
#[builder(setter(into), build_fn(skip))]
pub struct ServiceRoot<'a> {
    odata_type: String,
    id: String,
    name: String,
    redfish_version: String,
    uuid: Uuid,
    odata_id: Cow<'a, PathBuf>,
    systems: ComputerSystemCollection<'a>,
}

impl ServiceEndpoint for ServiceRoot<'_> {
    fn get_id(&self) -> &Path { &self.odata_id }
    fn resolve(&self, path: PathBuf) -> Self {
        let mut result = self.clone();
        result.odata_id = Cow::Owned(path);
        result
    }
}

impl Serialize for ServiceRoot<'_> {
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

        let systems = ServiceId::from(self.odata_id.join(
            self.systems.get_id()));
        state.serialize_field("Systems", &systems)?;
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

impl ServiceRootBuilder<'static> {
    // Create a `Router<Body, Infallible>` for response body type `hyper::Body`
    // and for handler error type `Infallible`.
    pub fn build(&self) -> Router<Body, Infallible> {
        let service = ServiceRoot {
            odata_type: self.odata_type.as_ref()
                .unwrap_or(&ODATA_TYPE.to_string()).clone(),
            odata_id: self.odata_id.as_ref()
                .unwrap_or(&Cow::Owned(PathBuf::from(SERVICE_PATH))).clone(),
            id: self.id.as_ref().unwrap_or(&DEFAULT_ID.to_string()).clone(),
            name: self.name.as_ref().unwrap_or(&DEFAULT_NAME.to_string())
                .clone(),
            redfish_version: self.redfish_version.as_ref()
                .unwrap_or(&SCHEMA_VERSION.to_string()).clone(),
            uuid: self.uuid.as_ref().unwrap_or(&Uuid::default()).clone(),
            systems: self.systems.as_ref()
                .unwrap_or(&ComputerSystemCollection::default()).clone(),
        };
        let mountpoint = "/".to_string() + service.get_id().to_owned()
            .as_os_str().to_str().unwrap();
        Router::builder()
        // Specify the state data which will be available to every route
        // handlers, error handler and middlewares.
            .data(service)
            .get(mountpoint, get)
            // .scope(computer_system::route(self.systems.clone()))
            .build()
            .unwrap()
    }
}

///////////////////////////////////////////////////////////////////////////////
