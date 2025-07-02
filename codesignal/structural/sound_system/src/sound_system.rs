// TODO: Define the LegacySoundSystem struct
// It should have a method `play_sound` that prints "Sound playing from legacy system."
pub struct LegacySoundSystem;

impl LegacySoundSystem {
    pub fn play_sound(&self) {
        println!("Sound playing from legacy system.");
    }
}

// TODO: Define the ModernSoundSystemInterface trait
// It should have a method `output_sound`
pub trait ModernSoundSystemInterface {
    fn output_sound(&self);
}

// TODO: Define the SoundSystemAdapter struct implementing ModernSoundSystemInterface
// It should have a field that stores a LegacySoundSystem instance.
// Implement the `output_sound` method to call the `play_sound` method of LegacySoundSystem.
pub struct SoundSystemAdapter {
    pub legacy_sound_system: LegacySoundSystem,
}

impl ModernSoundSystemInterface for SoundSystemAdapter {
    fn output_sound(&self) {
        self.legacy_sound_system.play_sound();
    }
}
