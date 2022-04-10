///////////////////////////////////////////////////////////////////////////////
// NAME:            computer_system_collection.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     ComputerSystemCollection model.
//
// CREATED:         04/03/2022
//
// LAST EDITED:     04/09/2022
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

use std::convert::Infallible;
use std::default::Default;
use std::path::Path;

use hyper::{Body, Request, Response};
use serde;
use derive_builder::Builder;
use odata;

use crate::service::Dispatch;

const DEFAULT_NAME: &'static str = "Computer System Collection";

#[derive(Builder, Clone, Default)]
#[builder(setter(into))]
pub struct ComputerSystemCollection {
    #[builder(default = "DEFAULT_NAME.to_string()")]
    name: String,
}

impl odata::ResourceMetadata for ComputerSystemCollection {
    const ODATA_TYPE: &'static str =
        "#ComputerSystemCollection.ComputerSystemCollection";
}

impl odata::Serialize for ComputerSystemCollection {
    const CARDINALITY: usize = 1;
    fn serialize<S>(&self, serializer: &mut S, _me: &Path) ->
        Result<(), S::Error>
    where S: serde::ser::SerializeStruct
    {
        serializer.serialize_field("Name", &self.name)
        // state.serialize_field("Members@odata.count", &self.members.len())?;
        // state.serialize_field("Members", &self.members)?;
    }
}

impl Dispatch for ComputerSystemCollection {
    type Error = Infallible;
    fn dispatch(&self, _path: &Path, _request: &Request<Body>) ->
        Result<Option<Response<Body>>, Self::Error>
    { Ok(None) }
}

///////////////////////////////////////////////////////////////////////////////
