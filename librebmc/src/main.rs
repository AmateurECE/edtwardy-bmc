///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the application.
//
// CREATED:         02/26/2022
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

use std::sync::Arc;
use std::path::PathBuf;

use axum::{routing::get, Router};
use odata::Resource;
use serde_json;

mod models;
use crate::models::{
    ServiceRoot, ServiceRootBuilder,
    ComputerSystemCollection, ComputerSystemCollectionBuilder
};

async fn get_computer_system_collection(systems: Arc<Resource<ComputerSystemCollection>>) ->
    String
{ serde_json::to_string(&*systems).unwrap() }

async fn get_service_root(service: Arc<Resource<ServiceRoot>>) -> String {
    serde_json::to_string(&*service).unwrap()
}

#[tokio::main]
async fn main() {
    let systems = Arc::new(odata::Resource::new(
        PathBuf::from("/redfish/v1/Systems"),
        ComputerSystemCollectionBuilder::default().build().unwrap()));
    let service = Arc::new(odata::Resource::new(
        PathBuf::from("/redfish/v1"),
        ServiceRootBuilder::default().systems(&*systems).build().unwrap()));

    let app = Router::new()
        .route("/redfish/v1", get({
            let service = Arc::clone(&service);
            move || get_service_root(service)
        }))
        .route("/redfish/v1/Systems", get({
            let systems = Arc::clone(&systems);
            move || get_computer_system_collection(systems)
        }));
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

///////////////////////////////////////////////////////////////////////////////
