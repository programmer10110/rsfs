//! A zero cost wrapper around [`std::fs`] that satisfies [`rsfs::FS`].
//!
//! # Example
//!
//! ```
//! use rsfs::*;
//! use rsfs::disk;
//!
//! let fs = disk::fs::FS;
//!
//! let meta = fs.metadata("/");
//! assert!(meta.unwrap().is_dir());
//! ```
//!
//! [`rsfs::FS`]: ../trait.FS.html
//! [`std::fs`]: https://doc.rust-lang.org/std/fs/

use fs;
use std::ffi::OsString;
use std::fs as rs_fs;
use std::io::{Read, Result, Seek, SeekFrom, Write};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};

pub struct Permissions(rs_fs::Permissions);

impl fs::Permissions for Permissions {
    fn readonly(&self) -> bool {
        self.0.readonly()
    }
    fn set_readonly(&mut self, readonly: bool) {
        self.0.set_readonly(readonly)
    }
}

pub struct FileType(rs_fs::FileType);

impl fs::FileType for FileType {
    fn is_dir(&self) -> bool {
        self.0.is_dir()
    }
    fn is_file(&self) -> bool {
        self.0.is_file()
    }
}

/// A single element tuple containing a [`std::fs::Metadata`].
///
/// [`std::fs::Metadata`]: https://doc.rust-lang.org/std/fs/struct.Metadata.html
#[derive(Debug)]
pub struct Metadata(rs_fs::Metadata);

impl fs::Metadata for Metadata {
    fn is_dir(&self) -> bool {
        self.0.is_dir()
    }
    fn is_file(&self) -> bool {
        self.0.is_file()
    }
    fn len(&self) -> u64 {
        self.0.len()
    }
    fn permissions(&self) -> u32 {
        self.0.permissions().mode()
    }
}

/// A single element tuple containing a [`std::fs::OpenOptions`].
///
/// [`std::fs::OpenOptions`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
#[derive(Debug)]
pub struct OpenOptions(rs_fs::OpenOptions);

impl fs::OpenOptions for OpenOptions {
    type File = File;

    fn read(&mut self, read: bool) -> &mut Self {
        self.0.read(read);
        self
    }
    fn write(&mut self, write: bool) -> &mut Self {
        self.0.write(write);
        self
    }
    fn append(&mut self, append: bool) -> &mut Self {
        self.0.append(append);
        self
    }
    fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.0.truncate(truncate);
        self
    }
    fn create(&mut self, create: bool) -> &mut Self {
        self.0.create(create);
        self
    }
    fn create_new(&mut self, create_new: bool) -> &mut Self {
        self.0.create_new(create_new);
        self
    }
    fn mode(&mut self, mode: u32) -> &mut Self {
        self.0.mode(mode);
        self
    }
    fn open<P: AsRef<Path>>(&self, path: P) -> Result<Self::File> {
        self.0.open(path).map(File)
    }
}

/// A single element tuple containing a [`std::fs::File`].
///
/// [`std::fs::File`]: https://doc.rust-lang.org/std/fs/struct.File.html
#[derive(Debug)]
pub struct File(rs_fs::File);

impl fs::File for File {
    type Metadata = Metadata;

    fn metadata(&self) -> Result<Self::Metadata> {
        self.0.metadata().map(Metadata)
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.0.read(buf)
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Seek for File {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.0.seek(pos)
    }
}

/// A single element tuple containing a [`std::fs::DirBuilder`].
///
/// [`std::fs::DirBuilder`]: https://doc.rust-lang.org/std/fs/struct.DirBuilder.html
#[derive(Debug)]
pub struct DirBuilder(rs_fs::DirBuilder);

impl fs::DirBuilder for DirBuilder {
    fn recursive(&mut self, recursive: bool) -> &mut Self {
        self.0.recursive(recursive);
        self
    }
    fn mode(&mut self, mode: u32) -> &mut Self {
        self.0.mode(mode);
        self
    }
    fn create<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.0.create(path)
    }
}

/// A single element tuple containing a [`std::fs::DirEntry`].
///
/// [`std::fs::DirEntry`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html
#[derive(Debug)]
pub struct DirEntry(rs_fs::DirEntry);

impl fs::DirEntry for DirEntry {
    type Metadata = Metadata;

    fn path(&self) -> PathBuf {
        self.0.path()
    }
    fn metadata(&self) -> Result<Self::Metadata> {
        self.0.metadata().map(Metadata)
    }
    fn file_name(&self) -> OsString {
        self.0.file_name()
    }
}

/// A single element tuple containing a [`std::fs::ReadDir`].
///
/// [`std::fs::ReadDir`]: https://doc.rust-lang.org/std/fs/struct.ReadDir.html
#[derive(Debug)]
pub struct ReadDir(rs_fs::ReadDir);

impl Iterator for ReadDir {
    type Item = Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|res_dirent| res_dirent.map(DirEntry))
    }
}

/// An empty struct that satisfies [`rsfs::FS`] by calling [`std::fs`] functions.
///
/// [`rsfs::FS`]: ../trait.FS.html
/// [`std::fs`]: https://doc.rust-lang.org/std/fs/
#[derive(Copy, Clone, Debug)]
pub struct FS;

impl fs::GenFS for FS {
    type Metadata = Metadata;
    type OpenOptions = OpenOptions;
    type DirBuilder = DirBuilder;
    type DirEntry = DirEntry;
    type ReadDir = ReadDir;

    fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<Self::Metadata> {
        rs_fs::metadata(path).map(Metadata)
    }
    fn read_dir<P: AsRef<Path>>(&self, path: P) -> Result<Self::ReadDir> {
        rs_fs::read_dir(path).map(ReadDir)
    }
    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<()> {
        rs_fs::rename(from, to)
    }
    fn remove_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        rs_fs::remove_dir(path)
    }
    fn remove_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        rs_fs::remove_dir_all(path)
    }
    fn remove_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        rs_fs::remove_file(path)
    }
    fn new_openopts(&self) -> Self::OpenOptions {
        OpenOptions(rs_fs::OpenOptions::new())
    }
    fn new_dirbuilder(&self) -> Self::DirBuilder {
        DirBuilder(rs_fs::DirBuilder::new())
    }
}