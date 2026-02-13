use crate::models::api_key::ApiKey;
use crate::utils::crypto::hash_api_key;
use actix_web::body::EitherBody;
use actix_web::{
    Error, HttpResponse,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
};
use futures::future::{LocalBoxFuture, Ready, ok};
use log::info;
use sqlx::PgPool;
use std::rc::Rc;

pub struct ApiKeyAuth;

impl<S, B> Transform<S, ServiceRequest> for ApiKeyAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = ApiKeyAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ApiKeyAuthMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct ApiKeyAuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ApiKeyAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let s_cloned = self.service.clone();

        Box::pin(async move {
            let api_key = req
                .headers()
                .get("x-goog-api-key")
                .and_then(|hv| hv.to_str().ok());

            let api_key = match api_key {
                Some(key) => key,
                None => {
                    info!("API Key missing, returning 401 Unauthorized");
                    let (req, _pl) = req.into_parts();
                    let res = HttpResponse::Unauthorized().finish().map_into_right_body();
                    return Ok(ServiceResponse::new(req, res));
                }
            };

            let pool = req
                .app_data::<Data<PgPool>>()
                .expect("Database pool not found in app data");

            let hashed_api_key = hash_api_key(api_key);

            let result = ApiKey::find_by_hashed_key(pool.get_ref(), &hashed_api_key).await;

            match result {
                Ok(Some(key)) if key.is_active => {
                    info!("API Key valid, forwarding request");
                    s_cloned.call(req).await.map(|res| res.map_into_left_body())
                }
                Ok(_) => {
                    info!("API Key invalid or inactive, returning 403 Forbidden");
                    let (req, _pl) = req.into_parts();
                    let res = HttpResponse::Forbidden().finish().map_into_right_body();
                    Ok(ServiceResponse::new(req, res))
                }
                Err(e) => {
                    info!("Database error during API Key validation: {:?}", e);
                    let (req, _pl) = req.into_parts();
                    let res = HttpResponse::InternalServerError()
                        .finish()
                        .map_into_right_body();
                    Ok(ServiceResponse::new(req, res))
                }
            }
        })
    }
}
