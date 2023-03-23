//---------------------------------------------------------------------------//
// Copyright (c) 2017-2023 Ismael Gutiérrez González. All rights reserved.
//
// This file is part of the Rusted Launcher (Runcher) project,
// which can be found here: https://github.com/Frodo45127/runcher.
//
// This file is licensed under the MIT license, which can be found here:
// https://github.com/Frodo45127/runcher/blob/master/LICENSE.
//---------------------------------------------------------------------------//

use qt_widgets::QMessageBox;

use qt_gui::SlotOfQStandardItem;

use qt_core::QBox;
use qt_core::SlotNoArgs;

use std::sync::Arc;

use rpfm_ui_common::clone;

use crate::mod_list_ui::VALUE_MOD_ID;
use crate::VERSION;
use crate::VERSION_SUBTITLE;

use super::*;

//-------------------------------------------------------------------------------//
//                              Enums & Structs
//-------------------------------------------------------------------------------//

#[derive(Getters)]
#[getset(get = "pub")]
pub struct AppUISlots {
    launch_game: QBox<SlotNoArgs>,
    open_settings: QBox<SlotNoArgs>,
    open_folders_submenu: QBox<SlotNoArgs>,
    open_game_root_folder: QBox<SlotNoArgs>,
    open_game_data_folder: QBox<SlotNoArgs>,
    open_game_content_folder: QBox<SlotNoArgs>,
    open_runcher_config_folder: QBox<SlotNoArgs>,
    open_runcher_error_folder: QBox<SlotNoArgs>,
    change_game_selected: QBox<SlotNoArgs>,

    update_pack_list: QBox<SlotOfQStandardItem>,

    about_qt: QBox<SlotNoArgs>,
    about_runcher: QBox<SlotNoArgs>,
    check_updates: QBox<SlotNoArgs>,

    copy_load_order: QBox<SlotNoArgs>,
    paste_load_order: QBox<SlotNoArgs>,
    reload: QBox<SlotNoArgs>,
    load_profile: QBox<SlotNoArgs>,
    save_profile: QBox<SlotNoArgs>,

    category_delete: QBox<SlotNoArgs>,
    mod_list_context_menu_open: QBox<SlotNoArgs>,
}

//-------------------------------------------------------------------------------//
//                             Implementations
//-------------------------------------------------------------------------------//

