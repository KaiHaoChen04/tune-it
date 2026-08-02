use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}};
use crossbeam_channel::{bounded, Sender};
use std::time::Duration;

fn start_audio_capture(tx: Sender<Vec<f32>>) -> cpal::Stream {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("Must have a valid device");
    let config = device.default_input_config().unwrap();

    let stream = device.build_input_stream(
        config.config(), 
        move |data: &[f32], _| {
            let _ = tx.try_send(data.to_vec());
        },
        |err| eprintln!("Audio failed (send failed)")
        , 
        Some(Duration::from_secs(5)) 
    ).unwrap();

    stream.play().unwrap();
    stream

}
