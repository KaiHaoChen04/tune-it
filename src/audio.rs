use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}};
use crossbeam_channel::{Sender};
use std::time::Duration;

pub fn start_audio_capture(tx: Sender<Vec<f32>>) -> cpal::Stream {
    let host = cpal::default_host(); // current host of compilation platform
    let device = host.default_input_device().expect("Must have a valid device"); // input device driver
    let config = device.default_input_config().unwrap(); // device config

    //build a pipeline that sends the wave to rx, wait max 5 seconds before panicing
    let stream = device.build_input_stream(
        config.config(), 
        move |data: &[f32], _| {
            let _ = tx.try_send(data.to_vec());
        },
        |_err| eprintln!("Audio failed (send failed)")
        , 
        Some(Duration::from_secs(5)) 
    ).unwrap();

    //start receiving
    stream.play().unwrap();
    stream

}
