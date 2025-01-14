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
    smashline::install_agent_frames!(
		chrom_frame
    );
}
#[fighter_frame( agent = FIGHTER_KIND_CHROM )]
fn chrom_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let frame = MotionModule::frame(boma);
			if [hash40("special_s1"), hash40("special_air_s1")].contains(&MotionModule::motion_kind(boma)) {
					if MotionModule::frame(boma) > 4.0 && MotionModule::frame(boma) < 31.0{
						if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_DASH{
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_DASH);
						};
					} else {
						if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
							if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION{
								KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
							};
						} else {
							if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_FALL{
								KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
							};
						}
					};
					if MotionModule::frame(boma) > 6.0 && MotionModule::frame(boma) < 8.0 {
						macros::SET_SPEED_EX(fighter, 1.7, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_NONE);
					};
					if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
						if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4, true);
						} else {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
						};
					};
			};
			if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
				CAN_SIDEB[ENTRY_ID] = 0;
			};
			if [hash40("special_s1"), hash40("special_air_s1")].contains(&MotionModule::motion_kind(boma)) {
					CAN_SIDEB[ENTRY_ID] = 1;
			};
		}
    }
}