impl AppUISlots {
    pub unsafe fn new(view: &Arc<AppUI>) -> Self {

        let launch_game = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
                if let Err(error) = view.launch_game() {
                    show_dialog(view.main_window(), error, false);
                }
            }
        ));

        let open_settings = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
            view.open_settings();
        }));

        let open_folders_submenu = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
            view.actions_ui().folders_button().show_menu();
        }));

        let open_game_root_folder = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
            let game = view.game_selected().read().unwrap();
            let game_path = setting_string(game.game_key_name());
            if !game_path.is_empty() {
                let _ = open::that(game_path);
            } else {
                show_dialog(view.main_window(), "Runcher cannot open that folder (maybe it doesn't exists/is misconfigured?).", false);
            }
        }));

        let open_game_data_folder = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
            let game = view.game_selected().read().unwrap();
            if let Ok(game_path) = game.data_path(&setting_path(game.game_key_name())) {
                let _ = open::that(game_path);
            } else {
                show_dialog(view.main_window(), "Runcher cannot open that folder (maybe it doesn't exists/is misconfigured?).", false);
            }
        }));

        let open_game_content_folder = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
            let game = view.game_selected().read().unwrap();
            if let Ok(game_path) = game.content_path(&setting_path(game.game_key_name())) {
                let _ = open::that(game_path);
            } else {
                show_dialog(view.main_window(), "Runcher cannot open that folder (maybe it doesn't exists/is misconfigured?).", false);
            }
        }));

        let open_runcher_config_folder = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
            if let Ok(path) = config_path() {
                let _ = open::that(path);
            } else {
                show_dialog(view.main_window(), "Runcher cannot open that folder (maybe it doesn't exists/is misconfigured?).", false);
            }
        }));

        let open_runcher_error_folder = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
            if let Ok(path) = error_path() {
                let _ = open::that(path);
            } else {
                show_dialog(view.main_window(), "Runcher cannot open that folder (maybe it doesn't exists/is misconfigured?).", false);
            }
        }));

        let change_game_selected = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
                if let Err(error) = view.change_game_selected(false) {
                    show_dialog(view.main_window(), error, false);
                }
            }
        ));

        let update_pack_list = SlotOfQStandardItem::new(&view.main_window, clone!(
            view => move |item| {
            if item.column() == 0 {
                if let Some(ref mut game_config) = *view.game_config().write().unwrap() {
                    let mod_id = item.data_1a(VALUE_MOD_ID).to_string().to_std_string();

                    // Update the mod's status.
                    if let Some(modd) = game_config.mods_mut().get_mut(&mod_id) {
                        modd.set_enabled(item.check_state() == CheckState::Checked);
                    }

                    // Reload the pack view.
                    let game_info = view.game_selected().read().unwrap();
                    let game_path = setting_path(game_info.game_key_name());
                    if let Err(error) = view.pack_list_ui().load(game_config, &game_info, &game_path) {
                        show_dialog(view.main_window(), error, false);
                    }

                    let game_info = view.game_selected().read().unwrap();
                    if let Err(error) = game_config.save(&game_info) {
                        show_dialog(view.main_window(), error, false);
                    }
                }
            }
        }));

        let about_qt = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
                QMessageBox::about_qt_1a(&view.main_window);
            }
        ));

        let about_runcher = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
                QMessageBox::about(
                    &view.main_window,
                    &qtr("about_runcher"),

                    // NOTE: This one is hardcoded, because I don't want people attributing themselves the program in the translations.
                    &QString::from_std_str(format!(
                        "<table>
                            <tr>
                                <td><h2><b>Runcher</b></h2></td>
                            </tr>
                            <tr>
                                <td>{} {} Patch</td>
                            </tr>
                        </table>

                        <p><b>Rusted Launcher</b> (a.k.a. Runcher) is a mod manager/launcher for modern Total War Games.</p>
                        <p>This program is <b>open-source</b>, under MIT License. You can always get the last version (or collaborate) here:</p>
                        <a href=\"https://github.com/Frodo45127/runcher\">https://github.com/Frodo45127/runcher</a>
                        <p>This program is also <b>free</b> (if you paid for this, sorry, but you got scammed), but if you want to help with money, here is <b>RPFM's Patreon</b>:</p>
                        <a href=\"https://www.patreon.com/RPFM\">https://www.patreon.com/RPFM</a>

                        <h3>Credits</h3>
                        <ul style=\"list-style-type: disc\">
                            <li>Created and Programmed by: <b>Frodo45127</b>.</li>
                        </ul>
                        ", &VERSION, &VERSION_SUBTITLE)
                    )
                );
            }
        ));

        let check_updates = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
                view.check_updates(true);
            }
        ));

        let copy_load_order = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
                if let Some(ref game_config) = *view.game_config().read().unwrap() {
                    view.toggle_main_window(false);

                    let receiver = CENTRAL_COMMAND.send_background(Command::GetStringFromLoadOrder(game_config.clone()));
                    let response = CENTRAL_COMMAND.recv_try(&receiver);
                    match response {
                        Response::String(response) => {
                            if let Err(error) = view.load_order_string_dialog(Some(response)) {
                                show_dialog(view.main_window(), error, false)
                            }
                        }
                        _ => panic!("{THREADS_COMMUNICATION_ERROR}{response:?}"),
                    }

                    view.toggle_main_window(true);
                }
            }
        ));

        let paste_load_order = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
                match view.load_order_string_dialog(None) {
                    Ok(string) => if let Some(string) = string {
                        view.toggle_main_window(false);

                        let receiver = CENTRAL_COMMAND.send_background(Command::GetLoadOrderFromString(string));
                        let response = CENTRAL_COMMAND.recv_try(&receiver);
                        match response {
                            Response::VecShareableMods(response) => {
                                if let Err(error) = view.load_order_from_shareable_mod_list(&response) {
                                    show_dialog(view.main_window(), error, false);
                                }
                            }
                            _ => panic!("{THREADS_COMMUNICATION_ERROR}{response:?}"),
                        }

                        view.toggle_main_window(true);
                    }
                    Err(error) => show_dialog(view.main_window(), error, false),
                }
            }
        ));

        let reload = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {

                // We just re-use the game selected logic
                if let Err(error) = view.change_game_selected(true) {
                    show_dialog(view.main_window(), error, false);
                }
            }
        ));

        let load_profile = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
                if let Err(error) = view.load_profile() {
                    show_dialog(view.main_window(), error, false);
                }
            }
        ));

        let save_profile = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
                if let Err(error) = view.save_profile() {
                    show_dialog(view.main_window(), error, false);
                }
            }
        ));

        let category_delete = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
                if let Err(error) = view.delete_category() {
                    show_dialog(view.main_window(), error, false);
                }
            }
        ));

        let mod_list_context_menu_open = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
                AppUI::generate_move_to_category_submenu(&view);
            }
        ));

        Self {
            launch_game,
            open_settings,
            open_folders_submenu,
            open_game_root_folder,
            open_game_data_folder,
            open_game_content_folder,
            open_runcher_config_folder,
            open_runcher_error_folder,
            change_game_selected,

            update_pack_list,

            about_qt,
            about_runcher,
            check_updates,

            copy_load_order,
            paste_load_order,
            reload,

            load_profile,
            save_profile,

            category_delete,
            mod_list_context_menu_open,
        }
    }
}
