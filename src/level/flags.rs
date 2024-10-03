use crate::prelude::*;

bitflags! {
    /// Determine where can a plant be put on, and what changes it makes to the terrain
    /// The lower 8 bits determines the compatibility, and the higher 8 bits determines the changes
    /// made to the terrain, respectively
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CreatureFlags: u16 {
        const TERRESTRIAL = 0x0001;
        const AQUATIC = 0x0002;
        const BARE_GROUND = 0x0004;

        const UNDIGGABLE = 0x0080;

        const MAKE_TERRESTRIAL = 0x0100;
        const MAKE_AQUATIC = 0x0200;
        const MAKE_BARE_GROUND = 0x0400;

        const MAKE_UNUSABLE = 0x8000;

        const TERRESTRIAL_PLANT = 0x8001;
        const AQUATIC_PLANT = 0x8002;
        const TERRESTRIAL_AQUATIC_PLANT = 0x8003;

        const LILY_PAD = 0x0102;
        const FLOWER_POT = 0x0105;
        const WATER_POT = 0x0205;
        const PUMPKIN = 0x0181;
        const COFFEE_BEAN = 0x0080;

        const GROUND_ZOMBIE = 0x0005;
        const GROUND_AQUATIC_ZOMBIE = 0x0007;
        const AQUATIC_ZOMBIE = 0x0002;

        const GRAVE = 0x8080;
        const GRAVE_BUSTER = 0x0080;
        const CRATER = 0x0080;
    }
}

impl CreatureFlags {
    pub fn compat_bits(&self) -> u16 {
        self.bits() & 0x00ff
    }

    pub fn terrain_bits(&self) -> u16 {
        self.bits() >> 8
    }

    // top is the flags provided by the top creature
    pub fn is_compat(&self, top: CreatureFlags) -> bool {
        self.compat_bits() & top.terrain_bits() != 0
    }

    pub fn is_pad(&self) -> bool {
        let bits = self.terrain_bits();
        bits == 0 || bits == CreatureFlags::MAKE_UNUSABLE.bits()
    }
}
