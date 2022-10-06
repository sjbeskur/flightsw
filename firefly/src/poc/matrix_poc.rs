use nalgebra as na;
use log::*;
use na::{SMatrix, Vector3, Matrix3};

type Matrix3x3 = SMatrix<i32, 3, 3>;

pub fn mult3x3s(){
    let my3x3 = Matrix3x3::new(11, 12, 13,
        21, 22, 23,
        31, 32, 33,);
    info!("{}", my3x3);
    info!("{}", my3x3 * my3x3);

    let coeff = 5;
    let r = my3x3 * 5;
    info!("\n{} * {} = {}", coeff, my3x3,r);

}

pub fn mult_vec3x_mat3(){
    let _v3 = Vector3::new(1, 2, 3);
    //Matrix3x3::zero();
    let _my3x3 = Matrix3x3::new(11, 12, 13,
        21, 22, 23,
        31, 32, 33,);

    //info!("{}", my3x3 * v3);

    let v: Vector3<f64> = na::zero();
    let m: Matrix3<f64> = na::one();
    info!("v = {}", v);
    info!("m ={}", m);

    let mxv = m * v;   // matrix-vector multiplication.
    info!("{}", mxv);
    
    //let vxm = v * m;   // this blows up...communitive property of multiplicaton not worky here :D

    let mxm = m * m;   // matrix-matrix multiplication.
    info!("{}", mxm);

    let vx2 = v * 2.0; // vector-scalar multiplication.
    info!("{}",vx2);
}