// Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Provides the VersionMap that deals with the microvm state versions.

use std::collections::HashMap;

use crate::device_manager::persist::DeviceStates;
use devices::virtio::block::persist::BlockState;

use lazy_static::lazy_static;
use versionize::VersionMap;
use versionize::Versionize;

lazy_static! {
    // Note: until we have a better design, this needs to be updated when the version changes.
    /// Static instance used for handling microVM state versions.
    pub static ref VERSION_MAP: VersionMap = {
        let mut version_map = VersionMap::new();
        version_map.new_version().set_type_version(DeviceStates::type_id(), 2);
        version_map.new_version().set_type_version(BlockState::type_id(), 2);
        version_map
    };

    /// Static instance used for creating a 1:1 mapping between Firecracker release version
    /// and snapshot data format version.
    pub static ref FC_VERSION_TO_SNAP_VERSION: HashMap<String, u16> = {
        let mut mapping = HashMap::new();
        mapping.insert(String::from("0.23.0"), 1);
        mapping.insert(String::from("0.24.0"), 2);
        mapping.insert(String::from("0.25.0"), 3);

        mapping
    };
}
