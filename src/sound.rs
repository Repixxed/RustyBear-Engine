use kira::{manager::{backend::DefaultBackend, AudioManagerSettings, AudioManager}, sound::{streaming::{StreamingSoundData, StreamingSoundSettings}, static_sound::{StaticSoundData, StaticSoundSettings}}};

use crate::config::ThemeConfiguration;

pub struct AudioEngine {
    manager: AudioManager,
    background_music: String,
    //click_sound: StaticSoundData,
}

impl AudioEngine {

    pub fn new(theme_conf: &ThemeConfiguration) -> Self
    {
        AudioEngine { manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap(), background_music: theme_conf.background_music.clone() }
    }

    pub fn play_click(&mut self)
    {
        //let mut sound = self.manager.play(self.click_sound.clone()).unwrap();
        //let _ = sound.set_volume(0.1, kira::tween::Tween::default());
    }

    pub fn play_background(&mut self)
    {
        let sound_data_res = StreamingSoundData::from_file(format!("themes/{}", self.background_music), StreamingSoundSettings::new().loop_region(0.0..));

        if let Ok(sound_data) = sound_data_res
        {
            let mut sound = self.manager.play(sound_data).unwrap();
            let _ = sound.set_volume(kira::Volume::Decibels(-20.0), kira::tween::Tween::default());
        }
        else
        {
            log::error!("Could not load background music {}. Silence.", self.background_music);
        }
    }
}

impl Default for AudioEngine {

    fn default() -> Self
    {
        Self::new(&ThemeConfiguration::default())
    }
}
