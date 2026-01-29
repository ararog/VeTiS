use crate::server::virtual_host::directory::Directory;

pub struct StaticDirectory<'a> {
    path: &'a str,
}

impl<'a> StaticDirectory<'a> {
    pub fn new(path: &'a str) -> StaticDirectory<'a> {
        StaticDirectory { path }
    }

    pub fn path(&self) -> &'a str {
        self.path
    }

    pub fn set_path(&mut self, path: &'a str) {
        self.path = path;
    }
}

impl<'a> Directory for StaticDirectory<'a> {}
