// Copyright (c) 2019-2022 Alibaba Cloud
// Copyright (c) 2019-2022 Ant Group
//
// SPDX-License-Identifier: Apache-2.0
//

use std::{env, fs::{create_dir, set_permissions, Permissions}, io, os::unix::fs::PermissionsExt, path::{Path, PathBuf}, sync::Mutex};
use lazy_static::lazy_static;
use nix::{errno::Errno, sys::stat, unistd::{chown, Gid, Uid}};

use crate::sl;

lazy_static! {
    static ref ROOTLESS_STATE: Mutex<Option<bool>> = Mutex::new(None);

    static ref ROOTLESS_DIR: String = {
        let dir = env::var("XDG_RUNTIME_DIR").unwrap_or_default();
        info!(sl!(), "XDG_RUNTIME_DIR = {}", dir);
        dir
    };
}

/// Set the rootless state.
pub fn set_rootless(rootless: bool) {
    let mut state = ROOTLESS_STATE.lock().unwrap();
    *state = Some(rootless);
}

/// Check if the current environment is rootless.
pub fn is_rootless() -> bool {
    let state = ROOTLESS_STATE.lock().unwrap();
    state.unwrap_or(false)
}

/// Get the directory used for rootless operations.
pub fn get_rootless_dir() -> String {
    ROOTLESS_DIR.clone()
}

/// Create a directory with the specified permissions, inheriting ownership from the parent directory.
pub fn create_dir_all_with_inherit_owner<P: AsRef<Path>>(path: P, perm: u32) -> io::Result<()> {
    let path = path.as_ref();

    if !path.is_absolute() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path must be absolute",
        ));
    }
    info!(sl!(), "fffirst, Creating directory: {}", path.display());
    if path.as_os_str().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path cannot be empty",
        ));
    }
    info!(sl!(), "sssecond, Creating directory: {}", path.display());

    let mut uid = Uid::current();
    let mut gid = Gid::current();
    let mut parents: Vec<PathBuf> = vec![];

    for ancestor in path.ancestors().collect::<Vec<_>>().into_iter().rev() {
        // 倒序收集，保证由浅到深创建
        parents.push(ancestor.to_path_buf());
        info!(sl!(), "Adding parent directory: {}", ancestor.display());
    }

    for p in &parents {
        match stat::stat(p) {
            Ok(st) if stat::SFlag::from_bits_truncate(st.st_mode).contains(stat::SFlag::S_IFDIR) => {
                uid = Uid::from_raw(st.st_uid);
                gid = Gid::from_raw(st.st_gid);
                info!(sl!(), "Directory exists: {}, uid: {}, gid: {}", p.display(), uid, gid);
            }
            Ok(_) => {
                // 如果存在但不是目录，则报错
                info!(sl!(), "Path exists but is not a directory: {}", p.display());
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!("{} exists but is not a directory", p.display()),
                ));
            }
            Err(Errno::ENOENT) => {
                // 目录不存在 -> 创建并 chown
                info!(sl!(), "Directory does not exist, creating: {}", p.display());
                create_dir(p)?;
                info!(sl!(), "Created directory: {}", p.display());
                set_permissions(p, Permissions::from_mode(perm))?;
                chown(p, Some(uid), Some(gid)).map_err(|e| io::Error::from_raw_os_error(e as i32))?;
            }
            Err(e) => {
                info!(sl!(), "Failed to stat path {}: {}", p.display(), e);
                return Err(io::Error::from_raw_os_error(e as i32));
            }
        }
    }
    Ok(())
}

/// Change ownership of a path to that of its parent directory.
pub fn chown_to_parent<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();
    if !path.is_absolute() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "path must be absolute",
        ));
    }
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "no parent directory"))?;

    let st = stat::stat(parent).map_err(|e| io::Error::from_raw_os_error(e as i32))?;
    let uid = Uid::from_raw(st.st_uid);
    let gid = Gid::from_raw(st.st_gid);

    chown(path, Some(uid), Some(gid)).map_err(|e| io::Error::from_raw_os_error(e as i32))
}
