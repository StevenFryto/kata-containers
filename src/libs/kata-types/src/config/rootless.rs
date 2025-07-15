// Copyright (c) 2019-2021 Ant Financial
// Copyright (c) 2019-2021 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

use std::{env, sync::Mutex};

use lazy_static::lazy_static;

lazy_static! {
    static ref ROOTLESS_STATE: Mutex<Option<bool>> = Mutex::new(None);
    
    static ref ROOTLESS_DIR: String = {
        let dir = env::var("XDG_RUNTIME_DIR").unwrap_or_default();
        dir
    };
}

/// Set the rootless state.
pub fn set_rootless(rootless: bool) {
    let mut state = ROOTLESS_STATE.lock().unwrap();
    *state = Some(rootless);
}

/// Get the rootless state.
pub fn is_rootless() -> bool {
    let state = ROOTLESS_STATE.lock().unwrap();
    match *state {
        Some(rootless) => rootless,
        None => {
            // If not set, check if the XDG_RUNTIME_DIR is set and not empty
            !ROOTLESS_DIR.is_empty()
        }
    }
}

/// Get the rootless directory path.
pub fn get_rootless_dir() -> String {
    ROOTLESS_DIR.clone()
}