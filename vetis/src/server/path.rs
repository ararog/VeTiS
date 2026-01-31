use std::{borrow::Cow, fs, future::Future, pin::Pin};

use crate::{
    errors::{VetisError, VirtualHostError},
    server::virtual_host::BoxedHandlerClosure,
    Request, Response,
};

pub trait Path {
    fn value(&self) -> &str;
    fn handle<'a>(
        &'a self,
        request: Request,
        uri: Cow<'a, str>,
    ) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + 'a>>;
}

pub enum HostPath {
    Handler(HandlerPath),
    Proxy(ProxyPath),
    Static(StaticPath),
}

impl Path for HostPath {
    fn value(&self) -> &str {
        match self {
            HostPath::Handler(handler) => handler.value(),
            HostPath::Proxy(proxy) => proxy.value(),
            HostPath::Static(static_path) => static_path.value(),
        }
    }

    fn handle<'a>(
        &'a self,
        request: Request,
        uri: Cow<'a, str>,
    ) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + 'a>> {
        match self {
            HostPath::Handler(handler) => handler.handle(request, uri),
            HostPath::Proxy(proxy) => proxy.handle(request, uri),
            HostPath::Static(static_path) => static_path.handle(request, uri),
        }
    }
}

pub struct HandlerPath {
    uri: String,
    handler: BoxedHandlerClosure,
}

impl HandlerPath {
    pub fn new_host_path(uri: String, handler: BoxedHandlerClosure) -> HostPath {
        HostPath::Handler(Self { uri, handler })
    }
}

impl Path for HandlerPath {
    fn value(&self) -> &str {
        &self.uri
    }

    fn handle<'a>(
        &'a self,
        request: Request,
        uri: Cow<'a, str>,
    ) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + 'a>> {
        (self.handler)(request)
    }
}

pub struct StaticPathBuilder {
    uri: String,
    extensions: String,
    directory: String,
}

impl StaticPathBuilder {
    pub fn uri(mut self, uri: String) -> Self {
        self.uri = uri;
        self
    }

    pub fn extensions(mut self, extensions: String) -> Self {
        self.extensions = extensions;
        self
    }

    pub fn directory(mut self, directory: String) -> Self {
        self.directory = directory;
        self
    }

    pub fn build(self) -> Result<HostPath, VetisError> {
        if self.uri.is_empty() {
            return Err(VetisError::VirtualHost(VirtualHostError::InvalidPath(
                "URI cannot be empty".to_string(),
            )));
        }
        if self
            .extensions
            .is_empty()
        {
            return Err(VetisError::VirtualHost(VirtualHostError::InvalidPath(
                "Extensions cannot be empty".to_string(),
            )));
        }
        if self
            .directory
            .is_empty()
        {
            return Err(VetisError::VirtualHost(VirtualHostError::InvalidPath(
                "Directory cannot be empty".to_string(),
            )));
        }

        Ok(HostPath::Static(StaticPath {
            uri: self.uri,
            extensions: self.extensions,
            directory: self.directory,
        }))
    }
}

pub struct StaticPath {
    uri: String,
    extensions: String,
    directory: String,
}

impl StaticPath {
    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn extensions(&self) -> &str {
        &self.extensions
    }

    pub fn directory(&self) -> &str {
        &self.directory
    }

    pub fn builder() -> StaticPathBuilder {
        StaticPathBuilder {
            uri: String::new(),
            extensions: String::new(),
            directory: String::new(),
        }
    }
}

impl Path for StaticPath {
    fn value(&self) -> &str {
        &self.uri
    }

    fn handle<'a>(
        &'a self,
        request: Request,
        uri: Cow<'a, str>,
    ) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + 'a>> {
        Box::pin(async move {
            let ext_regex = regex::Regex::new(&self.extensions);
            if let Ok(ext_regex) = ext_regex {
                if !ext_regex.is_match(uri.as_ref()) {
                    return Ok(Response::builder()
                        .status(http::StatusCode::BAD_REQUEST)
                        .text("Invalid file extension"));
                }
            }

            let result = fs::read(format!("{}/{}", self.directory, uri));
            if let Ok(data) = result {
                return Ok(Response::builder()
                    .status(http::StatusCode::OK)
                    .body(data.as_slice()));
            }

            // TODO: return 404
            Ok(Response::builder()
                .status(http::StatusCode::NOT_FOUND)
                .text("Not found"))
        })
    }
}

pub struct ProxyPath {
    uri: String,
    target: String,
}

impl Path for ProxyPath {
    fn value(&self) -> &str {
        &self.uri
    }

    fn handle(
        &self,
        _request: Request,
        _uri: Cow<str>,
    ) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send>> {
        todo!()
    }
}
