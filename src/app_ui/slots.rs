//---------------------------------------------------------------------------//
// Copyright (c) 2017-2023 Ismael Gutiérrez González. All rights reserved.
//
// This file is part of the Rusted PackFile Manager (RPFM) project,
// which can be found here: https://github.com/Frodo45127/rpfm.
//
// This file is licensed under the MIT license, which can be found here:
// https://github.com/Frodo45127/rpfm/blob/master/LICENSE.
//---------------------------------------------------------------------------//

use qt_widgets::QMessageBox;

use qt_gui::SlotOfQStandardItem;

use qt_core::QBox;
use qt_core::SlotNoArgs;

use std::sync::Arc;

use rpfm_ui_common::clone;

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
    change_game_selected: QBox<SlotNoArgs>,

    update_pack_list: QBox<SlotOfQStandardItem>,
    update_game_config: QBox<SlotNoArgs>,

    about_qt: QBox<SlotNoArgs>,
    about_runcher: QBox<SlotNoArgs>,

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

        let change_game_selected = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
                if let Err(error) = view.change_game_selected() {
                    show_dialog(view.main_window(), error, false);
                }
            }
        ));

        let update_pack_list = SlotOfQStandardItem::new(&view.main_window, clone!(
            view => move |item| {
            if item.column() == 0 {
                if let Some(ref mut game_config) = *view.game_config().write().unwrap() {
                    let mod_id = item.data_1a(21).to_string().to_std_string();

                    // Update the mod's status.
                    if let Some(modd) = game_config.mods_mut().get_mut(&mod_id) {
                        modd.set_enabled(item.check_state() == CheckState::Checked);
                    }

                    // Reload the pack view.
                    if let Err(error) = view.pack_list_ui().load(game_config) {
                        show_dialog(view.main_window(), error, false);
                    }
                }
            }
        }));

        let update_game_config = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
                let game_info = view.game_selected().read().unwrap();
                if let Some(ref mut game_config) = *view.game_config().write().unwrap() {
                    if let Err(error) = game_config.save(&game_info) {
                        show_dialog(view.main_window(), error, false);
                    }
                }
            }
        ));

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

                let selection = view.mod_list_selection();
                if selection.len() != 1 {
                    return;
                }

                if !selection[0].data_1a(40).to_bool() {
                    return;
                }

                let cat_to_delete = &selection[0];
                let mods_to_reassign = (0..view.mod_list_ui().model().row_count_1a(cat_to_delete))
                    .map(|index| cat_to_delete.child(index, 0).data_1a(21).to_string().to_std_string())
                    .collect::<Vec<_>>();

                if let Some(ref mut game_config) = *view.game_config().write().unwrap() {
                    game_config.mods_mut()
                        .iter_mut()
                        .for_each(|(id, modd)| if mods_to_reassign.contains(id) {
                            modd.set_category(None);
                        });
                }

                // Find the unassigned category.
                let mut unassigned_item = None;
                let unassigned = QString::from_std_str("Unassigned");
                for index in 0..view.mod_list_ui().model().row_count_0a() {
                    let item = view.mod_list_ui().model().item_1a(index);
                    if !item.is_null() && item.text().compare_q_string(&unassigned) == 0 {
                        unassigned_item = Some(item);
                        break;
                    }
                }

                if let Some(unassigned_item) = unassigned_item {
                    let cat_item = view.mod_list_ui().model().item_from_index(cat_to_delete);
                    for index in view.mod_list_ui().model().row_count_1a(cat_to_delete)..0 {
                        let index = index - 1;
                        let taken = cat_item.take_row(index).into_ptr();
                        unassigned_item.append_row_q_list_of_q_standard_item(taken.as_ref().unwrap());
                    }
                }

                view.mod_list_ui().model().remove_row_1a(cat_to_delete.row());
            }
        ));

        let mod_list_context_menu_open = SlotNoArgs::new(&view.main_window, clone!(
            view => move || {
                view.mod_list_ui().categories_send_to_menu().clear();
                let categories = view.mod_list_ui().categories();
                for category in &categories {

                    let item = view.mod_list_ui().category_item(category);
                    if let Some(item) = item {
                        let action = view.mod_list_ui().categories_send_to_menu().add_action_q_string(&QString::from_std_str(category));
                        let slot = SlotNoArgs::new(view.mod_list_ui().categories_send_to_menu(), clone!(
                            category,
                            view => move || {
                                let selection = view.mod_list_selection();
                                if selection.len() != 1 {
                                    return;
                                }

                                if selection[0].data_1a(40).to_bool() {
                                    return;
                                }
                                let mod_item = &selection[0];
                                let current_cat = mod_item.parent();
                                let mod_id = mod_item.data_1a(21).to_string().to_std_string();
                                let taken = view.mod_list_ui().model().item_from_index(&current_cat).take_row(mod_item.row()).into_ptr();
                                item.append_row_q_list_of_q_standard_item(taken.as_ref().unwrap());

                                if let Some(ref mut game_config) = *view.game_config().write().unwrap() {
                                    if let Some(ref mut modd) = game_config.mods_mut().get_mut(&mod_id) {
                                        modd.set_category(Some(category.to_string()));
                                    }

                                    let game_info = view.game_selected().read().unwrap();
                                    if let Err(error) = game_config.save(&game_info) {
                                        show_dialog(view.main_window(), error, false);
                                    }
                                }
                            }
                        ));

                        action.triggered().connect(&slot);
                    }
                }
            }
        ));

        Self {
            launch_game,
            open_settings,
            change_game_selected,

            update_pack_list,
            update_game_config,

            about_qt,
            about_runcher,

            load_profile,
            save_profile,

            category_delete,
            mod_list_context_menu_open,
        }
    }
}
