///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the application.
//
// CREATED:         02/26/2022
//
// LAST EDITED:     04/05/2022
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

use core::future::{self, Ready};
use core::clone::Clone;
use core::convert::Infallible;
use core::task::{Context, Poll};
use std::path::PathBuf;
use std::sync::Arc;

use hyper::{Body, Request, Response, service::Service};
use odata::{Resource, ResourceMetadata, Serialize};
use serde_json;

mod models;
use crate::models::{ServiceRootBuilder, ComputerSystemCollectionBuilder};

///////////////////////////////////////////////////////////////////////////////
// ResourceService
////

#[derive(Clone)]
pub struct ResourceService<T>(Arc<Resource<T>>)
where T: Serialize + ResourceMetadata + Clone;

impl<T> Service<Request<Body>> for ResourceService<T>
where T: Serialize + ResourceMetadata + Clone {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;
    fn poll_ready(&mut self, _context: &mut Context<'_>) ->
        Poll<Result<(), Self::Error>>
    { Ok(()).into() }

    fn call(&mut self, _request: Request<Body>) -> Self::Future {
        future::ready(
            Ok(Response::builder()
               .status(200)
               .body(Body::from(serde_json::to_string(&*self.0).unwrap()))
               .unwrap())
        )
    }
}

impl<T> From<Resource<T>> for ResourceService<T>
where T: Serialize + ResourceMetadata + Clone {
    fn from(resource: Resource<T>) -> Self {
        ResourceService(Arc::new(resource))
    }
}

///////////////////////////////////////////////////////////////////////////////
// ServiceFactory
////

pub struct ServiceFactory<T>(ResourceService<T>)
where T: Serialize + ResourceMetadata + Clone;

impl<R, T> Service<R> for ServiceFactory<T>
where T: Serialize + ResourceMetadata + Clone {
    type Response = ResourceService<T>;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;
    fn poll_ready(&mut self, _context: &mut Context<'_>) ->
        Poll<Result<(), Self::Error>>
    { Ok(()).into() }

    fn call(&mut self, _: R) -> Self::Future {
        future::ready(Ok(self.0.clone()))
    }
}

impl<T> From<ResourceService<T>> for ServiceFactory<T>
where T: Serialize + ResourceMetadata + Clone {
    fn from(service: ResourceService<T>) -> Self {
        ServiceFactory(service)
    }
}

///////////////////////////////////////////////////////////////////////////////
// Main
////

#[tokio::main]
async fn main() {
    let service = ResourceService::from(Resource::new(
        PathBuf::from("/redfish/v1"),
        ServiceRootBuilder::default().build().unwrap()));

    hyper::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(ServiceFactory::from(service))
        .await
        .unwrap();
}

///////////////////////////////////////////////////////////////////////////////
