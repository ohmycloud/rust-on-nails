use db::User;
use dioxus::prelude::*;

use crate::{layout::Layout, render};

pub fn index(users: Vec<User>) -> String {
    let page = rsx! {
        Layout {
            title: "User Table",
            table {
                thead {
                    tr {
                        th { "ID" }
                        th { "Email" }
                    }
                }
                tbody {
                    for user in users {
                        tr {
                            td {
                                strong { "{user.id}" }
                            }
                            td {
                                "{user.email}"
                            }
                        }
                    }
                }
            }
        }
    };
    render(page)
}
