use crate::{card::{Buff, Debuff}, fight::Enemy, game::{debuff_player_turn_wind_down, Game}};

impl Game {

    pub fn buff_enemy(enemy: &mut Enemy, buff: Buff) {
        fn panic_not_apply_enemies(buff: Buff) -> ! {
            panic!("Buff {:?} doesn't apply to enemies", buff);
        }
        match buff {
            Buff::Strength(x) => {
                enemy.buffs.strength += x;
            }
            Buff::Ritual(x) => {
                enemy.buffs.ritual += x;
            }
            Buff::RitualSkipFirst(x) => {
                enemy.buffs.ritual_skip_first += x;
            }
            Buff::Enrage(x) => {
                enemy.buffs.enrage += x;
            }
            Buff::EndTurnDamageAllEnemies(_)
            | Buff::EndTurnLoseHP(_)
            | Buff::DarkEmbraceBuff
            | Buff::EvolveBuff(_)
            | Buff::FNPBuff(_)
            | Buff::FireBreathingBuff(_)
            | Buff::TempSpikes(_)
            | Buff::Metallicize(_)
            | Buff::RageBuff(_)
            | Buff::RuptureBuff(_)
            | Buff::BarricadeBuff
            | Buff::EnergyEveryTurn
            | Buff::BrutalityBuff
            | Buff::CorruptionBuff
            | Buff::DoubleTap(_)
            | Buff::Juggernaut(_) => {
                panic_not_apply_enemies(buff);
            }
        }
    }
    
    pub fn apply_debuff_to_player(&mut self, debuff: Debuff) {
        match debuff {
            Debuff::Vulnerable(amount) => {
                debuff_player_turn_wind_down(&mut self.fight.player_debuffs.vulnerable, amount);
            }
            Debuff::Weak(amount) => {
                debuff_player_turn_wind_down(&mut self.fight.player_debuffs.weak, amount);
            }
            Debuff::Frail(amount) => {
                debuff_player_turn_wind_down(&mut self.fight.player_debuffs.frail, amount);
            }
            Debuff::Entangled => {
                self.fight.player_debuffs.entangled = true;
            }
            Debuff::StrengthDown(x) => {
                self.fight.player_debuffs.strength_down += x;
            }
            Debuff::NoDraw => {
                self.fight.player_debuffs.no_draw = true;
            }
            Debuff::DexterityDown(x) => {
                self.fight.player_debuffs.dexterity_down += x;
            }
            Debuff::MinusStrength(x) => {
                self.fight.player_buffs.strength -= x;
            }
            Debuff::MinusDexterity(x) => {
                self.fight.player_buffs.dexterity -= x;
            }
        }
    }

    pub fn apply_buff_to_player(&mut self, buff: Buff) {
        fn panic_not_apply_player(buff: Buff) -> ! {
            panic!("Buff {:?} doesn't apply to the player", buff);
        }
        match buff {
            //TODO handle if player has negative strength.
            Buff::Strength(x) => {
                self.fight.player_buffs.strength += x;
            }
            Buff::Ritual(x) => self.fight.player_buffs.ritual += x,
            Buff::RitualSkipFirst(_) => unimplemented!("Player gets normal ritual"),
            Buff::EndTurnLoseHP(x) => self.fight.player_buffs.end_turn_lose_hp += x,
            Buff::EndTurnDamageAllEnemies(x) => {
                self.fight.player_buffs.end_turn_damage_all_enemies += x
            }
            Buff::DarkEmbraceBuff => self.fight.player_buffs.dark_embrace += 1,
            Buff::EvolveBuff(x) => self.fight.player_buffs.evolve += x,
            Buff::FNPBuff(x) => self.fight.player_buffs.fnp += x,
            Buff::FireBreathingBuff(x) => self.fight.player_buffs.fire_breathing += x,
            Buff::TempSpikes(x) => self.fight.player_buffs.temp_spikes += x,
            Buff::Metallicize(x) => self.fight.player_buffs.metallicize += x,
            Buff::RageBuff(x) => self.fight.player_buffs.rage += x,
            Buff::RuptureBuff(x) => self.fight.player_buffs.rupture += x,
            Buff::BarricadeBuff => self.fight.player_buffs.barricade = true,
            Buff::EnergyEveryTurn => self.fight.player_buffs.energy_every_turn += 1,
            Buff::BrutalityBuff => self.fight.player_buffs.brutality += 1,
            Buff::CorruptionBuff => self.fight.player_buffs.corruption = true,
            Buff::DoubleTap(x) => self.fight.player_buffs.double_tap += x,
            Buff::Juggernaut(x) => self.fight.player_buffs.juggernaut += x,
            Buff::Enrage(_) => panic_not_apply_player(buff),
        }
    }

}