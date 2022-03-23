///////////////////////////////////////////////////////////////////////////////
// NAME:            computer_system.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Resources for mutating and querying computer systems.
//
// CREATED:         03/17/2022
//
// LAST EDITED:     03/22/2022
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
use uuid::Uuid;

use crate::redfish::ServiceEndpoint;

const ODATA_TYPE: &'static str = "#ComputerSystem.v1_16_1.ComputerSystem";

///////////////////////////////////////////////////////////////////////////////
// Supporting Enums
////

#[derive(Clone, Serialize)]
#[allow(dead_code)]
pub enum SystemType {
    Composed,
    DPU,
    OS,
    Physical,
    PhysicallyPartitioned,
    Virtual,
    VirtuallyPartitioned,
}

impl Default for SystemType {
    fn default() -> Self { SystemType::Physical }
}

#[derive(Clone, Serialize)]
#[allow(dead_code)]
pub enum State {
    Absent,
    Deferring,
    Disabled,
    Enabled,
    InTest,
    Qualified,
    Quiesced,
    StandbyOffline,
    StandbySpare,
    Starting,
    UnavailableOffline,
    Updating,
}

#[derive(Clone, Serialize)]
#[allow(dead_code)]
pub enum Health {
    Critical,
    OK,
    Warning,
}

#[derive(Clone)]
pub struct Status {
    state: State,
    health: Health,
    health_rollup: Health,
}

impl Default for Status {
    fn default() -> Self {
        Status { state: State::Enabled, health: Health::OK,
                 health_rollup: Health::OK }
    }
}

impl Serialize for Status {
    fn serialize<S: Serializer>(&self, serializer: S) ->
        Result<S::Ok, S::Error>
    {
        let mut state = serializer.serialize_struct("ComputerSystem", 8)?;
        state.serialize_field("State", &self.state)?;
        state.serialize_field("Health", &self.health)?;
        state.serialize_field("HealthRollup", &self.health_rollup)?;
        state.end()
    }
}

///////////////////////////////////////////////////////////////////////////////
// ComputerSystem
////

#[derive(Builder, Clone, Default)]
#[builder(setter(into))]
pub struct ComputerSystem {
    #[builder(default, setter(custom))]
    odata_id: PathBuf,

    #[builder(default = "ODATA_TYPE.to_string()", setter(skip))]
    odata_type: String,

    #[builder(default = "SystemType::Physical")]
    system_type: SystemType,

    #[builder(default)]
    uuid: Uuid,

    #[builder(default)]
    status: Status,

    #[builder(setter(custom))]
    id: String,

    name: String,
    serial_number: String,
    hostname: String,
}

impl ComputerSystemBuilder {
    pub fn id(&mut self, id: &str) -> &mut Self {
        self.odata_id = Some(PathBuf::from(id));
        self.id = Some(id.to_string());
        self
    }
}

impl ServiceEndpoint for ComputerSystem {
    fn get_id(&self) -> &Path { &self.odata_id }
    fn resolve(&self, _: PathBuf) -> Self { self.clone() }
}

impl Serialize for ComputerSystem {
    fn serialize<S: Serializer>(&self, serializer: S) ->
        Result<S::Ok, S::Error>
    {
        let mut state = serializer.serialize_struct("ComputerSystem", 9)?;
        state.serialize_field("@odata.id", &self.odata_id)?;
        state.serialize_field("@odata_type", &self.odata_type)?;
        state.serialize_field("SystemType", &self.system_type)?;
        state.serialize_field("UUID", &self.uuid)?;
        state.serialize_field("Status", &self.status)?;
        state.serialize_field("Id", &self.id)?;
        state.serialize_field("Name", &self.name)?;
        state.serialize_field("SerialNumber", &self.serial_number)?;
        state.serialize_field("Hostname", &self.hostname)?;
        // hostname: String,
        // actions: ?
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
