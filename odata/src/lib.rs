///////////////////////////////////////////////////////////////////////////////
// NAME:            lib.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     The odata crate provides types and traits for working with
//                  OData resource in an OpenData RESTful API.
//
// CREATED:         04/01/2022
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

use std::path::PathBuf;
use serde::{self, ser::SerializeStruct};

// Must be implemented by traits that can be wrapped with Resource<>
pub trait ResourceMetadata { const ODATA_TYPE: &'static str; }
pub trait Serialize {
    const CARDINALITY: usize;
    fn serialize<S: serde::ser::SerializeStruct>(&self, serializer: &mut S) ->
        Result<(), S::Error>;
}

///////////////////////////////////////////////////////////////////////////////
// Resource
////

// Resource Wrapper. Provides OData metadata for any type.
pub struct Resource<T: Serialize + ResourceMetadata> {
    resource: T,
    odata_id: PathBuf,
    odata_type: &'static str,
}

impl<T: Serialize + ResourceMetadata> Resource<T> {
    pub fn new(odata_id: PathBuf, resource: T) -> Self {
        Resource { odata_id, resource, odata_type: T::ODATA_TYPE }
    }
}

impl<T: Serialize + ResourceMetadata> serde::Serialize for Resource<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) ->
        Result<S::Ok, S::Error>
    {
        let mut state = serializer.serialize_struct(
            self.odata_type, 2 + T::CARDINALITY)?;
        state.serialize_field("@odata.id", &self.odata_id)?;
        state.serialize_field("@odata.type", &self.odata_type)?;
        self.resource.serialize(&mut state)?;
        state.end()
    }
}

///////////////////////////////////////////////////////////////////////////////
// Resource Test
////

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::{Resource, ResourceMetadata, Serialize};
    use serde::ser::SerializeStruct;
    use serde_json;

    struct Person (String);
    impl ResourceMetadata for Person {
        const ODATA_TYPE: &'static str = "#Person";
    }

    impl Serialize for Person {
        const CARDINALITY: usize = 1;
        fn serialize<S: SerializeStruct>(&self, serializer: &mut S) ->
            Result<(), S::Error>
        { serializer.serialize_field("Name", &self.0) }
    }

    #[test]
    fn serialize_correctness() {
        let resource: Resource<Person> = Resource::new(
            PathBuf::from("/Chuck"),
            Person("Chuck".to_string()));
        let result = serde_json::to_string(&resource);
        assert!(result.is_ok());
        assert_eq!(
            "{\"@odata.id\":\"/Chuck\",".to_string()
                + "\"@odata.type\":\"#Person\",\"Name\":\"Chuck\"}",
            result.unwrap()
        )
    }
}

///////////////////////////////////////////////////////////////////////////////
