// Copyright (c) 2019-2022 Alibaba Cloud
// Copyright (c) 2019-2022 Ant Group
//
// SPDX-License-Identifier: Apache-2.0
//

use std::{env, sync::Mutex};
use lazy_static::lazy_static;

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