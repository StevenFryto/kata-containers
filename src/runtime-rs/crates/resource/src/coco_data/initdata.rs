// Copyright (c) 2025 Ant Group
//
// SPDX-License-Identifier: Apache-2.0
//

use hypervisor::BlockConfig;
use kata_types::rootless::{get_rootless_dir, is_rootless};

/// The path /run/kata-containers/shared/initdata, combined with the sandbox ID,
/// will form the directory for storing the initdata image.
/// Path::new(KATA_SHARED_INIT_DATA_PATH).join(SID)
pub const DEFAULT_KATA_SHARED_INIT_DATA_PATH: &str = "/run/kata-containers/shared/initdata";

/// kata initdata image
pub const KATA_INIT_DATA_IMAGE: &str = "initdata.image";

/// InitDataConfig which is a tuple of Block Device Config and its digest of the encoded
/// string included in the disk. And, both of them will come up at the same time.
#[derive(Clone, Debug)]
pub struct InitDataConfig(pub BlockConfig, pub String);

pub fn kata_shared_init_data_path() -> String {
    if is_rootless() {
        let rootless_dir = get_rootless_dir();
        let a = [&rootless_dir, DEFAULT_KATA_SHARED_INIT_DATA_PATH].join("/");
        a
    } else {
        DEFAULT_KATA_SHARED_INIT_DATA_PATH.to_string()
    }
}
