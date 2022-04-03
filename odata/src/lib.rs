///////////////////////////////////////////////////////////////////////////////
// NAME:            lib.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoints for the odata crate.
//
// CREATED:         04/01/2022
//
// LAST EDITED:     04/01/2022
////

use std::path::PathBuf;

use serde::{self, ser::SerializeStruct};

pub trait ResourceMetadata {
    const ODATA_TYPE: &'static str;
}

pub trait Serialize {
    const CARDINALITY: usize;
    fn serialize<S: serde::ser::SerializeStruct>(&self, serializer: &mut S) ->
        Result<(), S::Error>;
}

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
// Test
////

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::{Resource, ResourceMetadata, Serialize};
    use serde::ser::SerializeStruct;

    struct Person {
        name: String,
    }

    impl ResourceMetadata for Person {
        const ODATA_ID: &'static str = "#Person";
    }

    impl Serialize for Person {
        const CARDINALITY: usize = 1;
        fn serialize<S: SerializeStruct>(&self, serializer: &mut S) ->
            Result<(), S::Error>
        {
            serializer.serialize_field("Name", &self.name)
        }
    }

    #[test]
    fn it_works() {
        let chuck = Person { name: "Chuck".to_string() };
        let resource: Resource<Person> = Resource {
            odata_id: PathBuf::from("/Chuck"),
            odata_type: PERSON_TYPE,
            resource: chuck,
        };

        assert!(serde_json::to_string(&resource).is_ok())
    }
}

///////////////////////////////////////////////////////////////////////////////
