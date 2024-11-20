use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage, HttpResponse};
use actix_web::body::EitherBody;
use futures::future::{ok, LocalBoxFuture, Ready};
use uuid::Uuid;
use crate::utils::jwt::decode_token;

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut authentication_pass = false;
        let mut user_id: Option<Uuid> = None;

        // Check Authorization header for Bearer token
        if let Some(header) = req.headers().get("Authorization") {
            if let Ok(str_header) = header.to_str() {
                if str_header.starts_with("Bearer ") {
                    let token = &str_header[7..];

                    // Decode the token and get user id
                    if let Ok(user_token) = decode_token(token.to_string()) {
                        authentication_pass = true;
                        user_id = Some(user_token.claims.id);
                    }
                }
            }
        }

        // If authentication failed, return Unauthorized response
        if !authentication_pass {
            let (request, _pl) = req.into_parts();
            let response = HttpResponse::Unauthorized().finish();
            
            // Bungkus response body dalam EitherBody
            let response = response.map_into_right_body();
        
            // Kembalikan response yang sudah dibungkus
            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        req.extensions_mut().insert(user_id);

        let fut = self.service.call(req);

        Box::pin(async move { fut.await.map(ServiceResponse::map_into_left_body) })
    }
}
