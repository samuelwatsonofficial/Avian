use std::sync::Arc;
use std::str::FromStr;
use std::io::{self,BufRead};

const full_angle: f64 = 6.28318530718;

pub struct Arm
{
    arm_length: f64,
    pub bone_lengths: Vec<f64>,
    coefficient_a: f64,
    coefficient_b: f64,
    coefficient_c: f64,
    max_angle: f64,
}
impl Arm 
{
    pub fn new(bone_lengths:Vec<f64>) -> Arm
    {
        Arm
        {
        arm_length: bone_lengths.iter().sum(),
        bone_lengths: bone_lengths.clone(),
        coefficient_a: 1.,
        coefficient_b: 1.,
        coefficient_c: 1.,
        max_angle: full_angle/(bone_lengths.len() as f64),
        }
    }
    pub fn sumofwaves(bone_lengths:&Vec<f64>, angle: f64) -> f64
    {
        let mut sin_sum:f64 = 0.;
        let mut cos_sum:f64 = 0.;
        let mut i = 1.0;
        for length in bone_lengths
        {
            
            sin_sum+=(i*angle).sin()*length;
            cos_sum+=(i*angle).cos()*length;
            i+=1.0;
        }
        return (sin_sum.powf(2.)+cos_sum.powf(2.)).sqrt();
    }
    pub fn final_position(arm:&Arm,angle:f64) -> (f64, f64)
    {
        let mut sin_sum:f64 = 0.;
        let mut cos_sum:f64 = 0.;
        let mut i = 1.0;
        for length in &arm.bone_lengths
        {
            
            sin_sum+=(i*angle).sin()*length;
            cos_sum+=(i*angle).cos()*length;
            i+=1.0;
        }
        return (sin_sum,cos_sum)
    }
    pub fn solver(arm:&Arm,distance:f64) -> f64
    {
        let mut tempangle = arm.max_angle.clone();
        //println!("{}",Self::sumofwaves(&arm.bone_lengths,tempangle-(tempangle/((2<<1)as f64))));
        for i in 0..20
        {
            if distance > Self::sumofwaves(&arm.bone_lengths,tempangle-(arm.max_angle.clone()/((2<<i)as f64)))
            {
                //println!("tempangle is {} and must be reduced by {}",tempangle, arm.max_angle.clone()/((2<<i)as f64));
                tempangle -= arm.max_angle.clone()/((2<<i)as f64);
            }
        }
        tempangle
    }
   


}
fn main() 
{

}
