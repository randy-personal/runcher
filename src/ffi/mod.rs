//---------------------------------------------------------------------------//
// Copyright (c) 2017-2023 Ismael Gutiérrez González. All rights reserved.
//
// This file is part of the Rusted Launcher (Runcher) project,
// which can be found here: https://github.com/Frodo45127/runcher.
//
// This file is licensed under the MIT license, which can be found here:
// https://github.com/Frodo45127/runcher/blob/master/LICENSE.
//---------------------------------------------------------------------------//

use qt_widgets::QMainWindow;
use qt_widgets::QWidget;

use qt_core::QBox;
use qt_core::QObject;
use qt_core::QPtr;
use qt_core::QRegExp;
use qt_core::QSortFilterProxyModel;
use qt_core::Signal;

use cpp_core::Ptr;

//---------------------------------------------------------------------------//
// Custom delegates stuff.
//---------------------------------------------------------------------------//

// This function setup the special filter used for the PackFile Contents `TreeView`.
extern "C" { fn mod_list_filter(parent: *mut QObject) -> *mut QSortFilterProxyModel; }
pub fn mod_list_filter_safe(parent: QPtr<QObject>) ->  QBox<QSortFilterProxyModel> {
    unsafe { QBox::from_raw(mod_list_filter(parent.as_mut_raw_ptr())) }
}

// This function triggers the special filter used for the PackFile Contents `TreeView`. It has to be triggered here to work properly.
extern "C" { fn mod_list_trigger_filter(filter: *const QSortFilterProxyModel, pattern: *mut QRegExp); }
pub fn mod_list_trigger_filter_safe(filter: &QSortFilterProxyModel, pattern: &Ptr<QRegExp>) {
    unsafe { mod_list_trigger_filter(filter, pattern.as_mut_raw_ptr()); }
}

// This function allow us to create a custom window.
extern "C" { fn launcher_window() -> *mut QMainWindow; }
pub fn launcher_window_safe() -> QBox<QMainWindow> {
    unsafe { QBox::from_raw(launcher_window()) }
}

pub fn main_window_drop_pack_signal(widget: QPtr<QWidget>) -> Signal<(*const ::qt_core::QStringList,)> {
    unsafe {
        Signal::new(
            ::cpp_core::Ref::from_raw(widget.as_raw_ptr()).expect("attempted to construct a null Ref"),
            ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"2openPack(QStringList const &)\0",
            ),
        )
    }
}

extern "C" { fn html_item_delegate(view: *mut QObject, column: i32); }
pub fn html_item_delegate_safe(view: &Ptr<QObject>, column: i32) {
    unsafe { html_item_delegate(view.as_mut_raw_ptr(), column) }
}
