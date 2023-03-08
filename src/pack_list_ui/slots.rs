//---------------------------------------------------------------------------//
// Copyright (c) 2017-2023 Ismael Gutiérrez González. All rights reserved.
//
// This file is part of the Rusted PackFile Manager (RPFM) project,
// which can be found here: https://github.com/Frodo45127/rpfm.
//
// This file is licensed under the MIT license, which can be found here:
// https://github.com/Frodo45127/rpfm/blob/master/LICENSE.
//---------------------------------------------------------------------------//

use qt_core::QBox;
use qt_core::{SlotNoArgs, SlotOfQString};

use std::sync::Arc;

use rpfm_ui_common::clone;

use super::*;

//-------------------------------------------------------------------------------//
//                              Enums & Structs
//-------------------------------------------------------------------------------//

#[derive(Getters)]
#[getset(get = "pub")]
pub struct PackListUISlots {
    filter_line_edit: QBox<SlotOfQString>,
    filter_case_sensitive_button: QBox<SlotNoArgs>,
    filter_trigger: QBox<SlotNoArgs>,
}

//-------------------------------------------------------------------------------//
//                             Implementations
//-------------------------------------------------------------------------------//

impl PackListUISlots {
    pub unsafe fn new(view: &Arc<PackListUI>) -> Self {

        let filter_line_edit = SlotOfQString::new(&view.table_view, clone!(
            view => move |_| {
            view.delayed_updates();
        }));

        let filter_case_sensitive_button = SlotNoArgs::new(&view.table_view, clone!(
            view => move || {
            view.filter_list();
        }));

        let filter_trigger = SlotNoArgs::new(&view.table_view, clone!(
            view => move || {
            view.filter_list();
        }));

        Self {
            filter_line_edit,
            filter_case_sensitive_button,
            filter_trigger,
        }
    }
}
