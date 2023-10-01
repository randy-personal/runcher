//---------------------------------------------------------------------------//
// Copyright (c) 2017-2023 Ismael Gutiérrez González. All rights reserved.
//
// This file is part of the Rusted Launcher (Runcher) project,
// which can be found here: https://github.com/Frodo45127/runcher.
//
// This file is licensed under the MIT license, which can be found here:
// https://github.com/Frodo45127/runcher/blob/master/LICENSE.
//---------------------------------------------------------------------------//

use qt_widgets::QCheckBox;
use qt_widgets::QComboBox;
use qt_widgets::QDialog;
use qt_widgets::QDialogButtonBox;
use qt_widgets::q_dialog_button_box::StandardButton;
use qt_widgets::{QFileDialog, q_file_dialog::{FileMode, Option as QFileDialogOption}};
use qt_widgets::QGridLayout;
use qt_widgets::QGroupBox;
use qt_widgets::QLabel;
use qt_widgets::QLineEdit;
use qt_widgets::QMainWindow;
use qt_widgets::QPushButton;
use qt_widgets::QToolButton;

use qt_gui::QIcon;

use qt_core::QBox;
use qt_core::QCoreApplication;
use qt_core::QFlags;
use qt_core::QPtr;
use qt_core::QString;

use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use getset::*;
use regex::Regex;

