pub struct HeaderToken {
    pub message: String,
}

#[cfg(feature = "actix-web")]
mod actix_impl {
    use super::HeaderToken;
    use crate::errors::{NanoServiceError, NanoServiceErrorStatus};
    pub use actix_web::{FromRequest as ActixFromRequest, HttpRequest, dev::Payload};
    use futures::future::{Ready, err, ok};

    impl ActixFromRequest for HeaderToken {
        type Error = NanoServiceError;
        type Future = Ready<Result<HeaderToken, NanoServiceError>>;
        fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
            let raw_data = match req.headers().get("token") {
                Some(data) => data,
                None => {
                    return err(NanoServiceError {
                        status: NanoServiceErrorStatus::Unauthorized,
                        message: "token not in header under key 'token'".to_string(),
                    });
                }
            };
            let message = match raw_data.to_str() {
                Ok(token) => token.to_string(),
                Err(_) => {
                    return err(NanoServiceError {
                        status: NanoServiceErrorStatus::Unauthorized,
                        message: "token not a valid string".to_string(),
                    });
                }
            };
            ok(HeaderToken { message })
        }
    }
}
