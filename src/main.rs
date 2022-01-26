
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main()-> Result<(), anyhow::Error> {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");
    let config = device.default_output_config().unwrap();
    println!("Default output config: {:?}", config);

    let sample_rate : f32 = config.sample_rate().0 as f32;
    let channels = config.channels() as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        if sample_clock < 10000.0 {
            (sample_clock * (200.0 + sample_clock / 50.0) * 2.0 * std::f32::consts::PI / sample_rate).sin() * 0.01
        } else {
            0.0
        }
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                let value: f32 = cpal::Sample::from::<f32>(&next_value());
                for sample in frame.iter_mut() {
                    *sample = value;
                }
    }

        },
        err_fn,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(10000));
    println!("suka");
    Ok(())
}
