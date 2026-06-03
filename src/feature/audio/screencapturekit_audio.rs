use std::sync::{Arc, Mutex};

use futures::channel::oneshot;
use parking_lot::Mutex as ParkingMutex;

use crate::feature::audio::{
    WgrabAudioError, WgrabAudioFormat, WgrabAudioStream, WgrabSampleFormat,
};
use crate::platform::platform_impl::objc_wrap::{
    DispatchQueue, NSArray, SCContentFilter, SCShareableContent, SCStream, SCStreamConfiguration,
    SCStreamHandler, SCStreamOutputType, SCStreamSampleRate,
};

pub(crate) struct ScreenCaptureKitAudioStream {
    stream: SCStream,
}

impl Drop for ScreenCaptureKitAudioStream {
    fn drop(&mut self) {
        self.stream.stop();
    }
}

pub(crate) fn build_default_screencapturekit_audio_stream(
) -> Result<WgrabAudioStream, WgrabAudioError> {
    if !SCStream::preflight_access() {
        return Err(WgrabAudioError::ScreenCaptureKitUnavailable(
            "screen capture permission is not granted".to_string(),
        ));
    }

    let content = shareable_content()?;
    let display = content.displays().into_iter().next().ok_or_else(|| {
        WgrabAudioError::ScreenCaptureKitUnavailable("no shareable display found".to_string())
    })?;
    let display_frame = display.frame();

    let mut config = SCStreamConfiguration::new();
    config.set_size(display_frame.size);
    config.set_capture_audio(true);
    config.set_sample_rate(SCStreamSampleRate::R48000);
    config.set_channel_count(2);
    config.set_exclude_current_process_audio(false);
    config.set_queue_depth(3);

    let filter = SCContentFilter::new_with_display_excluding_apps_excepting_windows(
        display,
        NSArray::new(),
        NSArray::new(),
    );
    let handler_queue =
        DispatchQueue::make_serial("com.ihasq.wgrab.screencapturekit.audio".to_string());
    let buffer = Arc::new(Mutex::new(Vec::new()));
    let callback_buffer = Arc::clone(&buffer);

    let handler = SCStreamHandler::new(move |stream_result| match stream_result {
        Ok((sample_buffer, SCStreamOutputType::Audio)) => {
            if let Ok(audio_data) = sample_buffer.copy_audio_samples_f32() {
                if let Ok(mut samples) = callback_buffer.lock() {
                    samples.extend_from_slice(&audio_data.samples);
                }
            }
        }
        Ok((_sample_buffer, SCStreamOutputType::Screen)) => {}
        Err(_error) => {}
    });

    let mut stream = SCStream::new_with_output_type(
        filter,
        config,
        handler_queue,
        handler,
        SCStreamOutputType::Audio,
    )
    .map_err(WgrabAudioError::ScreenCaptureKitUnavailable)?;
    stream.start();

    Ok(WgrabAudioStream::new_screencapturekit_audio(
        ScreenCaptureKitAudioStream { stream },
        buffer,
        WgrabAudioFormat {
            sample_rate: 48_000,
            channels: 2,
            sample_format: WgrabSampleFormat::F32,
        },
        Some("ScreenCaptureKit default display audio".to_string()),
    ))
}

fn shareable_content() -> Result<SCShareableContent, WgrabAudioError> {
    let (tx, rx) = oneshot::channel();
    let tx = Arc::new(ParkingMutex::new(Some(tx)));
    SCShareableContent::get_shareable_content_with_completion_handler(false, true, move |result| {
        if let Some(tx) = tx.lock().take() {
            let _ = tx.send(result);
        }
    });

    match futures::executor::block_on(rx) {
        Ok(Ok(content)) => Ok(content),
        Ok(Err(error)) => Err(WgrabAudioError::ScreenCaptureKitUnavailable(format!(
            "SCShareableContent error: code={}, description={}, reason={}",
            error.code(),
            error.description(),
            error.reason()
        ))),
        Err(error) => Err(WgrabAudioError::ScreenCaptureKitUnavailable(format!(
            "failed to receive SCShareableContent result: {error}"
        ))),
    }
}