use std::collections::BTreeMap;
use std::fs::{DirBuilder, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use rpfm_lib::games::{GameInfo, supported_games::{KEY_ARENA, KEY_SHOGUN_2, KEY_WARHAMMER_3}};

use rpfm_ui_common::locale::*;
use rpfm_ui_common::settings::*;
use rpfm_ui_common::utils::*;

use crate::SUPPORTED_GAMES;
use crate::updater::*;

use self::slots::SettingsUISlots;

mod slots;

const VIEW_DEBUG: &str = "ui_templates/settings_dialog.ui";
const VIEW_RELEASE: &str = "ui/settings_dialog.ui";

//-------------------------------------------------------------------------------//
//                              Enums & Structs
//-------------------------------------------------------------------------------//

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct SettingsUI {
    dialog: QPtr<QDialog>,

    paths_games_line_edits: BTreeMap<String, QBox<QLineEdit>>,
    paths_games_buttons: BTreeMap<String, QBox<QToolButton>>,

    steam_user_id_line_edit: QPtr<QLineEdit>,
    steam_api_key_line_edit: QPtr<QLineEdit>,

    language_combobox: QPtr<QComboBox>,
    default_game_combobox: QPtr<QComboBox>,
    update_chanel_combobox: QPtr<QComboBox>,
    check_updates_on_start_checkbox: QPtr<QCheckBox>,
    check_schema_updates_on_start_checkbox: QPtr<QCheckBox>,
    dark_mode_checkbox: QPtr<QCheckBox>,

    restore_default_button: QPtr<QPushButton>,
    accept_button: QPtr<QPushButton>,
    cancel_button: QPtr<QPushButton>,
}

//-------------------------------------------------------------------------------//
//                             Implementations
//-------------------------------------------------------------------------------//

impl SettingsUI {

    /// This function creates a ***Settings*** dialog, execute it, and returns a new `Settings`, or `None` if you close/cancel the dialog.
    pub unsafe fn new(main_window: &QBox<QMainWindow>) -> Result<bool> {
        let settings_ui = Self::new_with_parent(main_window)?;
        let slots = SettingsUISlots::new(&settings_ui, main_window.static_upcast());
        settings_ui.set_connections(&slots);

        // If load fails due to missing locale folder, show the error and cancel the settings edition.
        settings_ui.load()?;

        if settings_ui.dialog.exec() == 1 {
            settings_ui.save()?;
            settings_ui.dialog.delete_later();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub unsafe fn new_with_parent(main_window: &QBox<QMainWindow>) -> Result<Rc<Self>> {

        // Load the UI Template.
        let template_path = if cfg!(debug_assertions) { VIEW_DEBUG } else { VIEW_RELEASE };
        let main_widget = load_template(main_window, template_path)?;
        let dialog: QPtr<QDialog> = main_widget.static_downcast();

        let paths_groupbox: QPtr<QGroupBox> = find_widget(&main_widget.static_upcast(), "paths_groupbox")?;
        let language_label: QPtr<QLabel> = find_widget(&main_widget.static_upcast(), "language_label")?;
        let default_game_label: QPtr<QLabel> = find_widget(&main_widget.static_upcast(), "default_game_label")?;
        let update_chanel_label: QPtr<QLabel> = find_widget(&main_widget.static_upcast(), "update_chanel_label")?;
        let steam_user_id_label: QPtr<QLabel> = find_widget(&main_widget.static_upcast(), "steam_user_id_label")?;
        let steam_api_key_label: QPtr<QLabel> = find_widget(&main_widget.static_upcast(), "steam_api_key_label")?;
        let check_updates_on_start_label: QPtr<QLabel> = find_widget(&main_widget.static_upcast(), "check_updates_on_start_label")?;
        let check_schema_updates_on_start_label: QPtr<QLabel> = find_widget(&main_widget.static_upcast(), "check_schema_updates_on_start_label")?;
        let dark_mode_label: QPtr<QLabel> = find_widget(&main_widget.static_upcast(), "dark_mode_label")?;
        let language_combobox: QPtr<QComboBox> = find_widget(&main_widget.static_upcast(), "language_combobox")?;
        let default_game_combobox: QPtr<QComboBox> = find_widget(&main_widget.static_upcast(), "default_game_combobox")?;
        let update_chanel_combobox: QPtr<QComboBox> = find_widget(&main_widget.static_upcast(), "update_chanel_combobox")?;
        let steam_user_id_line_edit: QPtr<QLineEdit> = find_widget(&main_widget.static_upcast(), "steam_user_id_line_edit")?;
        let steam_api_key_line_edit: QPtr<QLineEdit> = find_widget(&main_widget.static_upcast(), "steam_api_key_line_edit")?;
        let check_updates_on_start_checkbox: QPtr<QCheckBox> = find_widget(&main_widget.static_upcast(), "check_updates_on_start_checkbox")?;
        let check_schema_updates_on_start_checkbox: QPtr<QCheckBox> = find_widget(&main_widget.static_upcast(), "check_schema_updates_on_start_checkbox")?;
        let dark_mode_checkbox: QPtr<QCheckBox> = find_widget(&main_widget.static_upcast(), "dark_mode_checkbox")?;
        let paths_layout: QPtr<QGridLayout> = paths_groupbox.layout().static_downcast();
        update_chanel_combobox.add_item_q_string(&QString::from_std_str(STABLE));
        update_chanel_combobox.add_item_q_string(&QString::from_std_str(BETA));

        paths_groupbox.set_title(&qtr("game_paths"));
        language_label.set_text(&qtr("language"));
        default_game_label.set_text(&qtr("default_game"));
        update_chanel_label.set_text(&qtr("update_channel"));
        steam_user_id_label.set_text(&qtr("steam_user_id"));
        steam_api_key_label.set_text(&qtr("steam_api_key"));
        check_updates_on_start_label.set_text(&qtr("check_updates_on_start"));
        check_schema_updates_on_start_label.set_text(&qtr("check_schema_updates_on_start"));
        dark_mode_label.set_text(&qtr("dark_mode"));

        // We automatically add a Label/LineEdit/Button for each game we support.
        let mut paths_games_line_edits = BTreeMap::new();
        let mut paths_games_buttons = BTreeMap::new();

        for (index, game) in SUPPORTED_GAMES.games_sorted().iter().enumerate() {
            if game.key() != KEY_ARENA {
                let game_key = game.key();
                let game_label = QLabel::from_q_string_q_widget(&QString::from_std_str(game.display_name()), &paths_groupbox);
                let game_line_edit = QLineEdit::from_q_widget(&paths_groupbox);
                let game_button = QToolButton::new_1a(&paths_groupbox);
                game_line_edit.set_placeholder_text(&qtre("settings_game_line_ph", &[game.display_name()]));
                game_button.set_icon(&QIcon::from_theme_1a(&QString::from_std_str("folder")));

                paths_layout.add_widget_5a(&game_label, index as i32, 0, 1, 1);
                paths_layout.add_widget_5a(&game_line_edit, index as i32, 1, 1, 1);
                paths_layout.add_widget_5a(&game_button, index as i32, 2, 1, 1);

                // Add the LineEdit and Button to the list.
                paths_games_line_edits.insert(game_key.to_owned(), game_line_edit);
                paths_games_buttons.insert(game_key.to_owned(), game_button);

                // Add the game to the default game combo.
                default_game_combobox.add_item_q_string(&QString::from_std_str(game.display_name()));
            }
        }

        if let Ok(locales) = Locale::get_available_locales() {
            for (language, _) in locales {
                language_combobox.add_item_q_string(&QString::from_std_str(language));
            }
        }

        let button_box: QPtr<QDialogButtonBox> = find_widget(&main_widget.static_upcast(), "button_box")?;
        let restore_default_button: QPtr<QPushButton> = button_box.button(StandardButton::RestoreDefaults);
        let accept_button: QPtr<QPushButton> = button_box.button(StandardButton::Ok);
        let cancel_button: QPtr<QPushButton> = button_box.button(StandardButton::Cancel);

        Ok(Rc::new(Self {
            dialog,
            paths_games_line_edits,
            paths_games_buttons,
            steam_user_id_line_edit,
            steam_api_key_line_edit,
            language_combobox,
            default_game_combobox,
            update_chanel_combobox,
            check_updates_on_start_checkbox,
            check_schema_updates_on_start_checkbox,
            dark_mode_checkbox,

            restore_default_button,
            accept_button,
            cancel_button,
        }))
    }


    /// This function loads the data from the provided `Settings` into our `SettingsUI`.
    pub unsafe fn load(&self) -> Result<()> {
        let q_settings = settings();

        // Load the Game Paths, if they exists.
        for (key, path) in self.paths_games_line_edits.iter() {
            let stored_path = setting_string_from_q_setting(&q_settings, key);
            if !stored_path.is_empty() {
                path.set_text(&QString::from_std_str(stored_path));
            }
        }

        // Get the default game.
        let default_game = setting_string_from_q_setting(&q_settings, "default_game");
        for (index, game) in SUPPORTED_GAMES.games_sorted().iter().enumerate() {
            if game.key() == default_game {
                self.default_game_combobox.set_current_index(index as i32);
                break;
            }
        }

        let language_selected = setting_string("language");
        let language_selected_split = language_selected.split('_').collect::<Vec<&str>>()[0];
        for (index, (language,_)) in Locale::get_available_locales()?.iter().enumerate() {
            if *language == language_selected_split {
                self.language_combobox.set_current_index(index as i32);
                break;
            }
        }

        for (index, update_channel_name) in [UpdateChannel::Stable, UpdateChannel::Beta].iter().enumerate() {
            if update_channel_name == &update_channel() {
                self.update_chanel_combobox.set_current_index(index as i32);
                break;
            }
        }

        self.steam_user_id_line_edit().set_text(&QString::from_std_str(setting_string_from_q_setting(&q_settings, "steam_user_id")));
        self.steam_api_key_line_edit().set_text(&QString::from_std_str(setting_string_from_q_setting(&q_settings, "steam_api_key")));
        self.dark_mode_checkbox().set_checked(setting_bool_from_q_setting(&q_settings, "dark_mode"));
        self.check_updates_on_start_checkbox().set_checked(setting_bool_from_q_setting(&q_settings, "check_updates_on_start"));
        self.check_schema_updates_on_start_checkbox().set_checked(setting_bool_from_q_setting(&q_settings, "check_schema_updates_on_start"));

        Ok(())
    }

    pub unsafe fn save(&self) -> Result<()> {
        let q_settings = settings();

        // For each entry, we check if it's a valid directory and save it into Settings.
        for (key, line_edit) in self.paths_games_line_edits.iter() {
            set_setting_string_to_q_setting(&q_settings, key, &line_edit.text().to_std_string());
        }

        // We get his game's folder, depending on the selected game.
        let mut game = self.default_game_combobox.current_text().to_std_string();
        if let Some(index) = game.find('&') { game.remove(index); }
        game = game.replace(' ', "_").to_lowercase();
        set_setting_string_to_q_setting(&q_settings, "default_game", &game);

        // We need to store the full locale filename, not just the visible name!
        let mut language = self.language_combobox.current_text().to_std_string();
        if let Some(index) = language.find('&') { language.remove(index); }
        if let Some((_, locale)) = Locale::get_available_locales()?.iter().find(|(x, _)| &language == x) {
            let file_name = format!("{}_{}", language, locale.language);
            set_setting_string_to_q_setting(&q_settings, "language", &file_name);
        }

        set_setting_string_to_q_setting(&q_settings, "update_channel", &self.update_chanel_combobox.current_text().to_std_string());
        set_setting_string_to_q_setting(&q_settings, "steam_user_id", &self.steam_user_id_line_edit().text().to_std_string());
        set_setting_string_to_q_setting(&q_settings, "steam_api_key", &self.steam_api_key_line_edit().text().to_std_string());
        set_setting_bool_to_q_setting(&q_settings, "dark_mode", self.dark_mode_checkbox().is_checked());
        set_setting_bool_to_q_setting(&q_settings, "check_updates_on_start", self.check_updates_on_start_checkbox().is_checked());
        set_setting_bool_to_q_setting(&q_settings, "check_schema_updates_on_start", self.check_schema_updates_on_start_checkbox().is_checked());

        // Save the settings.
        q_settings.sync();

        Ok(())
    }

    pub unsafe fn set_connections(&self, slots: &SettingsUISlots) {
        for (key, button) in self.paths_games_buttons.iter() {
            button.released().connect(&slots.select_game_paths()[key]);
        }

        self.restore_default_button.released().connect(slots.restore_default());
        self.accept_button.released().connect(self.dialog.slot_accept());
        self.cancel_button.released().connect(self.dialog.slot_close());
    }

    unsafe fn update_entry_path(&self, game: &str) {

        // We check if we have a game or not. If we have it, update the `LineEdit` for that game.
        // If we don't, update the `LineEdit` for `MyMod`s path.
        let line_edit = match self.paths_games_line_edits.get(game) {
            Some(line_edit) => line_edit,
            None => return,
        };

        // Create the `FileDialog` and configure it.
        let title = qtr("settings_select_folder");
        let file_dialog = QFileDialog::from_q_widget_q_string(
            &self.dialog,
            &title,
        );

        file_dialog.set_file_mode(FileMode::Directory);
        file_dialog.set_options(QFlags::from(QFileDialogOption::ShowDirsOnly));

        // Get the old Path, if exists.
        let old_path = line_edit.text().to_std_string();

        // If said path is not empty, and is a dir, set it as the initial directory.
        if !old_path.is_empty() && Path::new(&old_path).is_dir() {
            file_dialog.set_directory_q_string(&line_edit.text());
        }

        // Run it and expect a response (1 => Accept, 0 => Cancel).
        if file_dialog.exec() == 1 {

            // Get the path of the selected file.
            let selected_files = file_dialog.selected_files();
            let path = selected_files.at(0);

            // Add the Path to the LineEdit.
            line_edit.set_text(path);
        }
    }
}

//-------------------------------------------------------------------------------//
//                         Setting-related functions
//-------------------------------------------------------------------------------//

pub unsafe fn init_settings(main_window: &QPtr<QMainWindow>) {
    let q_settings = settings();

    set_setting_if_new_q_byte_array(&q_settings, "originalGeometry", main_window.save_geometry().as_ref());
    set_setting_if_new_q_byte_array(&q_settings, "originalWindowState", main_window.save_state_0a().as_ref());

    let path = PathBuf::from("C:/Program Files (x86)/Steam/config/loginusers.vdf");
    let steam_user_id = if path.is_file() {
        match File::open(path) {
            Ok(mut file) => {
                let mut data = String::new();
                let _ = file.read_to_string(&mut data);

                let regex = Regex::new("(\\d+)").unwrap();
                if let Some(captures) = regex.captures(&data) {
                    if let Some(capture) = captures.get(0) {
                        if capture.as_str().parse::<u64>().is_ok() {
                            capture.as_str().to_owned()
                        } else { String::new() }
                    } else { String::new() }
                } else { String::new() }
            }
            Err(_) => String::new(),
        }
    } else {
        String::new()
    };

    set_setting_if_new_string(&q_settings, "steam_user_id", &steam_user_id);
    set_setting_if_new_string(&q_settings, "steam_api_key", "");
    set_setting_if_new_string(&q_settings, "default_game", KEY_WARHAMMER_3);
    set_setting_if_new_string(&q_settings, "update_channel", "stable");
    set_setting_if_new_string(&q_settings, "language", "English_en");
    set_setting_if_new_bool(&q_settings, "check_updates_on_start", true);
    set_setting_if_new_bool(&q_settings, "check_schema_updates_on_start", true);
    set_setting_if_new_bool(&q_settings, "dark_mode", false);

    for game in &SUPPORTED_GAMES.games_sorted() {
        if game.key() != KEY_ARENA && game.key() != KEY_SHOGUN_2 {
            set_setting_if_new_bool(&q_settings, &format!("enable_logging_{}", game.key()), false);
            set_setting_if_new_bool(&q_settings, &format!("enable_skip_intros_{}", game.key()), false);
            set_setting_if_new_string(&q_settings, &format!("enable_translations_{}", game.key()), "--");
            set_setting_if_new_bool(&q_settings, &format!("merge_all_mods_{}", game.key()), false);

            let game_path = if let Ok(Some(game_path)) = game.find_game_install_location() {
                game_path.to_string_lossy().to_string()
            } else {
                String::new()
            };

            // If we got a path and we don't have it saved yet, save it automatically.
            let current_path = setting_string_from_q_setting(&q_settings, game.key());
            if current_path.is_empty() && !game_path.is_empty() {
                set_setting_string_to_q_setting(&q_settings, game.key(), &game_path);
            } else {
                set_setting_if_new_string(&q_settings, game.key(), &game_path);
            }
        }
    }

    q_settings.sync();
}

//-------------------------------------------------------------------------------//
//                             Extra Helpers
//-------------------------------------------------------------------------------//

#[must_use = "Many things depend on this folder existing. So better check this worked."]
pub fn init_config_path() -> Result<()> {
    DirBuilder::new().recursive(true).create(error_path()?)?;
    DirBuilder::new().recursive(true).create(game_config_path()?)?;
    DirBuilder::new().recursive(true).create(profiles_path()?)?;
    DirBuilder::new().recursive(true).create(schemas_path()?)?;

    DirBuilder::new().recursive(true).create(translations_local_path()?)?;

    // Within the config path we need to create a folder to store the temp packs of each game.
    // Otherwise they interfere with each other due to being movie packs.
    for (_, game) in SUPPORTED_GAMES.games_sorted().iter().enumerate() {
        if game.key() != KEY_ARENA {
            DirBuilder::new().recursive(true).create(config_path()?.join("temp_packs").join(game.key()))?;
        }
    }

    Ok(())
}

pub fn temp_packs_folder(game: &GameInfo) -> Result<PathBuf> {
    Ok(config_path()?.join("temp_packs").join(game.key()))
}

pub fn schemas_path() -> Result<PathBuf> {
    Ok(config_path()?.join("schemas"))
}

pub fn game_config_path() -> Result<PathBuf> {
    Ok(config_path()?.join("game_config"))
}

pub fn profiles_path() -> Result<PathBuf> {
    Ok(config_path()?.join("profiles"))
}

pub fn rpfm_config_path() -> Result<PathBuf> {
    if cfg!(debug_assertions) { std::env::current_dir().map_err(From::from) } else {
        unsafe {
            match ProjectDirs::from(&QCoreApplication::organization_domain().to_std_string(), &QCoreApplication::organization_name().to_std_string(), "rpfm") {
                Some(proj_dirs) => Ok(proj_dirs.config_dir().to_path_buf()),
                None => Err(anyhow!("Failed to get RPFM's config path."))
            }
        }
    }
}

pub fn translations_local_path() -> Result<PathBuf> {
    rpfm_config_path().map(|path| path.join("translations_local"))
}
