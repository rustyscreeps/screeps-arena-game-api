
pub const BODYPART_HITS: u32 = 100;

pub const RANGED_ATTACK_POWER: u32  = 10;

pub const RANGED_ATTACK_DISTANCE_RATE: &[f32;4] = &[1.0_f32, 1.0_f32, 0.4_f32, 0.1_f32];

pub const ATTACK_POWER: u32  = 30;

pub const HEAL_POWER: u32  = 12;

pub const RANGED_HEAL_POWER: u32  = 4;

pub const CARRY_CAPACITY: u32 = 50;

pub const REPAIR_POWER: u32 = 100;

pub const DISMANTLE_POWER: u32 = 50;

pub const REPAIR_COST: f32 = 0.01_f32;

pub const DISMANTLE_COST: f32 = 0.005_f32;

pub const HARVEST_POWER: u32 = 2;

pub const BUILD_POWER: u32 = 5;

pub const OBSTACLE_OBJECT_TYPES: &[&str;6] = &["creep","tower","constructedWall","spawn","extension","link"];

pub const TOWER_ENERGY_COST: u32 = 10;

pub const TOWER_RANGE: u32 = 50;

pub const TOWER_HITS: u32 = 3000;

pub const TOWER_CAPACITY: u32 = 50;

pub const TOWER_POWER_ATTACK: u32 = 150;

pub const TOWER_POWER_HEAL: u32 = 100;

pub const TOWER_POWER_REPAIR: u32 = 200;

pub const TOWER_OPTIMAL_RANGE: u32 = 5;

pub const TOWER_FALLOFF_RANGE: u32 = 20;

pub const TOWER_FALLOFF: f32 = 0.75_f32;

pub const TOWER_COOLDOWN: u32 = 10;

pub const MAX_CREEP_SIZE: u32 = 50;

pub const CREEP_SPAWN_TIME: u32 = 3;

pub const RESOURCE_ENERGY: &str =  "energy";

pub const RESOURCES_ALL: &[&str;1] = &[RESOURCE_ENERGY];

pub const SOURCE_ENERGY_REGEN: u32 = 10;

pub const RESOURCE_DECAY: u32 = 1000;

pub const MAX_CONSTRUCTION_SITES: u32 = 10;

pub const CONSTRUCTION_COST_ROAD_SWAMP_RATIO: u32 = 5;

pub const CONSTRUCTION_COST_ROAD_WALL_RATIO: u32 = 150;

pub const CONTAINER_HITS: u32 = 300;

pub const CONTAINER_CAPACITY: u32 = 2000;

pub const WALL_HITS: u32 = 10000;

pub const WALL_HITS_MAX: u32 = 10000;

pub const RAMPART_HITS: u32 = 10000;

pub const RAMPART_HITS_MAX: u32 = 10000;

pub const ROAD_HITS: u32 = 500;

pub const ROAD_WEAROUT: u32 = 1;

pub const EXTENSION_HITS: u32 = 100;

pub const EXTENSION_ENERGY_CAPACITY: u32 = 100;

pub const SPAWN_ENERGY_CAPACITY: u32 = 1000;

pub const SPAWN_HITS: u32 = 3000;
