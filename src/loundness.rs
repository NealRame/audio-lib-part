use std::path::Path;
use std::fs::File;

use symphonia::core::audio::AudioBufferRef;
use symphonia::core::codecs::{CODEC_TYPE_NULL, Decoder, DecoderOptions};
use symphonia::core::formats::{FormatOptions, FormatReader, Track};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use ebur128::EbuR128;

fn create_probe_hint() -> Hint {
    let mut hint = Hint::new();
    hint.with_extension("mp3");
    hint.with_extension("ogg");
    hint.with_extension("flac");
    hint.with_extension("wav");
    hint
}

fn find_audio_track(
    reader: &dyn FormatReader,
) -> Result<Track, Box<dyn std::error::Error>> {
    match reader.tracks().iter().find(
        |t| t.codec_params.codec != CODEC_TYPE_NULL
    ) {
        Some(track) => Ok(track.clone()),
        None => Err("No supported audio track found".into()),
    }
}

fn create_decoder(
    track: &Track,
) -> Result<Box<dyn Decoder>, Box<dyn std::error::Error>> {
    let decoder_options = DecoderOptions::default();
    let decoder = symphonia::default::get_codecs().make(
        &track.codec_params,
        &decoder_options
    )?;

    Ok(decoder)
}

fn analyse_audio_buffer(
    audio_buffer: &AudioBufferRef,
    ebur128: &mut EbuR128,
) -> Result<(), Box<dyn std::error::Error>> {
    match audio_buffer {
        AudioBufferRef::F32(buffer) => {
            ebur128.add_frames_planar_f32(buffer.planes().planes())?;
            Ok(())
        },
        AudioBufferRef::S16(buffer) => {
            ebur128.add_frames_planar_i16(buffer.planes().planes())?;
            Ok(())
        },
        _ => {
            Err("Unsupported audio buffer type".into())
        }
    }
}

fn is_eof(
    e: &symphonia::core::errors::Error,
) -> bool {
    match e {
        symphonia::core::errors::Error::IoError(e) => {
            e.kind() == std::io::ErrorKind::UnexpectedEof
        },
        _ => false,
    }
}

pub fn global(
    file_path: &Path,
) -> Result<f64, Box<dyn std::error::Error>> {
    let src = File::open(file_path)?;

    let hint = create_probe_hint();
    let format_opts = FormatOptions::default();
    let metadata_opts = MetadataOptions::default();

    let media_source_stream = MediaSourceStream::new(
        Box::new(src),
        Default::default()
    );

    let mut reader = symphonia::default::get_probe().format(
        &hint,
        media_source_stream,
        &format_opts,
        &metadata_opts
    )?.format;

    let track = find_audio_track(reader.as_ref())?;
    let mut track_decoder = create_decoder(&track)?;

    let mut ebur128 = EbuR128::new(
        track.codec_params.channels.unwrap().count() as u32,
        track.codec_params.sample_rate.unwrap(),
        ebur128::Mode::I,
    )?;

    loop {
        match reader.next_packet() {
            Err(e) if is_eof(&e) => {
                return Ok(ebur128.loudness_global()?);
            },
            Err(e) => {
                return Err(e.into());
            },
            Ok(pkt) if pkt.track_id() == track.id => {
                let audio_buffer = track_decoder.decode(&pkt)?;
                analyse_audio_buffer(
                    &audio_buffer,
                    &mut ebur128,
                )?;
            },
            _ => {
                // Ignore packets from other tracks
            },
        }
    }
}