#![allow(dead_code, unused_imports, unused_variables)]
use core::time;
use std::error::Error;
use std::sync::mpsc::{self, Receiver};

use std::{thread, io};
use std::time::{Duration, Instant};

use crossterm::cursor::{Hide, Show, MoveLeft};
use crossterm::event::{Event, KeyCode};
use crossterm::{terminal, ExecutableCommand, event};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use invaders::frame::{new_frame, Drawable, Frame};
use invaders::player::Player;
use invaders::{frame, render};
fn main() -> Result <(), Box<dyn Error>> {
    
    //Setup
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    //Render loop in seperate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        //First render can use last frame reference. It will be force rendered anyway
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            //Renders the frames
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });
    //Gameloop
    let mut player = Player::new();
    let mut instant = Instant::now();
    'gameloop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        //Per frame init
        let mut curr_frame:Frame = frame::new_frame();
        //Input handling
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Right | KeyCode::Char('d') => player.move_right(),
                    KeyCode::Left | KeyCode::Char('a') => player.move_left(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {}
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
        //Updates
        player.update(delta);
        //Draw and render. Playing field is black
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        //Generates fewer frames per second
        thread::sleep(Duration::from_millis(1));
    }


    //Cleanup
    drop(render_tx);
    render_handle.join().ok();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
