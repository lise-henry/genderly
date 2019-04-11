// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![allow(missing_docs)]
use error_chain::*;

error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
    }
}