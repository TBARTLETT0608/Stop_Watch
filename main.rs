/***************************************************************************
// What does this application do?
// Keeps track of real-time using interrupts and timers
***************************************************************************/

#![no_std]
#![no_main]

/***********************************************************************************************************
                                       // Crate Imports
***********************************************************************************************************/
use esp_backtrace as _;
use esp_hal::
{
    prelude::*,
    peripherals::TIMG0,
    delay::MicrosDurationU64,
    timer::timg::{TimerGroup, Timer, Timer0},
    gpio::{Event, Input, Pull, Io, Level, Output},
    interrupt::Priority,
};
use esp_println::println;
use core::cell::{Cell, RefCell};
use critical_section::Mutex;


/***********************************************************************************************************
                                        // Type Definitions
***********************************************************************************************************/
// Need to create a struct that keeps track of time.
struct Time
{
    seconds: u32,
    minutes: u32,
    hours: u32,
}

#[entry]
fn main() -> ! 
{
    /***********************************************************************************************************
                                       // Peripheral Configuration //
    ***********************************************************************************************************/
    // Take Peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default());
    // Instantiate Timer Group 0
    let timer_group0 = TimerGroup::new(peripherals.TIMG0);
    // Instantiate Timer0 in Timer Group 0
    let timer0 = timer_group0.timer0;


    /***********************************************************************************************************
                                    // Button Input Configurations //
    ***********************************************************************************************************/
    // Instantiate and Create Handle for IO
    let mut io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    // Configuration (Start Button)
    let mut button_start = Input::new(io.pins.gpio1, Pull::Up);


    // Configuration (Stop Button)
    let mut button_stop = Input::new(io.pins.gpio5, Pull::Up);


    // Configuration (Reset Button)
    let mut button_reset = Input::new(io.pins.gpio9, Pull::Up);

    /***********************************************************************************************************
                                           // Application Logic //
    ***********************************************************************************************************/
    // Set up a Time struct to keep track of time
    let mut t = Time 
    {
        seconds: 0_u32,
        minutes: 0_u32,
        hours: 0_u32,
    };

    let mut start = timer0.now();
    timer0.start();

    loop 
    {
        // Start the timer and increment time every one second
        if button_start.is_low()
        {
            while button_stop.is_high() && button_reset.is_high()
            {
                if timer0.now().checked_duration_since(start).unwrap().to_secs() >= 1
                {
                    t.seconds = t.seconds.wrapping_add(1);

                    if t.seconds > 59
                    {
                        t.minutes += 1;
                        t.seconds = 0;
                    }
                    if t.minutes > 59
                    {
                        t.hours += 1;
                        t.minutes = 0;
                    }
                    if t.hours > 23
                    {
                        t.seconds = 0;
                        t.minutes = 0;
                        t.hours = 0;
                    }
                    start = timer0.now();
                    println!("Time Elapsed {:0>2}:{:0>2}:{:0>2}", t.hours, t.minutes, t.seconds);
                }
            }
        }
        // Stop by doing nothing if stop button is pressed
        if button_stop.is_low()
        {
            
        }
        // Reset values of the time struct if reset button is pressed
        if button_reset.is_low()
        {
            t.seconds = 0;
            t.minutes = 0;
            t.hours = 0;
        } 
    }
}
