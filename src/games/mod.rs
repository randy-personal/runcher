//---------------------------------------------------------------------------//
// Copyright (c) 2017-2023 Ismael Gutiérrez González. All rights reserved.
//
// This file is part of the Rusted Launcher (Runcher) project,
// which can be found here: https://github.com/Frodo45127/runcher.
//
// This file is licensed under the MIT license, which can be found here:
// https://github.com/Frodo45127/runcher/blob/master/LICENSE.
//---------------------------------------------------------------------------//

use anyhow::Result;

use std::path::Path;

use rpfm_lib::files::pack::Pack;
use rpfm_lib::games::{*, supported_games::*};

use rpfm_ui_common::settings::*;

use crate::app_ui::AppUI;
use crate::SCHEMA;

const EMPTY_CA_VP8: [u8; 595] = [
    0x43, 0x41, 0x4d, 0x56, 0x01, 0x00, 0x29, 0x00, 0x56, 0x50, 0x38, 0x30, 0x80, 0x02, 0xe0, 0x01, 0x55, 0x55,
    0x85, 0x42, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x4a, 0x02, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x21, 0x02, 0x00, 0x00, 0x00, 0x50, 0x42, 0x00, 0x9d, 0x01, 0x2a, 0x80, 0x02, 0xe0, 0x01, 0x00, 0x47, 0x08,
    0x85, 0x85, 0x88, 0x85, 0x84, 0x88, 0x02, 0x02, 0x00, 0x06, 0x16, 0x04, 0xf7, 0x06, 0x81, 0x64, 0x9f, 0x6b,
    0xdb, 0x9b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27, 0x38, 0x7b, 0x27,
    0x38, 0x7b, 0x27, 0x37, 0x80, 0xfe, 0xff, 0xab, 0x50, 0x80, 0x29, 0x00, 0x00, 0x00, 0x21, 0x02, 0x00, 0x00,
    0x01,
];

mod warhammer_3;

//-------------------------------------------------------------------------------//
//                             Implementations
//-------------------------------------------------------------------------------//

