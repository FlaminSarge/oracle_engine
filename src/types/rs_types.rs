use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::enemies::EnemyType;

#[derive(Debug, Clone, Deserialize)]
pub struct DamageMods {
    pub pve: f64,
    pub minor: f64,
    pub elite: f64,
    pub miniboss: f64,
    pub champion: f64,
    pub boss: f64,
    pub vehicle: f64,
}
impl Default for DamageMods {
    fn default() -> Self {
        DamageMods {
            pve: 1.0,
            minor: 1.0,
            elite: 1.0,
            miniboss: 1.0,
            champion: 1.0,
            boss: 1.0,
            vehicle: 1.0,
        }
    }
}
impl DamageMods {
    pub fn get_mod(&self, _type: &EnemyType) -> f64 {
        let combatant_scale = match _type {
            &EnemyType::MINOR => self.minor,
            &EnemyType::ELITE => self.elite,
            &EnemyType::MINIBOSS => self.miniboss,
            &EnemyType::CHAMPION => self.champion,
            &EnemyType::BOSS => self.boss,
            &EnemyType::VEHICLE => self.vehicle,
            _ => 1.0,
        };
        combatant_scale
    }
}

#[derive(Debug, Clone, Default)]
pub struct RangeFormula {
    pub start: StatQuadraticFormula,
    pub end: StatQuadraticFormula,
    pub floor_percent: f64,
    pub fusion: bool,
}

//even if just linear use this
#[derive(Debug, Clone, Default, Deserialize)]
pub struct StatQuadraticFormula {
    pub evpp: f64,
    pub vpp: f64,
    pub offset: f64,
}
impl StatQuadraticFormula {
    pub fn solve_at(&self, _x: f64) -> f64 {
        self.evpp * _x * _x + self.vpp * _x + self.offset
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReloadFormula {
    pub reload_data: StatQuadraticFormula,
    pub ammo_percent: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct HandlingFormula {
    pub ready: StatQuadraticFormula,
    pub stow: StatQuadraticFormula,
    pub ads: StatQuadraticFormula,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AmmoFormula {
    pub mag: StatQuadraticFormula,
    pub round_to: i32,
    pub reserve_id: u32,
}

#[derive(Debug, Clone, Default)]
pub struct RangeResponse {
    pub hip_falloff_start: f64,
    pub hip_falloff_end: f64,
    pub ads_falloff_start: f64,
    pub ads_falloff_end: f64,
    pub floor_percent: f64,
}

#[derive(Debug, Clone, Default, Copy)]
pub struct HandlingResponse {
    pub ready_time: f64,
    pub stow_time: f64,
    pub ads_time: f64,
}

#[derive(Debug, Clone, Default)]
pub struct AmmoResponse {
    pub mag_size: i32,
    pub reserve_size: i32,
}

#[derive(Debug, Clone, Default)]
pub struct ReloadResponse {
    pub reload_time: f64,
    pub ammo_time: f64,
}

#[derive(Debug, Clone, Default)]
pub struct DpsResponse {
    pub dps_per_mag: Vec<f64>,
    pub time_damage_data: Vec<(f64, f64)>,
    pub total_damage: f64,
    pub total_time: f64,
    pub total_shots: i32,
}
impl DpsResponse {
    pub fn apply_rpl(&mut self, rpl: f64) {
        for mag in self.dps_per_mag.iter_mut() {
            *mag *= rpl;
        }
        for (_, damage) in self.time_damage_data.iter_mut() {
            *damage *= rpl;
        }
        self.total_damage *= rpl;
    }
}

#[derive(Debug, Clone, Default)]
pub struct FiringResponse {
    pub pvp_damage: f64,
    pub pvp_crit_mult: f64,

    pub pve_damage: f64,
    pub pve_crit_mult: f64,

    pub burst_delay: f64,
    pub burst_duration: f64,
    pub burst_size: i32,

    pub rpm: f64,
}
impl FiringResponse{
    pub fn set_rpm(&mut self, _extra_charge_delay: f64) {
        let mut time = 0.0;
        time += self.burst_delay;
        time += self.burst_duration;
        // time += _extra_charge_delay*self.burst_delay;
        let avg_bullet_time = time / self.burst_size as f64;
        self.rpm = (1.0/avg_bullet_time)*60.0
    }
    pub fn apply_pve_bonuses(&mut self, _rpl_mult: f64, _gpl_mult: f64, _pve_mult: f64, _combatant_mult: f64) {
        self.pve_damage *= _rpl_mult * _gpl_mult * _pve_mult * _combatant_mult;
    }
}