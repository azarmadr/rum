use std::sync::mpsc::{self, TryRecvError};

use mpv::{MpvHandler, MpvHandlerBuilder, Result};

struct MediaWorker {
    handler: MpvHandler,
}

impl MediaWorker {
    pub fn new() -> Result<Self> {
        let handler = MpvHandlerBuilder::new()?.build()?;
        Ok(Self { handler })
    }

    pub fn loadfile(&mut self, url: &str) -> Result<()> {
        self.handler.command(&["loadfile", &url, "append-play"])?;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        self.handler.command(&["stop"])?;
        Ok(())
    }

    pub fn next(&mut self) -> Result<()> {
        self.handler.command(&["playlist-next"])?;
        Ok(())
    }

    pub fn prev(&mut self) -> Result<()> {
        self.handler.command(&["playlist-prev"])?;
        Ok(())
    }

    pub fn poll_events(&mut self) -> Result<bool> {
        while let Some(ev) = self.handler.wait_event(0.1) {
            match ev {
                mpv::Event::Shutdown | mpv::Event::Idle => {
                    return Ok(false);
                }
                mpv::Event::FileLoaded => {
                    log::debug!("mpv: file loaded");
                }
                _ => {}
            }
        }
        Ok(true)
    }
}

pub enum Command {
    Enqueue(String),
    Stop,
    NextTrack,
    PrevTrack,
}

pub struct Player {
    rx: mpsc::Receiver<Command>,
}

impl Player {
    pub fn new() -> (Self, mpsc::Sender<Command>) {
        let (tx, rx) = mpsc::channel();
        (Self { rx }, tx)
    }

    pub fn start_worker(self) -> std::thread::JoinHandle<Result<()>> {
        std::thread::spawn(move || {
            let mut worker = MediaWorker::new()?;
            loop {
                worker.poll_events()?;
                match self.rx.try_recv() {
                    Ok(Command::Enqueue(url)) => {
                        if let Err(err) = worker.loadfile(&url) {
                            log::error!("cannot perform loadfile: {}", err);
                        }
                    }
                    Ok(Command::Stop) => {
                        if let Err(err) = worker.stop() {
                            log::error!("cannot stop the track: {}", err);
                        }
                    }
                    Ok(Command::NextTrack) => {
                        if let Err(err) = worker.next() {
                            log::error!("cannot switch to next track: {}", err);
                        }
                    }
                    Ok(Command::PrevTrack) => {
                        if let Err(err) = worker.prev() {
                            log::error!("cannot switch to previous track: {}", err);
                        }
                    }
                    Err(TryRecvError::Empty) => {}
                    Err(TryRecvError::Disconnected) => {
                        log::warn!("player command stream disconnected, finishing");
                        return Ok(());
                    }
                }
            }
        })
    }
}
