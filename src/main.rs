#![windows_subsystem = "windows"]

use single_instance::SingleInstance;
use inputbot::KeybdKey::*;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

fn play_audio(audio_path: &str) {
    // Create audio stream handle
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Open the audio file, decode it
    let file = BufReader::new(File::open(audio_path).unwrap());
    let source = Decoder::new(file).unwrap();

    // Create a new Sink to play the audio
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Append the decoded audio source to the Sink and play it
    sink.append(source);
    sink.play();
    
    // Sleep for the duration of the audio to keep the application alive
    std::thread::sleep(std::time::Duration::from_secs(4));
}

fn main() {
    println!("Starting Keydio. ");

    // Ensure that only one instance can run.
    let instance = SingleInstance::new("Keydio").unwrap();
    assert!(instance.is_single());

    // Define handlers for keys used later below in combinations
    LControlKey.bind(|| {});
    LAltKey.bind(|| {});
    LShiftKey.bind(|| {});

    // ENTER key listener
    EnterKey.bind(|| {
        play_audio("audio\\enter.mp3");
    });

    // SPACE key listener
    SpaceKey.bind(|| {
        play_audio("audio\\space.mp3");
    });

    // PRINT-SCREEN key listener
    OtherKey(44).bind(|| {
        play_audio("audio\\prntscrn.mp3");
    });

    // BACKSPACE key listener
    BackspaceKey.bind(|| {
        play_audio("audio\\back.mp3");
    });

    //SAVE key listener
    SKey.bind(|| {
        if LControlKey.is_pressed() {
            play_audio("audio\\save.mp3");
        }
    });

    //UNDO key listener
    ZKey.bind(|| {
        if LControlKey.is_pressed() {
            play_audio("audio\\undo.mp3");
        }
    });

    //COPY key listener
    CKey.bind(|| {
        if LControlKey.is_pressed() {
            play_audio("audio\\copy.mp3");
        }
    });

    //PASTE key listener
    VKey.bind(|| {
        if LControlKey.is_pressed() {
            play_audio("audio\\paste.mp3");
        }
    });

    //CLOSE key listener
    F4Key.bind(|| { 
        //close application (boom)
        if LAltKey.is_pressed() {
            play_audio("audio\\close.mp3");
        }
    });

    //CTRL+ALT+DELETE listener
    DeleteKey.bind(|| {
        if LControlKey.is_pressed() && LAltKey.is_pressed() {
            play_audio("audio\\panic.mp3");
        }

        //Only DELETE
        else {
            play_audio("audio\\delete.mp3");
        }
    });

    //CTRL+SHIFT+ESCAPE listener
    EscapeKey.bind(|| {
        if LControlKey.is_pressed() && LShiftKey.is_pressed() {
            play_audio("audio\\taskman.mp3");
        }
    });

    //EXIT callback
    QKey.bind(|| {
        if LControlKey.is_pressed() && LAltKey.is_pressed() {
            //unbind all other keys so that they don't trigger as we exit
            EnterKey.unbind();
            SpaceKey.unbind();
            OtherKey(44).unbind();
            BackspaceKey.unbind();
            SKey.unbind();
            ZKey.unbind();
            VKey.unbind();
            CKey.unbind();
            F4Key.unbind();
            QKey.unbind();
            DeleteKey.unbind();
            EscapeKey.unbind();

            //finally exit
            println!("See you next time!");
            play_audio("audio\\exit.mp3");
            std::process::exit(0);
        }
    });

    // Start listening for bound keys
    inputbot::handle_input_events();
}