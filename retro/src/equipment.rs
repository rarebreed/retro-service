use crate::{
    Attribute,
    Trait,
    Senses,
    EMSpectrum,
    AuditorySpectrum
};

pub enum WeaponClass {
    Unarmed,
    Melee,
    Ranged
}

pub enum DamageType {
    Energy,
    Kinetic,
    Radiation,
    Pathogen,
    Chemical
}

pub enum StructuralDamageType {
    Muscular,
    Bones,
    ConnectiveTissue
}

/// Trauma is immediate and destructive forms of damage
/// 
/// Weapons and other inflictions (disease, falling, being on fire) all cause certain kinds of damage.  This enum
/// covers the various kinds of damage that can be inflicted
pub enum Trauma {
    /// Damages by slashing, causing secondary Hemorraghic and Structural damage
    Cutting,
    /// Damages from an impactful force.  Causes internal hemorraghing and Structural damage
    Crushing,
    /// A piercing damage.  Causes less structural damage, but can cause deeper bleeding and obstructive shock
    Impaling,
    /// Tissue is destroyed by being burned
    Burning,
    Freezing,
    Poison,
    /// Diesease borne effect
    Pathological,
}

pub enum NeurologicalEffect {
    Pain,
    Unconsciousness,
    Stunned,
    Sickened,
    Weakened
}
/// 
pub enum DamageEffects {
    /// unable to breathe
    Asphyxiation,
    /// No nutrients or caloric intake, leading to eventual death
    Starvation,
    /// No water intake, leading to death
    Dehydration,
    /// Hemorraghic shock.  Ex-Sanguination.  How long until bleed out?
    Shock(u32),
    Structural {
        damage_type: StructuralDamageType,
        condition: Condition
    },
    Sensory {
        sense: Senses,
        condition: Condition
    },
    Neurological {
        effect: NeurologicalEffect,
        severity: u16
    },
    OrganFailure(u32)
}

/// Condition of equipment, part, organ, etc
pub enum Condition {
    /// The item is totally destroyed, not even salvageable
    Obliterated,
    /// The item is not able to be repaired.  Might be some salvage
    Destroyed,
    /// The item might be able to be repaired, but is essentially unusable
    Broken,
    /// The item is functional, but just barely.  Item needs to be serviced or healed
    Failing,
    /// The item is serviceable, but needs maintainance or rest.  More likely to breakdown
    Worn,
    /// Item is in regular and maintained condition
    Normal,
    /// Factory new
    Pristine,
    /// The equipment has been made more durable
    Reinforced(u16)
}

pub struct DamageClass {
    pub damage_type: DamageType,
    pub power: u16
}

pub struct Weapon {
    pub integrity: Condition
}