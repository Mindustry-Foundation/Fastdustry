use brood::Registry;
use crate::components::position::{ Position, Velocity };
use crate::components::unit_type::{ IsFloating, IsGround, IsFlying };

pub type Registry = Registry!(Position, Velocity, IsFlying, IsGround, IsFloating);
