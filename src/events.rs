use std::{sync::mpsc::{channel, Receiver, Sender}, thread, time::{Duration, Instant}};

use crossterm::event::{self, KeyEvent};

use crate::tui::Event;

pub struct EventHandler{
    receiver: Receiver<Event>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self{

        let tick_rate = Duration::from_millis(tick_rate);
        
        let (sender, receiver): (Sender<Event>,Receiver<Event>) = channel();

        {
            thread::spawn(move||{
                let sender = sender.clone();
                let mut last_tick = Instant::now();

                loop {
                    let timeout = last_tick.elapsed().checked_sub(tick_rate).unwrap_or(tick_rate);
                    
                    sender.send(Event::Tick).expect("unable to send tick event");
                    
                    if event::poll(timeout).expect("unable to pool event"){
                        match event::read().expect("unable to read event") {
                            event::Event::Key(e)=>{
                                sender.send(Event::Key(e)).expect("unable to send key event");
                            },
                            _=>{}
                        }
                    }

                    if last_tick.elapsed()>=tick_rate{
                        last_tick = Instant::now();
                    }
                }

            });
        }
        Self { receiver: receiver }
    }

    pub fn next(&self)->Result<Event, std::sync::mpsc::RecvError>{
        self.receiver.recv()
    }
}