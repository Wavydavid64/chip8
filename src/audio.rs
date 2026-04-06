use rodio::source::{SineWave, Source};
use rodio::{DeviceSinkBuilder, Player};

pub struct Audio {
    _device: rodio::MixerDeviceSink,
    player: Player,
}

impl Audio {
    pub fn new() -> Self {
        let device =
            DeviceSinkBuilder::open_default_sink().expect("failed to open default audio device");

        let player = Player::connect_new(device.mixer());

        let tone = SineWave::new(440.0).amplify(0.20).repeat_infinite();

        player.append(tone);
        player.pause();

        Self {
            _device: device,
            player,
        }
    }

    pub fn play_tone(&self) {
        self.player.play();
    }

    pub fn pause_tone(&self) {
        self.player.pause();
    }
}
