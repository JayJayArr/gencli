use thiserror::Error;
#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Http Client Error")]
    Reqwest(#[from] reqwest::Error),
    #[error("Authentication Reqwest Error")]
    Authentication(#[from] oauth2::reqwest::Error),
    #[error("Authentication Reqwest Token Error")]
    TokenRequestError(
        #[from]
        oauth2::RequestTokenError<
            oauth2::HttpClientError<oauth2::reqwest::Error>,
            oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
        >,
    ),
    #[error("OAuth Url Parse Error")]
    OAuthUrlParse(#[from] oauth2::url::ParseError),
    #[error("Deserialization Error")]
    Deserialization(#[from] serde_json::Error),
}
