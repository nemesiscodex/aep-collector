use std::pin::Pin;
use futures::{Future, future::{ok, Ready}};
use serde::Deserialize;
use actix_web::{Error, dev::{Service, ServiceRequest, ServiceResponse, Transform}, error::ErrorUnauthorized, web::Query};


/// This middleware will check the api_key query string on every request
#[derive(Clone, Deserialize)]
pub struct ApiKey {
    api_key: String
}

impl ApiKey {
    pub fn new(key: String) -> Self {
        ApiKey { api_key: key }
    }

    pub fn verify(&self, other: &ApiKey) -> bool {
        self.api_key == other.api_key
    }
}

/// Middleware to check the api_key query string on every request
pub struct ApiKeyMiddleware<S> {
    service: S,
    key: ApiKey
}

impl<S, B> Transform<S> for ApiKey
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ApiKeyMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ApiKeyMiddleware{ service, key: self.clone() })
    }
}

impl<S, B> Service for ApiKeyMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, ctx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let raw_query = 
            Query::<ApiKey>::from_query(req.query_string());

        match raw_query {
            Ok(query) if self.key.verify(&query) => {
                let fut = self.service.call(req);

                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            },
            _ => {
                Box::pin(async move {
                    Err(ErrorUnauthorized("Invalid api key"))
                })
            }
        }


        
    }
}