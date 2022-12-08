// https://doc.rust-lang.org/book/ch10-02-traits.html
// https://blog.logrocket.com/5-rust-game-engines-consider-next-project/
// https://blog.logrocket.com/how-to-build-a-blockchain-in-rust/
// https://www.uc.edu/cdc/urban_database/food_resources/NUTRITION-LESSON-2-NUTRIENT0&FOOD-LABEL-FACTS.htm
// https://www.researchgate.net/publication/286184266_Micronutrient_interrelationships_Synergism_and_antagonism
// https://www.reddit.com/r/rust/comments/3fimgp/comment/hx83bsz/?utm_source=share&utm_medium=web2x&context=3
// https://www.reddit.com/r/rust/comments/zg55sj/rust_is_easy_the_compiler_teaches_you/?utm_source=share&utm_medium=web2x&context=3
// https://www.reddit.com/r/rust/comments/zg3xg0/handling_minecraftlike_worlds_in_a_rusty_way/

enum Task{
    name(String), cooldown, timeAllowance, autoAddMorning, autoAddEvening
}

impl Task{
    fn getName(&self){
        
    }
}

struct{
    
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
