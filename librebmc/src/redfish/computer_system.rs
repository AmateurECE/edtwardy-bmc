///////////////////////////////////////////////////////////////////////////////
// NAME:            computer_system.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Resources for mutating and querying computer systems.
//
// CREATED:         03/17/2022
//
// LAST EDITED:     03/20/2022
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

use std::convert::{From, Infallible};
use std::{default::Default, path::{Path, PathBuf}};

use hyper::{Body, Request, Response};
use routerify::prelude::*;
use serde_json;
use serde::{Serialize, Serializer, ser::SerializeStruct};
use derive_builder::Builder;

use crate::redfish::ServiceEndpoint;

///////////////////////////////////////////////////////////////////////////////
// ComputerSystem
////

#[derive(Builder, Clone, Default)]
#[builder(setter(into))]
pub struct ComputerSystem {
    #[builder(default)]
    odata_id: PathBuf,
}

impl ServiceEndpoint for ComputerSystem {
    fn get_id(&self) -> &Path { &self.odata_id }
    fn resolve(&self, _: PathBuf) -> Self { self.clone() }
}

impl Serialize for ComputerSystem {
    fn serialize<S: Serializer>(&self, serializer: S) ->
        Result<S::Ok, S::Error>
    {
        let mut state = serializer.serialize_struct("ComputerSystem", 1)?;
        state.serialize_field("@odata.id", &self.odata_id)?;
        state.end()
    }
}

pub async fn get(request: Request<Body>) -> Result<Response<Body>, Infallible>
{
    let service = request.data::<ComputerSystem>().unwrap()
        .resolve(PathBuf::from(request.uri().path()));
    Ok(Response::new(Body::from(
        serde_json::to_string::<ComputerSystem>(&service).unwrap())))
}

///////////////////////////////////////////////////////////////////////////////
