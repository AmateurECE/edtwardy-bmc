///////////////////////////////////////////////////////////////////////////////
// NAME:            service.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic to compose the service
//
// CREATED:         03/20/2022
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

use std::convert::Infallible;
use hyper::{Body};
use routerify::Router;

use crate::redfish::{self, ServiceEndpoint};

pub fn compose() -> Router<Body, Infallible> {
    let systems = redfish::ComputerSystemCollectionBuilder::default().build()
        .unwrap();
    // let systems_mountpoint = "/".to_string() + systems.get_id().as_os_str()
    //     .into_str().unwrap();

    let service = redfish::ServiceRootBuilder::default()
        .systems(&systems)
        .build()
        .unwrap();

    let service_mountpoint = "/".to_string() + service.get_id().to_owned()
        .as_os_str().to_str().unwrap();
    Router::builder()
        // Specify the state data which will be available to every route
        // handlers, error handler and middlewares.
        .data(service)
        .get(service_mountpoint, redfish::service_root::get)
        .build()
        .unwrap()
}

///////////////////////////////////////////////////////////////////////////////