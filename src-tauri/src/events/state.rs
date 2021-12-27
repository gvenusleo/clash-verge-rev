use std::sync::{Arc, Mutex};

use super::emit::ClashInfoPayload;
use crate::{config::VergeConfig, utils::sysopt::SysProxyConfig};

#[derive(Default)]
pub struct ClashInfoState(pub Arc<Mutex<ClashInfoPayload>>);

#[derive(Default)]
pub struct ProfileLock(pub Mutex<bool>);

#[derive(Default)]
pub struct VergeConfLock(pub Arc<Mutex<VergeConfig>>);

#[derive(Default)]
pub struct SomthingState(pub Arc<Mutex<Option<SysProxyConfig>>>);