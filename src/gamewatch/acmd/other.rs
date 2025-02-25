use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::lib::{L2CValue, L2CAgent};
use std::mem;
use smash::app::*;
use smash::phx::Vector3f;
use crate::util::*;
use super::*;
pub fn install() {
    smashline::install_acmd_scripts!(
		gnw_sidetaunt_snd, gnw_sidetaunt_expr
    );
}
#[acmd_script(
    agent = "gamewatch",
    scripts =  ["sound_appealsr", "sound_appealsl"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn gnw_sidetaunt_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_gamewatch_wave03_mi"));
		}
		frame(fighter.lua_state_agent, /*Frames*/ 25.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_gamewatch_win_03"));
		}
}	
#[acmd_script(
    agent = "gamewatch",
    scripts =  ["expression_appealsr", "expression_appealsl"],
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn gnw_sidetaunt_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	