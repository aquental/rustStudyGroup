mod sound_system;

use sound_system::{LegacySoundSystem, ModernSoundSystemInterface, SoundSystemAdapter};

fn main() {
    // TODO: Create an instance of LegacySoundSystem
    let legacy_system = LegacySoundSystem {};
    // Modern system would be a concrete implementation of the interface

    // TODO: Create an instance of SoundSystemAdapter using the LegacySoundSystem instance
    let adapter = SoundSystemAdapter { legacy_sound_system: legacy_system};

    // TODO: Call the output_sound method via Adapter
    adapter.output_sound();
}
