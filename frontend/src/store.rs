use common::User;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct AlertInput {
    pub show_alert: bool,
    pub alert_message: String,
}

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "local", storage_tab_sync)]
pub struct Store {
    pub users: Vec<User>,
    pub loading: bool,
    pub alert_input: AlertInput,
}

pub fn set_user(user: User, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.users.insert(0, user);
    })
}

pub fn delete_user(user: User, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.users.retain(|u| u.id != user.id);
    })
}

pub fn set_loading(loading: bool, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.loading = loading;
    })
}

pub fn set_show_alert(message: String, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input = AlertInput {
            alert_message: message,
            show_alert: true,
        };
    })
}

pub fn set_hide_alert(dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input.show_alert = false;
    })
}