use std::{collections::HashMap, future::Future, sync::Arc};

use crate::{errors::VetisError, server::virtual_host::VirtualHost, VetisRwLock};

pub mod config;
pub mod conn;
pub mod tls;
pub mod virtual_host;

pub trait Server<RequestBody, ResponseBody> {
    fn port(&self) -> u16;

    fn set_virtual_hosts(
        &mut self,
        virtual_hosts: Arc<VetisRwLock<HashMap<String, Box<dyn VirtualHost>>>>,
    );

    fn start(&mut self) -> impl Future<Output = Result<(), VetisError>>;

    fn stop(&mut self) -> impl Future<Output = Result<(), VetisError>>;
}
