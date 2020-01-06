use std::convert::TryFrom;
use std::fmt;

use http::Method;
use url::Url;
pub use web_sys::{RequestMode, RequestCache};

use super::{Body, Client, Response};
use crate::header::{HeaderMap, HeaderName, HeaderValue};

/// A request which can be executed with `Client::execute()`.
pub struct Request {
    method: Method,
    url: Url,
    headers: HeaderMap,
    body: Option<Body>,
    fetch_mode: Option<RequestMode>,
    cache_mode: Option<RequestCache>,
}

/// A builder to construct the properties of a `Request`.
pub struct RequestBuilder {
    client: Client,
    request: crate::Result<Request>,
}

impl Request {
    pub(super) fn new(method: Method, url: Url) -> Self {
        Request {
            method,
            url,
            headers: HeaderMap::new(),
            body: None,
            fetch_mode: None,
            cache_mode: None,
        }
    }

    /// Get the method.
    #[inline]
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Get a mutable reference to the method.
    #[inline]
    pub fn method_mut(&mut self) -> &mut Method {
        &mut self.method
    }

    /// Get the url.
    #[inline]
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Get a mutable reference to the url.
    #[inline]
    pub fn url_mut(&mut self) -> &mut Url {
        &mut self.url
    }

    /// Get the headers.
    #[inline]
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Get a mutable reference to the headers.
    #[inline]
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    /// Get the body.
    #[inline]
    pub fn body(&self) -> Option<&Body> {
        self.body.as_ref()
    }

    /// Get a mutable reference to the body.
    #[inline]
    pub fn body_mut(&mut self) -> &mut Option<Body> {
        &mut self.body
    }

    /// Get the request fetch mode.
    /// To know the mode, refer https://developer.mozilla.org/en-US/docs/Web/API/Request/mode
    #[inline]
    pub fn fetch_mode(&self) -> Option<&RequestMode> {
        self.fetch_mode.as_ref()
    }

    /// Get a mutable reference to the request fetch mode.
    /// To know the mode, refer https://developer.mozilla.org/en-US/docs/Web/API/Request/mode
    #[inline]
    pub fn fetch_mode_mut(&mut self) -> &mut Option<RequestMode> {
        &mut self.fetch_mode
    }

    /// Get the cache mode.
    /// To know the mode, refer https://developer.mozilla.org/en-US/docs/Web/API/Request/cache
    #[inline]
    pub fn cache_mode(&self) -> Option<&RequestCache> {
        self.cache_mode.as_ref()
    }

    /// Get a mutable reference to the cache mode.
    /// To know the mode, refer https://developer.mozilla.org/en-US/docs/Web/API/Request/cache
    #[inline]
    pub fn cache_mode_mut(&mut self) -> &mut Option<RequestCache> {
        &mut self.cache_mode
    }
}

impl RequestBuilder {
    pub(super) fn new(client: Client, request: crate::Result<Request>) -> RequestBuilder {
        RequestBuilder { client, request }
    }

    /// Set the request body.
    pub fn body<T: Into<Body>>(mut self, body: T) -> RequestBuilder {
        if let Ok(ref mut req) = self.request {
            req.body = Some(body.into());
        }
        self
    }

    /// Add a `Header` to this Request.
    pub fn header<K, V>(mut self, key: K, value: V) -> RequestBuilder
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        let mut error = None;
        if let Ok(ref mut req) = self.request {
            match <HeaderName as TryFrom<K>>::try_from(key) {
                Ok(key) => match <HeaderValue as TryFrom<V>>::try_from(value) {
                    Ok(value) => {
                        req.headers_mut().append(key, value);
                    }
                    Err(e) => error = Some(crate::error::builder(e.into())),
                },
                Err(e) => error = Some(crate::error::builder(e.into())),
            };
        }
        if let Some(err) = error {
            self.request = Err(err);
        }
        self
    }

    /// Set a request fetch mode to this request.
    /// To know the mode, refer https://developer.mozilla.org/en-US/docs/Web/API/Request/mode
    pub fn fetch_mode(mut self, mode: RequestMode) -> RequestBuilder {
        if let Ok(ref mut req) = self.request {
            req.fetch_mode = Some(mode);
        }
        self
    }

    /// Set a request cache mode to this request.
    /// To know the mode, refer https://developer.mozilla.org/en-US/docs/Web/API/Request/cache
    pub fn cache_mode(mut self, mode: RequestCache) -> RequestBuilder {
        if let Ok(ref mut req) = self.request {
            req.cache_mode = Some(mode);
        }
        self
    }

    /// Constructs the Request and sends it to the target URL, returning a
    /// future Response.
    ///
    /// # Errors
    ///
    /// This method fails if there was an error while sending request.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use reqwest::Error;
    /// #
    /// # async fn run() -> Result<(), Error> {
    /// let response = reqwest::Client::new()
    ///     .get("https://hyper.rs")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(self) -> crate::Result<Response> {
        let req = self.request?;
        self.client.execute_request(req).await
    }
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_request_fields(&mut f.debug_struct("Request"), self).finish()
    }
}

impl fmt::Debug for RequestBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct("RequestBuilder");
        match self.request {
            Ok(ref req) => fmt_request_fields(&mut builder, req).finish(),
            Err(ref err) => builder.field("error", err).finish(),
        }
    }
}

fn fmt_request_fields<'a, 'b>(
    f: &'a mut fmt::DebugStruct<'a, 'b>,
    req: &Request,
) -> &'a mut fmt::DebugStruct<'a, 'b> {
    f.field("method", &req.method)
        .field("url", &req.url)
        .field("headers", &req.headers)
}
