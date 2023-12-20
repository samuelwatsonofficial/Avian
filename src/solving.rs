use futures::executor::block_on;
extern crate queues;
use queues::*;
struct Practical{
    name:String,
    equipment: Vec<String>,
    method:Queue<(String,i32)>,
    risks:Vec<String>,
    independent_variable:String,
    dependent_variable:String,
    control_variable:String,
    interval:i32,
    limit:i64,
}
async fn HelloWorld(){
    println!("heloo")
}
fn main() {
    //let future = HelloWorld();
    //block_on(future);
    const thermal_insulation:Practical=Practical {
        name: String::from("Thermal insulation"),
        equipment: vec![String::from("100cm^3 beaker"),String::from("250cm^3 beaker"),String::from("thermometer"),String::from("kettle"),String::from("piece of cardboard"),String::from("scissors"),String::from("stopwatch"),String::from("insulating material"),String::from( "rubber bands")],
        method: queue![(String::from("making hole in cardboard"),10),(String::from("boiling kettle"),60)],
        risks: vec!["be careful with scissors","don't burn yourself on boiling water","put cardboard lid on large beaker","put thermometer through hole",],
        String::from("material being used as insulator"),
        String::from("material being used as insulator"),
        String::from("tAemperature of water"),
        };
}