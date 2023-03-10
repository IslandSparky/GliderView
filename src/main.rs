/*Game of Life - gliderview

MIT License
Copyright (c) 2023 Darwin Geiselbrecht
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect};
use std::time::Duration;

use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;
use std::io::Read;
use std::io;

//extern crate rand;
//use rand::Rng;
//use std::[thread,time];


const X_MAX: usize= 1600;               // width of the world in pixels
const Y_MAX: usize = 1200;              // heigth of the world in pixels
const SIZE: usize = 6;                  // size of a cell

const NUM_X: usize = X_MAX / SIZE;       // width of the world in cells
const NUM_Y: usize = Y_MAX / SIZE;       // height of the world in cells

const START_SIZE: usize = 20 ;           // length of each side of the starting block in the center of world

const HISTORY_MAX:usize = 20;                    // Length of the arrays to check for stable census




// Save the starting world for possible replay, should be called right after randomize
fn save_starting_world (world:  [[u8;NUM_Y]; NUM_X]) -> [[u8;START_SIZE];START_SIZE]{
    let mut starting_world = [[0u8;START_SIZE];START_SIZE];

    // Now remember  only the center cells for possible world restart
    let x_start:usize = NUM_X / 2 - START_SIZE / 2;  // offset to center the starting block
    let y_start:usize = NUM_Y / 2 -START_SIZE / 2;
    for  y in 0 .. START_SIZE {
        for x in 0.. START_SIZE {
           starting_world [x][y]  = world[x+x_start][y+y_start] ;   
        }
    }     
    starting_world
}
fn read_starting_world  (file: &mut File ) -> [[u8;START_SIZE];START_SIZE]{


	let mut starting_world = [[0u8;START_SIZE];START_SIZE];
    let mut buffer = [0u8;START_SIZE*START_SIZE];

	file.read_exact(&mut buffer ).expect("could not read record");

    // now unflatten the buffer to the two dimensional array
    let mut buffer_index = 0;
    for  y in 0 .. START_SIZE {
        for x in 0.. START_SIZE {
           starting_world [x][y] = buffer[buffer_index]; 
           buffer_index += 1;  
        }
    } 
    starting_world
}

// Restore the world back to the initial condition saved in the last save_starting world
fn restore_starting_world (starting_world: [[u8;START_SIZE];START_SIZE]) -> [[u8;NUM_Y]; NUM_X]  {
    
    let mut new_world = [[0u8;NUM_Y]; NUM_X];       // start with a clean slate

    // Now remember  only the center cells for possible world restart
    let x_start:usize = NUM_X / 2 - START_SIZE / 2;  // offset to center the starting block
    let y_start:usize = NUM_Y / 2 -START_SIZE / 2;
    for  y in 0 .. START_SIZE {
        for x in 0.. START_SIZE {
           new_world[x+x_start][y+y_start] =starting_world [x][y] ;   
        }
    } 
    new_world
}

// Save starting world by flattening it and appending to file "saved.dat"
fn write_starting_world (starting_world: [[u8;START_SIZE];START_SIZE]) {

    let mut buffer = [0u8;START_SIZE*START_SIZE];

    // now flatten array into flat buffer
    let mut buffer_index = 0;
    for  y in 0 .. START_SIZE {
        for x in 0.. START_SIZE {
           buffer[buffer_index] = starting_world [x][y];
           buffer_index += 1;  
        }
    }  

    // open the saved.dat file and append the buffer to it
    {
		let mut file = OpenOptions::new().append(true).create(true).open("saved.dat").unwrap();
		file.write_all(&buffer).expect("Couldn't write to saved.dat file");
	} // end of scope closes file
    println!("Wrote this world to saved.dat")
}

// Count the number of live cells in the current world
// returns a tuple consistion of the current population plus the size of the active region.
// Where size is the number of cells in the smallest rectangle that contains living cells
fn census ( world:  [[u8;NUM_Y]; NUM_X]) -> (i32,usize) {
    let mut count: i32 = 0;
    let mut smallest_x = NUM_X;
    let mut largest_x = 0;
    let mut smallest_y = NUM_Y;
    let mut largest_y = 0;

    for  y in 0 .. NUM_Y  {
        for x in 0.. NUM_X {
            if world[x][y] != 0{
                count += 1;
                if x < smallest_x {
                    smallest_x = x;
                }
                if x > largest_x {
                    largest_x = x;
                }
                if y < smallest_y {
                    smallest_y = y;
                }
                if y > largest_y {
                    largest_y = y;
                }
            }
        }
    } 
    let x_size = largest_x - smallest_x;
    let y_size = largest_y - smallest_y;
    (count, ( x_size * y_size)  )
}

// Check for gliders (or something) hitting the fence.  If an ordinary 5 cell glider then zap it.  If something else
// return true to stop the game.  If nothing interesting, return false
fn check_traps (  world: &mut  [[u8;NUM_Y]; NUM_X]) -> bool {

    let mut trapped: bool = false;


    let y = 2;                              // run the north trap line
    for x in 2 .. NUM_X - 2 {                   
        if world[x][y] != 0 {
            let top_left_x = x-2;
            let top_left_y = y;
//            println! ("north trap line - top left x,y = {}  {}",top_left_x,top_left_y);
            if inspect_trap( world,top_left_x,top_left_y) {
                trapped = true;
            }

        }
    }
    let y = NUM_Y - 2;                              // run the south  trap line
    for x in 2 .. NUM_X - 2 {                   
        if world[x][y] != 0 {
            let top_left_x = x-2;
            let top_left_y = y-4;
//            println! ("south trap line -  left x,y = {}  {}",top_left_x,top_left_y);

            if inspect_trap( world,top_left_x,top_left_y) {
                trapped = true;
            }
        }
    }
    let x = 2;                                      // run the east trap line;                              // run the south  trap line
    for y in 2 .. NUM_Y - 2 {                   
        if world[x][y] != 0 {
            let top_left_x = x;
            let top_left_y = y-2;
            if inspect_trap( world,top_left_x,top_left_y) {
                trapped = true;
            }
}
    }  
    let x = NUM_X - 2;                                      // run the west trap line;                              // run the south  trap line
    for y in 2 .. NUM_Y - 2 {                   
        if world[x][y] != 0 {
            let top_left_x = x-4;
            let top_left_y = y-2;
 //            println! ("west trap linetop left x,y = {}  {}",top_left_x,top_left_y);
        if inspect_trap( world,top_left_x,top_left_y) {
                trapped = true;
            }            
        }
    }         

    trapped   
}

// Inspect the trapped item to see if it is ordinary 5 element glider. If so, zap it and return false.
// If not a common beast, leave it alone and return true
fn inspect_trap (  world: &mut [[u8;NUM_Y]; NUM_X] ,top_left_x:usize,top_left_y:usize) -> bool {

    let mut interesting = false;

    let mut count = 0;
    for y in top_left_y .. top_left_y + 5 {
        for x in top_left_x .. top_left_x + 5 {
            if world[x][y] > 0 {
                count += 1;
            }
        }
    }
    
    if count == 5 {
        //println! ("Common small glider found");
        for y in top_left_y .. top_left_y + 5 {               // common fare - zap it
            for x in top_left_x .. top_left_x + 5 {
                 world[x][y] = 0;
            }
        }
    } else if count > 0 {
        interesting = true;
    }

    interesting
}




// generate the next generation based on the rules of the game of life
fn generation ( world: & [[u8;NUM_Y]; NUM_X]) -> [[u8;NUM_Y]; NUM_X] {

    let mut new_world = [[0u8;NUM_Y]; NUM_X];

    for  y in 0 .. NUM_Y  {
        for x in 0.. NUM_X {
            let mut count = 0;
            if x > 0 {
                count = count + world[x-1][y];
            }
            if x > 0 && y >0 {
                count = count + world[x-1][y-1];
            }
            if x > 0 &&  y < NUM_Y-1{
                count = count + world[x-1][y+1];
            }
            if x < NUM_X - 1 && y > 0{
                count = count + world[x+1][y-1];
            }
            if x < NUM_X - 1 {
                count = count + world[x+1][y];
            }
            if x < NUM_X - 1 && y < NUM_Y - 1{
                count = count + world [x+1][y+1];
            }
            if y > 0 {
                count = count + world[x][y-1];
            }
            if y < NUM_Y - 1 {
                count = count + world[x][y+1];
            }

            new_world[x][y] = 0;

            if (count <3) && world[x][y] == 1{
                new_world[x][y] = 0
            }
            if world[x][y] == 1 && (count ==2) || count == 3 {
                new_world[x][y] = 1;
            }
            if (world[x][y] == 0) && (count ==3) {
                new_world[x][y] = 1;
            }
        }    
    }    
    new_world
}
// look for (semi) static history of census data by looking for repeating periods
fn look_for_static (history:[i32;HISTORY_MAX]) -> bool {


    // Go through the census delta array to see if population is (semi) stable
    let mut period_1: bool = true;
    for  i in 0 .. HISTORY_MAX - 1 {
        if history[i+1] != history [i] {
            period_1 = false;
        }
    }  
    let mut period_2: bool = true;
    for  i in 0 .. HISTORY_MAX - 2 {
        if history[i+2] != history [i] {
            period_2 = false;
        }
    } 
    let mut period_3: bool = true;
    for  i in 0 .. HISTORY_MAX - 3 {
        if history[i+3] != history [i] { 
            period_3 = false;
        }
    } 
    let mut period_4: bool = true;
    for  i in 0 .. HISTORY_MAX - 4 {
        if history[i+4] != history [i] {
            period_4 = false;
        } 
    } 
    let mut period_5: bool = true;
    for  i in 0 .. HISTORY_MAX - 5 {
        if history[i+5] != history [i] {
            period_5 = false;
        } 
    } 
    let mut period_6: bool = true;
    for  i in 0 .. HISTORY_MAX - 6 {
        if history[i+6] != history [i] {
            period_6 = false;
        } 
    } 
    //println!(" {}  {}  {}  {}  {}  {}",period_1,period_2,period_3,period_4,period_5,period_6);

    period_1 | period_2 | period_3 | period_4 | period_5 | period_6
}


fn main() -> Result<(), String> {

// Open the desired file

    let mut input = String::new();
    let mut letter: char = ' ';
    let mut preview = true;
    let mut file_name = String::new();

    println!("Can either read from preview or saved file");
    println!("Enter p for preview or s for saved file");
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            letter = input.to_lowercase().chars().nth(0).unwrap();
        }
        Err(error) => {
            println!("Didn't get any input");
        }
    }

    if letter == 'p' {
        file_name =String::from( "preview.dat");
        preview = true;
    } else if letter == 's' {
        file_name =String::from( "saved.dat");
        preview = false;
    } else {
        println! (" Invalid input, need either p for preview or s for saved review mode");
    }
    let mut file = OpenOptions::new().read(true).open(file_name).unwrap();

// Set up the display window and canvas

    let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window
    ("Game of Life   Press esc to exit, up to speed up, down to slow down,left to pause,right to resume, r to restart same game",
     X_MAX.try_into().unwrap(), Y_MAX.try_into().unwrap())
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    

    let mut canvas = window.into_canvas().present_vsync().build()
        .expect("could not make a canvas");

        canvas.set_draw_color(Color::RGB(200,205,200));     // very pale green
        canvas.clear();

    let mut census_count_history = [0i32;HISTORY_MAX];                // used to detect (almost) stable population 
    let mut census_size_history = [0i32;HISTORY_MAX]; 

    let mut world = [[0u8;NUM_Y]; NUM_X];
    let mut starting_world = [[0u8;START_SIZE];START_SIZE];


    let mut world_number = 0i32;

    let mut paused: bool = false;
    let mut display = true;
    let mut event_pump = sdl_context.event_pump()?;
    let mut frames_per_second: u32 = 10;




    'read: loop {
        starting_world = read_starting_world(&mut file);
        world = restore_starting_world(starting_world);
        world_number += 1;
        println! ("World number  {}",world_number); 

            'running: loop {

                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit {..} |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {  // look for exit
                            break 'read;
                        },
                        Event::KeyDown { keycode: Some(Keycode::Space), ..}  => {   // Start new world on space
                            starting_world = read_starting_world(&mut file);
                            world = restore_starting_world(starting_world);
                            world_number += 1;
                            println! ("World number  {}",world_number);
                            paused = false;
                        },
                        Event::KeyDown { keycode: Some(Keycode::D), ..}  => {   // toggle the display
                            display = !display;                                
                        }, 
                        Event::KeyDown { keycode: Some(Keycode::R), ..}  => {   // re-start the world on R
                            world = restore_starting_world(starting_world);
                            paused = false;
                        }, 
                        Event::KeyDown { keycode: Some(Keycode::S), ..}  => {   // save the world on S
                           if preview {  // can't save if we aren't previewing
                                 write_starting_world(starting_world);
                            } 
                        },                                     
                        Event::KeyDown { keycode: Some(Keycode::Left), ..}  => {    // pause on left arrow
                            paused = true;
                        }, 
                        Event::KeyDown { keycode: Some(Keycode::Right), ..}  => {   // un-pause on right arrow
                            paused = false;
                            display = true;
                        }, 
                        Event::KeyDown { keycode: Some(Keycode::Up), ..}  => {      // run faster on up arrow
                            frames_per_second += 1;
                        },                                         
                        Event::KeyDown { keycode: Some(Keycode::Down), ..}  => {    // run slower on down arrow
                            frames_per_second = 1;
                        },                                          
                        _ => {}
                    }
                }

                if !paused {                
                    world = generation(&world);
                    if check_traps(&mut world) {        // if we found something interesting
                        paused = true;
                        display = true;
                    }

                    let (census_count , census_size ) = census (world);
                    for i in 0 .. HISTORY_MAX -1 {
                        census_count_history[HISTORY_MAX - i -1] = census_count_history[HISTORY_MAX- i - 2];
                    }
                    census_count_history [0] = census_count;
                    //println!(" census count history  {:?}",census_count_history); 

                    let census_size_i32 = census_size.try_into().unwrap();
                    for i in 0 .. HISTORY_MAX -1 {
                        census_size_history[HISTORY_MAX - i -1] = census_size_history[HISTORY_MAX- i - 2];
                    }
                    census_size_history [0] = census_size_i32;
                    //println!(" census size history  {:?}",census_size_history); 

                    if look_for_static(census_count_history) && look_for_static(census_size_history){
                       paused = true;
                       display = true;

                    }
                } // end of paused section   

                    // show the display if display enabled  
                    if display {
                        canvas.clear();
                        // display the world
                        let i_size: i32 = SIZE.try_into().unwrap();
                        let mut y_pos: i32= 0;
                            for  y in 0 .. NUM_Y {
                                let mut x_pos = 0;
                                for x in 0.. NUM_X {
                                    if world[x][y] != 0 {

                                        canvas.set_draw_color(Color::RGB(0,0,0));           
                                    } else {
                                        canvas.set_draw_color(Color::RGB(200,205,200));   // very pale green        
                                    }
                                // A draw a rectangle in the cell
                                canvas.fill_rect(Rect::new(x_pos+1, y_pos+1, (SIZE-2).try_into().unwrap(), (SIZE-2).try_into().unwrap())).expect("Couldn't build rectangle");   
                                x_pos =x_pos + i_size;
                                }
                    
                            y_pos = y_pos + i_size;
                            }

                            canvas.present();
                        
                        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / frames_per_second));    
                    } // end of display section

            } // end of game loop
    } // end of read loop
    Ok(())
}
