///////////////////////////////////////////////////////////////////////////////
// NAME:            redfish.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Structs and Traits for serving Redfish endpoints with
//                  Routerify.
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

use std::borrow::Cow;
use std::{convert::From, default::Default, path::{Path, PathBuf}};
use serde::{Serializer, Serialize, ser::SerializeStruct};

pub mod service_root;
pub use service_root::ServiceRoot;
pub use service_root::ServiceRootBuilder;

pub mod computer_system;
pub use computer_system::ComputerSystemCollection;
pub use computer_system::ComputerSystemCollectionBuilder;

///////////////////////////////////////////////////////////////////////////////
// ServiceId
////

#[derive(Clone, Default)]
pub struct ServiceId<'a> {
    odata_id: Cow<'a, PathBuf>,
}

impl<'a> From<PathBuf> for ServiceId<'a> {
    fn from(value: PathBuf) -> ServiceId<'a> {
        ServiceId { odata_id: Cow::Owned(value) }
    }
}

impl AsRef<Path> for ServiceId<'_> {
    fn as_ref(&self) -> &Path { &self.odata_id }
}

impl Serialize for ServiceId<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) ->
        Result<S::Ok, S::Error>
    {
        let mut state = serializer.serialize_struct("ServiceId", 1)?;
        state.serialize_field("@odata.id", &self.odata_id)?;
        state.end()
    }
}

// Trait can be used to get exact routes to service endpoints.
pub trait ServiceEndpoint {
    fn get_id(&self) -> &Path;
}

///////////////////////////////////////////////////////////////////////////////
