// https://doc.rust-lang.org/book/ch10-02-traits.html

enum Task{
    name(String), cooldown, timeAllowance, autoAddMorning, autoAddEvening
}

impl Task{
    fn getName(&self){
        
    }
}

fn main() {
    let (eins, zwei, eat) = (5, 60, 20);
    let (teeth, shave, shower, dress) = (5, 15, 30, 15);
    let (run, lift, pack, sleep, work) = (30, 60, 15, (10*60), (10.5*60));
    let (laundryStart, laundryTransfer, laundryFinish, loadDuration) = (10, 10, 10, 60);
    let (trash, dishes) = (15, 15);
    
    let transit = 5;
    
    let sleepMin = 8*60;
    let sleepMax = 10*60;
    
    // get system current time
    // AM/PM? Just woke up, or end of day?
    // assign start time, end time
    // checkbox list of things to do
    let awakening = [eins, zwei, laundryStart, pack, teeth, shave, shower, dress, (transit*8)];
    
    // USE ENUMS https://www.tutorialspoint.com/rust/rust_enums.htm
    
    // all of these should be objects tbh...
    // async bool OR cooldown (must be returned to AFTER, not before, but AFTER x minutes)
    // time allotted
    // autoAM / autoPM
    // transit
    // .with(task, task, task) <- calls all their transits/preps/etc
}