pub unsafe fn setup_launch_options(app_ui: &AppUI, game: &GameInfo) {

    // Only set enabled the launch options that work for the current game.
    match game.key() {
        KEY_WARHAMMER_3 => {
            let schema = SCHEMA.read().unwrap();
            app_ui.actions_ui().enable_logging().set_enabled(true);
            app_ui.actions_ui().enable_skip_intro().set_enabled(true);
            app_ui.actions_ui().merge_all_mods().set_enabled(true);
            app_ui.actions_ui().unit_multiplier_spinbox().set_enabled(schema.is_some());
        },
        KEY_TROY => {
            app_ui.actions_ui().enable_logging().set_enabled(false);
            app_ui.actions_ui().enable_skip_intro().set_enabled(false);
            app_ui.actions_ui().merge_all_mods().set_enabled(true);
            app_ui.actions_ui().unit_multiplier_spinbox().set_enabled(false);
        },
        KEY_THREE_KINGDOMS => {
            app_ui.actions_ui().enable_logging().set_enabled(false);
            app_ui.actions_ui().enable_skip_intro().set_enabled(false);
            app_ui.actions_ui().merge_all_mods().set_enabled(true);
            app_ui.actions_ui().unit_multiplier_spinbox().set_enabled(false);
        },
        KEY_WARHAMMER_2 => {
            app_ui.actions_ui().enable_logging().set_enabled(false);
            app_ui.actions_ui().enable_skip_intro().set_enabled(false);
            app_ui.actions_ui().merge_all_mods().set_enabled(true);
            app_ui.actions_ui().unit_multiplier_spinbox().set_enabled(false);
        },
        KEY_WARHAMMER => {
            app_ui.actions_ui().enable_logging().set_enabled(false);
            app_ui.actions_ui().enable_skip_intro().set_enabled(false);
            app_ui.actions_ui().merge_all_mods().set_enabled(true);
            app_ui.actions_ui().unit_multiplier_spinbox().set_enabled(false);
        },
        KEY_THRONES_OF_BRITANNIA => {
            app_ui.actions_ui().enable_logging().set_enabled(false);
            app_ui.actions_ui().enable_skip_intro().set_enabled(false);
            app_ui.actions_ui().merge_all_mods().set_enabled(true);
            app_ui.actions_ui().unit_multiplier_spinbox().set_enabled(false);
        },
        KEY_ATTILA => {
            app_ui.actions_ui().enable_logging().set_enabled(false);
            app_ui.actions_ui().enable_skip_intro().set_enabled(false);
            app_ui.actions_ui().merge_all_mods().set_enabled(true);
            app_ui.actions_ui().unit_multiplier_spinbox().set_enabled(false);
        },
        KEY_ROME_2 => {
            app_ui.actions_ui().enable_logging().set_enabled(false);
            app_ui.actions_ui().enable_skip_intro().set_enabled(false);
            app_ui.actions_ui().merge_all_mods().set_enabled(true);
            app_ui.actions_ui().unit_multiplier_spinbox().set_enabled(false);
        },
        KEY_SHOGUN_2 => {
            app_ui.actions_ui().enable_logging().set_enabled(false);
            app_ui.actions_ui().enable_skip_intro().set_enabled(false);
            app_ui.actions_ui().merge_all_mods().set_enabled(true);
            app_ui.actions_ui().unit_multiplier_spinbox().set_enabled(false);
        },
        KEY_NAPOLEON => {
            app_ui.actions_ui().enable_logging().set_enabled(false);
            app_ui.actions_ui().enable_skip_intro().set_enabled(false);
            app_ui.actions_ui().merge_all_mods().set_enabled(true);
            app_ui.actions_ui().unit_multiplier_spinbox().set_enabled(false);
        },
        KEY_EMPIRE => {
            app_ui.actions_ui().enable_logging().set_enabled(false);
            app_ui.actions_ui().enable_skip_intro().set_enabled(false);
            app_ui.actions_ui().merge_all_mods().set_enabled(true);
            app_ui.actions_ui().unit_multiplier_spinbox().set_enabled(false);
        }
        &_ => {},
    }

    // Update the launch options for the new game.
    app_ui.actions_ui().enable_logging().set_checked(setting_bool(&format!("enable_logging_{}", game.key())));
    app_ui.actions_ui().enable_skip_intro().set_checked(setting_bool(&format!("enable_skip_intros_{}", game.key())));
    app_ui.actions_ui().merge_all_mods().set_checked(setting_bool(&format!("merge_all_mods_{}", game.key())));
    app_ui.actions_ui().unit_multiplier_spinbox().set_value({
        let value = setting_f32(&format!("unit_multiplier_{}", game.key()));
        if value == 0.00 {
            1.00
        } else {
            value
        }
    } as f64);
}

pub unsafe fn prepare_unit_multiplier(app_ui: &AppUI, game: &GameInfo, game_path: &Path, reserved_pack: &mut Pack) -> Result<()> {
    match *SCHEMA.read().unwrap() {
        Some(ref schema) => {
            if app_ui.actions_ui().unit_multiplier_spinbox().is_enabled() && app_ui.actions_ui().unit_multiplier_spinbox().value() != 1.00 {
                match game.key() {
                    KEY_WARHAMMER_3 => warhammer_3::prepare_unit_multiplier(app_ui, game, game_path, reserved_pack, schema),
                    _ => Ok(())
                }
            } else {
                Ok(())
            }
        }
        None => Ok(())
    }
}

pub unsafe fn prepare_script_logging(app_ui: &AppUI, game: &GameInfo, reserved_pack: &mut Pack) -> Result<()> {
    if app_ui.actions_ui().enable_logging().is_enabled() && app_ui.actions_ui().enable_logging().is_checked() {
        match game.key() {
            KEY_WARHAMMER_3 => warhammer_3::prepare_script_logging(reserved_pack),
            _ => Ok(())
        }
    } else {
        Ok(())
    }
}

pub unsafe fn prepare_skip_intro_videos(app_ui: &AppUI, game: &GameInfo, reserved_pack: &mut Pack) -> Result<()> {
    if app_ui.actions_ui().enable_skip_intro().is_enabled() && app_ui.actions_ui().enable_skip_intro().is_checked() {
        match game.key() {
            KEY_WARHAMMER_3 => warhammer_3::prepare_skip_intro_videos(reserved_pack),
            _ => Ok(())
        }
    } else {
        Ok(())
    }
}
