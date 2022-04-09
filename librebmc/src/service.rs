///////////////////////////////////////////////////////////////////////////////
// NAME:            service.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Logic to compose the service
//
// CREATED:         03/20/2022
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

use core::future::{self, Ready};
use core::clone::Clone;
use core::convert::Infallible;
use core::fmt::Debug;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use hyper::{Body, Method, Request, Response, service::Service};
use odata::{Resource, ResourceMetadata, Serialize};
use serde_json;

///////////////////////////////////////////////////////////////////////////////
// Convenience Responses
////

pub struct NotFound;
impl Into<Response<Body>> for NotFound {
    fn into(self) -> Response<Body> {
        Response::builder().status(404).body("".into()).unwrap()
    }
}

pub struct MethodNotAllowed(Vec<Method>);
impl MethodNotAllowed {
    pub fn new(allowed: Vec<Method>) -> Self {
        MethodNotAllowed(allowed)
    }
}

impl Into<Response<Body>> for MethodNotAllowed {
    fn into(self) -> Response<Body> {
        let allowed = self.0.iter().map(|method| method.as_str())
            .collect::<Vec<&str>>()
            .join(", ");
        Response::builder()
            .status(405)
            .header("Allow", &allowed)
            .body("".into()).unwrap()
    }
}

///////////////////////////////////////////////////////////////////////////////
// Dispatch
////

pub trait Dispatch {
    type Error: Debug;
    fn dispatch(&self, path: &Path, request: &Request<Body>) ->
        Result<Option<Response<Body>>, Self::Error>;
}

///////////////////////////////////////////////////////////////////////////////
// ODataResource
////

pub struct ODataResource<T>(Resource<T>)
where T: Serialize + ResourceMetadata + Dispatch;

impl<T> Dispatch for ODataResource<T>
where T: Serialize + ResourceMetadata + Dispatch {
    type Error = <T as Dispatch>::Error;
    fn dispatch(&self, path: &Path, request: &Request<Body>) ->
        Result<Option<Response<Body>>, Self::Error>
    {
        let this_url = self.0.get_id();
        if this_url.as_ref() == path {
            match request.method() {
                &Method::GET => Ok(Some(
                    Response::builder()
                        .status(200)
                        .body(serde_json::to_string(&self.0).unwrap().into())
                        .unwrap()
                )),

                _ => Ok(Some(MethodNotAllowed::new(vec![Method::GET]).into())),
            }
        }

        else if path.starts_with(this_url.as_ref()) {
            self.0.get().dispatch(
                path.strip_prefix(this_url).unwrap(), request)
        }

        else {
            Ok(None)
        }
    }
}

impl<T> From<Resource<T>> for ODataResource<T>
where T: Serialize + ResourceMetadata + Dispatch {
    fn from(value: Resource<T>) -> Self {
        ODataResource(value)
    }
}

impl<T> serde::Serialize for ODataResource<T>
where T: Serialize + ResourceMetadata + Dispatch {
    fn serialize<S: serde::Serializer>(&self, serializer: S) ->
        Result<S::Ok, S::Error>
    { self.0.serialize(serializer) }
}

///////////////////////////////////////////////////////////////////////////////
// RouteFuture
////

pub struct RouteFuture<T>
where T: Serialize + ResourceMetadata + Clone + Dispatch {
    resource: Arc<ODataResource<T>>,
    request: Request<Body>,
}

impl<T> core::future::Future for RouteFuture<T>
where T: Serialize + ResourceMetadata + Clone + Dispatch {
    type Output = Result<Response<Body>, Infallible>;
    fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) ->
        Poll<Self::Output>
    {
        let path = PathBuf::from(self.request.uri().path());
        let result = (&*self.resource).dispatch(&path, &self.request);
        let response: Response<Body> = match result.unwrap() {
            Some(response) => response,
            None => NotFound.into(),
        };
        Poll::Ready(Ok(response))
    }
}

///////////////////////////////////////////////////////////////////////////////
// ResourceService
////

#[derive(Clone)]
pub struct ResourceService<T>(Arc<ODataResource<T>>)
where T: Serialize + ResourceMetadata + Clone + Dispatch;

impl<T> Service<Request<Body>> for ResourceService<T>
where T: Serialize + ResourceMetadata + Clone + Dispatch {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = RouteFuture<T>;
    fn poll_ready(&mut self, _context: &mut Context<'_>) ->
        Poll<Result<(), Self::Error>>
    { Ok(()).into() }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        RouteFuture { resource: self.0.clone(), request }
    }
}

impl<T> From<ODataResource<T>> for ResourceService<T>
where T: Serialize + ResourceMetadata + Clone + Dispatch {
    fn from(resource: ODataResource<T>) -> Self {
        ResourceService(Arc::new(resource))
    }
}

///////////////////////////////////////////////////////////////////////////////
// ServiceFactory
////

pub struct ServiceFactory<T>(ResourceService<T>)
where T: Serialize + ResourceMetadata + Clone + Dispatch;

impl<R, T> Service<R> for ServiceFactory<T>
where T: Serialize + ResourceMetadata + Clone + Dispatch {
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
where T: Serialize + ResourceMetadata + Clone + Dispatch {
    fn from(service: ResourceService<T>) -> Self {
        ServiceFactory(service)
    }
}

///////////////////////////////////////////////////////////////////////////////
