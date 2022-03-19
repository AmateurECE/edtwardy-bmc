///////////////////////////////////////////////////////////////////////////////
// NAME:            computer_system.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Resources for mutating and querying computer systems.
//
// CREATED:         03/17/2022
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
use std::collections::HashMap;
use std::convert::{From, Infallible};
use std::{default::Default, path::{Path, PathBuf}};

use hyper::{Body, Request, Response};
use routerify::prelude::*;
use routerify::Router;
use serde_json;
use serde::{Serialize, Serializer, ser::SerializeStruct};
use derive_builder::Builder;

use crate::redfish::{ServiceEndpoint, ServiceId};

///////////////////////////////////////////////////////////////////////////////
// ComputerSystem
////

// #[derive(Builder, Clone, Default)]
// #[builder(setter(into))]
// pub struct ComputerSystem {
//     #[builder(default)]
//     odata_id: PathBuf,
// }

// impl Serialize for ComputerSystem {
//     fn serialize<S: Serializer>(&self, serializer: S) ->
//         Result<S::Ok, S::Error>
//     {
//         let mut state = serializer.serialize_struct("ComputerSystem", 1)?;
//         state.serialize_field("@odata.id", &self.odata_id)?;
//         state.end()
//     }
// }

// impl ServiceEndpoint for ComputerSystem {
//     fn get_id(&self) -> &Path { &self.odata_id }
//     fn set_id(&mut self, id: PathBuf) { self.odata_id = id; }
// }

///////////////////////////////////////////////////////////////////////////////
// ComputerSystemCollection
////

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
    members: HashMap<String, ServiceId<'a>>,
}

impl ServiceEndpoint for ComputerSystemCollection<'_> {
    fn get_id(&self) -> &Path { &self.odata_id }
    fn resolve(&self, path: PathBuf) -> Self {
        let mut result = self.clone();
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

pub async fn get(request: Request<Body>) ->
    Result<Response<Body>, Infallible>
{
    let service = request.data::<ComputerSystemCollection>().unwrap()
        .resolve(PathBuf::from(request.uri().path()));
    Ok(Response::new(Body::from(
        serde_json::to_string::<ComputerSystemCollection>(&service).unwrap())))
}

// Create a `Router<Body, Infallible>` for response body type `hyper::Body`
// and for handler error type `Infallible`.
pub fn route(service: ComputerSystemCollection<'static>) ->
    Router<Body, Infallible>
{
    let mountpoint = "/".to_string() + service.get_id().to_owned().as_os_str()
        .to_str().unwrap();
    Router::builder()
        // Specify the state data which will be available to every route
        // handlers, error handler and middlewares.
        .data(service)
        .get(mountpoint, get)
        .build()
        .unwrap()
}

///////////////////////////////////////////////////////////////////////////////
