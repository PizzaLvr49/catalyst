#![expect(unused)]

use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Combatant {
    health: u32,
    mana: u32,
    stats: CombatStats,
}

#[derive(Debug)]
pub struct CombatStats {
    strength: u32,
    vitality: u32,
    dexterity: u32,
    intelligence: u32,
}

#[derive(Component)]
pub struct IsAlly;

#[derive(Component)]
pub struct IsEnemy;

pub struct AbilityCtx {
    allies: Vec<Combatant>,
    enemies: Vec<Combatant>,
}

pub enum AbilityTarget {
    First,
}

impl AbilityCtx {
    fn find_target(&mut self, target: AbilityTarget) -> &mut Combatant {
        assert!(!self.enemies.is_empty(), "No enemies");

        match target {
            AbilityTarget::First => self.enemies.first_mut().unwrap(),
        }
    }

    pub fn damage_enemy(&mut self, target: AbilityTarget, amount: u32) {
        let target = self.find_target(target);
        target.health.saturating_sub(amount);
    }
}

pub trait CombatAbility {
    fn execute(ctx: &mut AbilityCtx);
}

/// Example
pub struct Fireball;

impl CombatAbility for Fireball {
    fn execute(ctx: &mut AbilityCtx) {
        ctx.damage_enemy(AbilityTarget::First, 30);
    }
}
