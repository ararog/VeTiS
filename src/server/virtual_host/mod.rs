pub mod directory;

pub trait VirtualHost {
    fn hostname(&self) -> String;

    fn is_secure(&self) -> bool;

    fn set_directories<D>(&mut self, directories: Vec<D>)
    where
        D: directory::Directory;

    fn directories<D>(&self) -> Vec<&D>
    where
        D: directory::Directory;
}
