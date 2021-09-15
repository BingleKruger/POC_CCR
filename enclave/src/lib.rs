// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate sgx_types;
extern crate serde_json;
extern crate serde;
extern crate sgx_tseal;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate hex;
extern crate rustc_serialize;
extern crate rusty_machine;
extern crate serde_cbor;
extern crate sgx_rand;



use sgx_types::*;
use sgx_types::marker::ContiguousMemory;
use sgx_tseal::{SgxSealedData};
use std::mem::size_of;
use std::io::{Write, Read};
use std::slice;
use std::str;
use serde::{Serialize, Deserialize};
use std::time::*;
use std::untrusted::time::{InstantEx};
use std::sgxfs::SgxFile;
use std::vec::Vec;
use rusty_machine::linalg::{Matrix, Vector, BaseMatrix};
use rusty_machine::learning::SupModel;
use rusty_machine::learning::lin_reg::LinRegressor;
use std::ops::Deref;
use std::string::String;


pub const DATAFILE: &'static str = "sealed.json";

//Deserialize - Used to parse the JSON to struct
#[derive(Debug, Deserialize)]
pub struct User {
    pub ID: sgx_tstd::string::String,
    pub JobTitle: sgx_tstd::string::String,
    pub EmailAddress: sgx_tstd::string::String,
    pub FirstNameLastName :sgx_tstd::string::String,
    pub Field1: sgx_tstd::string::String,
    pub Field2: sgx_tstd::string::String,
    pub Field3: sgx_tstd::string::String,
    pub Field4: sgx_tstd::string::String,
    pub Field5: sgx_tstd::string::String,
    pub Field6: sgx_tstd::string::String
}

#[derive(Debug, Deserialize)]
pub struct Client {
    pub wage: f64,
    pub educ: f64
}

#[derive(Debug, Deserialize)]
pub struct Client_Multiple {
    pub colGPA: f64,
    pub hsGPA: f64,
    pub ACT: f64
}

#[derive(Debug, Deserialize)]
pub struct Client_Time {
    pub lchnimp: f64,
    pub lgas: f64,
    pub lrtwex: f64,
    pub lchempi: f64,
    pub befile6: f64,
    pub affile6: f64,
    pub afdec6: f64,
}

#[derive(Debug, Deserialize)]
pub struct Client_Time_Month {
    pub lchnimp: f64,
    pub lgas: f64,
    pub lrtwex: f64,
    pub lchempi: f64,
    pub befile6: f64,
    pub affile6: f64,
    pub afdec6: f64,
    pub feb: f64,
    pub mar: f64,
    pub apr: f64,
    pub may: f64,
    pub jun: f64,
    pub jul: f64,
    pub aug: f64,
    pub sep: f64,
    pub oct: f64,
    pub nov: f64,
    pub dec: f64
}

#[derive(Debug, Deserialize)]
pub struct Client_Panel {
    pub kids: f64,
    pub educ: f64,
    pub age: f64,
    pub agesq: f64,
    pub black: f64,
    pub east: f64,
    pub northcen: f64,
    pub west: f64,
    pub farm: f64,
    pub othrural: f64,
    pub town: f64,
    pub smcity: f64,
    pub y74: f64,
    pub y76: f64,
    pub y78: f64,
    pub y80: f64,
    pub y82: f64,
    pub y84: f64
}

#[derive(Serialize, Debug, Deserialize, Copy, Clone)]
pub struct Client_Sealing_One {
    pub colGPA: f64,
    pub hsGPA: f64,
    pub ACT: f64
}

#[derive(Serialize, Debug, Deserialize, Copy, Clone)]
pub struct Client_Unsealing_One {
    pub colGPA: f64,
    pub hsGPA: f64,
    pub ACT: f64
}

#[derive(Serialize, Debug, Deserialize, Copy, Clone)]
pub struct Client_Sealing_Two {
    pub colGPA: f64,
    pub hsGPA: f64,
    pub ACT: f64
}

#[derive(Serialize, Debug, Deserialize, Copy, Clone)]
pub struct Client_Unsealing_Two {
    pub colGPA: f64,
    pub hsGPA: f64,
    pub ACT: f64
}

#[derive(Debug, Deserialize)]
pub struct Student {
    pub DOF: f64,
    pub X1: f64,
    pub X2: f64,
    pub X3: f64,
    pub X4: f64,
    pub X5: f64,
    pub X6: f64,
    pub X7: f64,
    pub X8: f64,
    pub X9: f64,
    pub X10: f64,
    pub X11: f64,
    pub X12: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub ID: sgx_tstd::string::String,
    pub Name: sgx_tstd::string::String,
    pub Surname: sgx_tstd::string::String,
    pub SEX: f64,
    pub EDUCATION: f64,
    pub MARRIAGE: f64,
    pub AGE: f64,
    pub Phone: sgx_tstd::string::String,
    pub Email: sgx_tstd::string::String,
    pub Street: sgx_tstd::string::String,
    pub City: sgx_tstd::string::String,
    pub Postal: f64,
    pub Loan: f64,
    pub Interest: f64,
    pub Term: f64,
    pub Income: f64,
}

//A function that is used to calculate a range for the p-value
//The function requires the t value, number of rows and t matrix
//The function returns a string
fn p_value(mut t_value: f64, nrow: usize, t_matrix: &Matrix<f64>) -> sgx_tstd::string::String {
    let t_row : Matrix<f64>;
    if nrow <= 102 {
        t_row = t_matrix.select_rows(&[nrow - 3]);
    } else if nrow <= 502 {
        t_row = t_matrix.select_rows(&[100]);
    } else if nrow <= 1002 {
        t_row = t_matrix.select_rows(&[101]);
    } else if nrow <= 5002 {
        t_row = t_matrix.select_rows(&[102]);
    } else if nrow <= 10002 {
        t_row = t_matrix.select_rows(&[103]);
    } else if nrow <= 30002 {
        t_row = t_matrix.select_rows(&[104]);
    } else if nrow <= 50002 {
        t_row = t_matrix.select_rows(&[105]);
    } else if nrow <= 100002 {
        t_row = t_matrix.select_rows(&[106]);
    } else {
        t_row = t_matrix.select_rows(&[107]);
    }


    let t_data = t_row.data().deref()[..].to_vec();

    // println!("t-stat: {:?}", t_data);
    // println!("t-val: {:?}", t_value);

    let mut t_string: sgx_tstd::string::String = String::from("placeholder");

    for i in 1..12 {
        if t_value < 0.0 {
            t_value = t_value.abs();
        }
        if t_value <  t_data[1] {
            t_string =  String::from("p > 0.5");
            break
        } else if ( t_data[i] < t_value) && (t_data[i+1] > t_value) {
            if i == 1 {
                t_string =  String::from("0.2 < p < 0.5");
                break
            } else if i == 2 {
                t_string =  String::from("0.1 < p < 0.2");
                break
            } else if i == 3 {
                t_string =  String::from("0.05 < p < 0.1");
                break
            } else if i == 4 {
                t_string =  String::from("0.02 < p < 0.05  [Significant]");
                break
            } else if i == 5 {
                t_string =  String::from("0.01 < p < 0.02  [Significant]");
                break
            } else if i == 6 {
                t_string =  String::from("0.004 < p < 0.01  [Significant]");
                break
            } else if i == 7 {
                t_string =  String::from("0.002 < p < 0.004  [Significant]");
                break
            } else if i == 8 {
                t_string =  String::from("0.0005 < p < 0.002  [Significant]");
                break
            } else if i == 9 {
                t_string =  String::from("0.0004 < p < 0.001  [Significant]");
                break
            } else if i == 10 {
                t_string =  String::from("0.0002 < p < 0.0004  [Significant]");
                break
            }  else if i == 11 {
                t_string =  String::from("0.0001 < p < 0.0002  [Significant]");
                break
            }
        } else if t_value >  t_data[12] {
            t_string =  String::from("p < 0.0001  [Significant]");
            break
        }
    }
    return t_string
}

//A function that is used to calculate the standard error, t value and p value
fn ols_stats(nrow: usize, ncol: usize, input_vec: Vec<f64> , input_matrix: Matrix<f64>, targets: Vector<f64>, model: &LinRegressor, t_matrix: &Matrix<f64>) -> (Vec<f64>, Vec<f64>, Vec<String>) {

    let parameters = model.parameters().unwrap().data();
    // println!("The model parameters are: {:?}", parameters);

    let output = model.predict(&input_matrix).unwrap();
    // println!("Predicted: {:?}", output);
    // println!("Actual: {:?}", targets);

    let sigma: f64;
    let mut numerator: f64 = 0.0; //top
    let denominator: f64 = (nrow - ncol) as f64; //Regression: nrow(X) - ncol(X)
    let mut xvec: Vec<f64> = sgx_tstd::vec::Vec::new();

    for i in 0..nrow {
        numerator = numerator + (targets[i] - output[i]) * (targets[i] - output[i]);
        xvec.push(1.0);
    }

    xvec.append(&mut input_vec.clone());

    let X = Matrix::new(ncol, xvec.len() / ncol, xvec);

    sigma = numerator / denominator;

    // println!("Sigma: {:?}", sigma);

    let c = &X * &X.transpose(); // Matrix product of a and b

    let inverse = c.inverse().unwrap();

    // println!("Inverse: {:?}", inverse);

    // No simple way to iterate over matrix diagonal, only to copy it into a new Vector
    let diagonal_matrix: Vec<_> = inverse.diag().cloned().collect();

    let mut standard_error: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut t_value: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut p_val: Vec<String> = sgx_tstd::vec::Vec::new();


    for i in 0..diagonal_matrix.len() {
        standard_error.push((sigma * diagonal_matrix[i]).sqrt());
        t_value.push(parameters[i] / standard_error[i]);
        p_val.push(p_value(t_value[i], nrow, &t_matrix))
    }

    // println!("SE: {:?}", standard_error);
    // println!("The t-values are: {:?}", t_value);
    // println!("The p-values are: {:?}", p_val);

    return (standard_error, t_value, p_val)
}

//A function that is used to determine the model parameters for simple linear regression
//The function requires the input vector and the target vector
//The function returns a result
fn simple_ols(inputs_vec: sgx_tstd::vec::Vec<f64>, targets_vec: sgx_tstd::vec::Vec<f64>) -> Result<(), sgx_tstd::boxed::Box<sgx_tstd::fmt::Error>> {
    println!();
    println!("Simple linear regression (OLS):");

    let inputs = Matrix::new(inputs_vec.len(), 1, inputs_vec.clone());
    let targets = Vector::new(targets_vec.clone());

    let model = train_model(inputs.clone(), targets.clone());

    let json_t =  sgx_tstd::path::Path::new("../data/studentT.json");
    let file_t =  sgx_tstd::untrusted::fs::File::open(json_t).expect("file not found");
    let students:sgx_tstd::vec::Vec<Student> = serde_json::from_reader(file_t).expect("Error while reading or parsing");

    let mut DOF: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X1: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X2: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X3: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X4: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X5: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X6: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X7: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X8: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X9: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X10: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X11: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X12: Vec<f64> = sgx_tstd::vec::Vec::new();

    //Note test is upper tail, multiply values by two to get actual prob
    for student in students {
        DOF.push(student.DOF); // Degrees of freedom
        X1.push(student.X1); //0.25
        X2.push(student.X2); //0.1
        X3.push(student.X3); // 0.05
        X4.push(student.X4); //0.025
        X5.push(student.X5); //0.01
        X6.push(student.X6); //0.005
        X7.push(student.X7); //0.002
        X8.push(student.X8); //0.001
        X9.push(student.X9); //0.0005
        X10.push(student.X10); //0.0002
        X11.push(student.X11); //0.0001
        X12.push(student.X12); //0.00005
    }

    DOF.append(&mut X1);
    DOF.append(&mut X2);
    DOF.append(&mut X3);
    DOF.append(&mut X4);
    DOF.append(&mut X5);
    DOF.append(&mut X6);
    DOF.append(&mut X7);
    DOF.append(&mut X8);
    DOF.append(&mut X9);
    DOF.append(&mut X10);
    DOF.append(&mut X11);
    DOF.append(&mut X12);

    let t_matrix = Matrix::new(13, DOF.len() / 13, DOF).transpose();

    let ols_statistics = ols_stats(targets_vec.len(), 2, inputs_vec, inputs, targets, &model, &t_matrix);

    let parameters = model.parameters();

    // println!("The model parameters are: {:?}", model.parameters());

    match parameters {
        Some(x) => {
            println!("The intercept is: {:.6}          SE: {:.6}     t-value: {:.6}     {}", x[0], ols_statistics.0[0], ols_statistics.1[0], ols_statistics.2[0]);
            println!("The 1st coefficient is: {:.6}    SE: {:.6}     t-value: {:.6}     {}", x[1], ols_statistics.0[1], ols_statistics.1[1], ols_statistics.2[1]);
            println!();
        },
        None => println!("Error: Cannot determine model parameters"),
    }

    Ok(())
}

//A function that is used to determine the model parameters for multiple linear regression
//The function requires the input matrix (in the correct format - remember to transpose matrix) and the target vector
//The function returns a result
fn multiple_ols(inputs_mat: Matrix<f64>, targets_vec: sgx_tstd::vec::Vec<f64>) -> Result<(), sgx_tstd::boxed::Box<sgx_tstd::fmt::Error>> {
    println!("Multiple linear regression (OLS):");

    let ncol = inputs_mat.clone().col_iter().len();

    let inputs_vec = inputs_mat.clone().transpose().data().to_vec();

    let targets = Vector::new(targets_vec.clone());

    let model_multiple = train_model(inputs_mat.clone(), targets.clone());

    let json_t =  sgx_tstd::path::Path::new("../data/studentT.json");
    let file_t =  sgx_tstd::untrusted::fs::File::open(json_t).expect("file not found");
    let students:sgx_tstd::vec::Vec<Student> = serde_json::from_reader(file_t).expect("Error while reading or parsing");

    let mut DOF: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X1: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X2: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X3: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X4: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X5: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X6: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X7: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X8: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X9: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X10: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X11: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut X12: Vec<f64> = sgx_tstd::vec::Vec::new();

    //Note test is upper tail, multiply values by two to get actual prob
    for student in students {
        DOF.push(student.DOF); // Degrees of freedom
        X1.push(student.X1); //0.25
        X2.push(student.X2); //0.1
        X3.push(student.X3); // 0.05
        X4.push(student.X4); //0.025
        X5.push(student.X5); //0.01
        X6.push(student.X6); //0.005
        X7.push(student.X7); //0.002
        X8.push(student.X8); //0.001
        X9.push(student.X9); //0.0005
        X10.push(student.X10); //0.0002
        X11.push(student.X11); //0.0001
        X12.push(student.X12); //0.00005
    }

    DOF.append(&mut X1);
    DOF.append(&mut X2);
    DOF.append(&mut X3);
    DOF.append(&mut X4);
    DOF.append(&mut X5);
    DOF.append(&mut X6);
    DOF.append(&mut X7);
    DOF.append(&mut X8);
    DOF.append(&mut X9);
    DOF.append(&mut X10);
    DOF.append(&mut X11);
    DOF.append(&mut X12);

    let t_matrix = Matrix::new(13, DOF.len() / 13, DOF).transpose();

    let ols_statistics = ols_stats(targets_vec.len(), ncol+1, inputs_vec, inputs_mat, targets, &model_multiple, &t_matrix);


    let parameters_multiple = model_multiple.parameters();

    // println!("MLR model parameters: {:?} ", model_multiple);

    match parameters_multiple {
        Some(x) => {
            let parameters = x.clone().into_vec();
            // println!("The output from the model is: {:?}", parameters);
            for i in 1..parameters.len()+1 {
                if i == 1 {
                    println!("The intercept is: {:.6}          SE: {:.6}     t-value: {:.6}     {}", x[0], ols_statistics.0[0], ols_statistics.1[0], ols_statistics.2[0]);
                } else if i == 2 {
                    println!("The 1st coefficient is: {:.6}    SE: {:.6}     t-value: {:.6}     {}", x[1], ols_statistics.0[1], ols_statistics.1[1], ols_statistics.2[1]);
                } else if i == 3 {
                    println!("The 2nd coefficient is: {:.6}    SE: {:.6}     t-value: {:.6}     {}", x[2], ols_statistics.0[2], ols_statistics.1[2], ols_statistics.2[2]);
                } else if i == 4 {
                    println!("The 3rd coefficient is: {:.6}    SE: {:.6}     t-value: {:.6}     {}", x[3], ols_statistics.0[3], ols_statistics.1[3], ols_statistics.2[3]);
                } else {
                    println!("The {}th coefficient is: {:.6}    SE: {:.6}     t-value: {:.6}     {}", i-1, x[i-1], ols_statistics.0[i-1], ols_statistics.1[i-1], ols_statistics.2[i-1]);
                }
            }
            println!();
        },
        None => println!("Error: Cannot determine model parameters"),
    }

    Ok(())
}

//A function that is used to train a linear model using ordinary least squares (OLS)
//The function requires the input matrix (in the correct format) and the target vector
//The function returns a linear model
fn train_model(inputs: Matrix<f64>, targets: Vector<f64>) -> LinRegressor {
    let mut lin_mod = LinRegressor::default();
    lin_mod.train(&inputs, &targets).unwrap();
    lin_mod
}


//A function that is used to calculate the summary statistics of a vector
//The function prints out the following statistics:
//Mean, Median (Median = Q2 = P50), P5, P50, P95, SD, Q1, Q2, Q3, IQR, Skewness (if any), number of observations
//The function only requires a vector
fn summary(numbers: &mut sgx_tstd::vec::Vec<f64>) {

    //Create a clone of the numbers vector received to ensure it does not change
    let mut numbers_cloned: sgx_tstd::vec::Vec<f64> = numbers.clone();

    //Sort the numbers_cloned from the smallest to the largest value
    numbers_cloned.sort_by(|a, b| a.partial_cmp(b).unwrap());

    //Find the index for Q1, Q2, Q3, P5 and P95
    //Assume n = number of observations
    //Index for percentile: (percentile) * n
    //Index for quartile: (quartile) * (n+1)
    let Q1_Index = 0.25 * (numbers_cloned.len() as i64 +1) as f64;
    let Q2_Index = 0.5 * (numbers_cloned.len() as i64 +1) as f64;
    let Q3_Index = 0.75 * (numbers_cloned.len() as i64 +1) as f64;
    let P5_Index = 0.05 * (numbers_cloned.len() as i64) as f64;
    let P95_Index = 0.95 * (numbers_cloned.len() as i64) as f64;

    //For Q1, Q2, Q3, P5, P50 and P95 the index is rounded to the nearest integer
    //The index is then used to find the datapoint in the vector that is sorted from smallest to largest
    //E.g. Q1 for 100 observations would be the 25th variable
    //E.g. Q2 for 100 observations would be the 50th variable
    //E.g. Q3 for 100 observations would be the 75th variable
    //E.g. P5 for 100 observations would be the 5th variable
    //E.g. P95 for 100 observations would be the 95th variable
    let Q1: f64 = numbers_cloned[(Q1_Index.round() -1.0) as usize];
    let Q2: f64 = mean(&vec![numbers_cloned[(Q2_Index.floor() -1.0) as usize], numbers_cloned[(Q2_Index.ceil() -1.0) as usize]]) as f64;
    let Q3: f64 = numbers_cloned[(Q3_Index.round() -1.0) as usize];
    let P5: f64 =  numbers_cloned[(P5_Index -1.0) as usize];
    let P95: f64 = numbers_cloned[(P95_Index.round() -1.0) as usize];

    println!("The total number of observations (n) is: {:?}", numbers_cloned.len());
    println!("P5 = {:.6}", P5);
    println!("Q1 = {:.6}", Q1);
    println!("Q2 = P50 = {:.6}", Q2);
    println!("Q3 = {:.6}", Q3);
    println!("P95 = {:.6}", P95);
    println!("IQR = {:.6}", Q3 - Q1);

    //Skewness is calculated as follow:
    //If Q3 - Q2 > Q2 - Q1 -- Positive skew
    //If Q3 - Q2 > Q2 - Q1 -- Negative skew
    //If Q3 - Q2 == Q2 - Q1 -- Symmetric
    if Q3 - Q2 > Q2 - Q1 {
        println!("The data is positively skewed");
    } else if Q3 - Q2 < Q2 - Q1 {
        println!("The data is negatively skewed");
    } else {
        println!("The data is symmetric");
    }

    //Calculate the mean of the vector
    let data_mean = mean(&numbers_cloned);
    println!("The sample mean is: {:.6}", data_mean);

    //Calculate the variance of the vector
    let variance = numbers_cloned.iter().map(|value| {
        let diff = data_mean - (*value as f64);
        diff * diff
    }).sum::<f64>() / (numbers_cloned.len() as f64 - 1.0);
    println!("The sample standard deviation is: {:.6}", variance.sqrt());
    println!();

}

//A function that is used to calculate the mean of a vector
//The function only requires a vector and returns a float
fn mean(numbers: &sgx_tstd::vec::Vec<f64>) -> f64 {
    let sum: f64 = numbers.iter().sum();
    sum as f64 / numbers.len() as f64

}

//A function that is used to seal data using SgxSealedData
//The function requires the data to be formatted as a Vec using the predefined struct: Client_Sealing_One
//The function returns a result
fn seal_method_one(unsealed_vector: sgx_tstd::vec::Vec<Client_Sealing_One>, sealed_cipher_vec_pointer: *mut u8) -> Result<(*mut u8, usize), sgx_status_t> {

    let encoded_vec = serde_cbor::to_vec(&unsealed_vector).unwrap();
    let encoded_slice = encoded_vec.as_slice();
    // println!("Length of encoded slice: {}", encoded_slice.len());
    // println!("Encoded slice: {:?}", encoded_slice);

    let aad: [u8; 0] = [0_u8; 0];
    let result = SgxSealedData::<[u8]>::seal_data(&aad, encoded_slice);
    let sealed_data = match result {
        Ok(x) => x,
        Err(ret) => {
            return Err(ret)
        },
    };

    // let mut sealed_cipher_vec: Vec<u8> = vec![0; SCRATCH_PAD_SIZE];
    // let sealed_cipher_vec_pointer: *mut u8 = &mut sealed_cipher_vec[0];

    let sealed_log_size = size_of::<sgx_sealed_data_t>() + encoded_slice.len();

    let opt = to_sealed_log_for_slice(&sealed_data, sealed_cipher_vec_pointer, sealed_log_size as u32);
    if opt.is_none() {
        println!("Error encountered: {}", sgx_status_t::SGX_ERROR_INVALID_PARAMETER);
    } else {
        println!("Data sealed successfully using SgxSeledData")
    }
    // println!("Option {:?}", opt);

    let sealed_encrypted_data = unsafe {
        slice::from_raw_parts(sealed_cipher_vec_pointer, sealed_log_size as usize)
    };

    println!("Length of the sealed data {:?}", sealed_encrypted_data.len());
    // println!("This is the sealed data {:?}", sealed_encrypted_data);
    Ok((sealed_cipher_vec_pointer, sealed_log_size))
}

//A function that is used to unseal data using SgxSealedData
//The function requires the sealed_log and sealed_log_size
//The function returns a result
fn unseal_method_one(sealed_log: *mut u8, sealed_log_size: usize) ->  Result<sgx_tstd::vec::Vec<Client_Unsealing_One>, sgx_status_t> {

    // Verify sealed data for serializable
    let opt_one = from_sealed_log_for_slice::<u8>(sealed_log, sealed_log_size as usize as u32);
    let sealed_data = match opt_one {
        Some(x) => {
            x
        },
        None => {
            return Err(sgx_status_t::SGX_ERROR_INVALID_PARAMETER);
        },
    };

    let result = sealed_data.unseal_data();
    let unsealed_data = match result {
        Ok(x) => x,
        Err(ret) => {
            return Err(ret);
        },
    };


    let encoded_slice = unsealed_data.get_decrypt_txt();
    println!("Length of encoded slice: {}", encoded_slice.len());
    // println!("Encoded slice: {:?}", encoded_slice);
    println!("Data unsealed successfully using SgxUnsealedData");
    return Ok(serde_cbor::from_slice(encoded_slice).unwrap());

}

//A function that is used to seal data using SgxFile
//The function requires the data to be formatted as a Vec using the predefined struct: Client_Sealing_Two
//The function returns a status
fn seal_method_two(unsealed_vector: sgx_tstd::vec::Vec<Client_Sealing_Two>) -> sgx_status_t {
    let encoded_vec = serde_cbor::to_vec(&unsealed_vector).unwrap();
    let encoded_slice = encoded_vec.as_slice();
    // println!("Length of encoded slice for SgxFile: {}", encoded_slice.len());
    // println!("Encoded slice for SgxFile: {:?}", encoded_slice);

    match SgxFile::create(DATAFILE) {
        Ok(mut f) => match f.write_all(encoded_slice) {
            Ok(()) => {
                println!("SgxFile write JSON file success!");
                println!("Data sealed successfully using SgxFile");
                sgx_status_t::SGX_SUCCESS
            }
            Err(x) => {
                println!("SgxFile write JSON file failed! {}", x);
                sgx_status_t::SGX_ERROR_UNEXPECTED
            }
        },
        Err(x) => {
            println!("SgxFile create file {} error {}", DATAFILE, x);
            sgx_status_t::SGX_ERROR_UNEXPECTED
        }
    }
}

//A function that is used to unseal data using SgxFile
//The function does not require any input
//The function returns an option containing a Vec of the struct: Client_Unsealing_Two
fn unseal_method_two() -> Option<Vec<Client_Unsealing_Two>> {
    let mut data_vec: Vec<u8> = Vec::new();

    let data_json_str : Option<Vec<Client_Unsealing_Two>> = match SgxFile::open(DATAFILE) {
        Ok(mut f) => match f.read_to_end(&mut data_vec) {
            Ok(len) => {
                println!("Read {} bytes from Key file", len);
                println!("Data unsealed successfully");
                let clients_unsealing_two:sgx_tstd::vec::Vec<Client_Unsealing_Two> = serde_cbor::from_slice(&data_vec).unwrap();
                Some(clients_unsealing_two)
            }
            Err(x) => {
                println!("Read keyfile failed {}", x);
                None
            }
        },
        Err(_) => {
            println!("get_sealed_pcl_key cannot open keyfile, please check that key is provisioned successfully!");
            None
        }
    };

    return data_json_str;

}

// Initialise a scratch pad for the enclave and define it's size
pub const MEGA_BYTE: usize = 100_000;
pub const SCRATCH_PAD_SIZE: usize = 50 * MEGA_BYTE;

// Use case 1 - Lending profiles
#[no_mangle]
pub extern "C" fn lending_profiles(some_string_postal: *const u8, some_len_postal: usize, some_string_term: *const u8, some_len_term: usize) -> sgx_status_t {

    //Receive the command from the untrusted code
    let str_slice_postal = unsafe { slice::from_raw_parts(some_string_postal, some_len_postal) };
    let str_slice_postal = str::from_utf8(&str_slice_postal).unwrap().parse::<f64>().unwrap();

    let str_slice_term = unsafe { slice::from_raw_parts(some_string_term, some_len_term) };
    let str_slice_term = str::from_utf8(&str_slice_term).unwrap().parse::<f64>().unwrap();

    // Sealing is done as a separate command

    // Unseal if sealed
    println!("Unsealing the CCR database");
    println!();
    let unsealed_data_ccr = unseal_method_two_ccr().unwrap();
    println!();


    let mut Loan_Filtered: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Interest_Filtered: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Term_Filtered: Vec<f64> = sgx_tstd::vec::Vec::new();

    for i in 0..unsealed_data_ccr.len() {

        if unsealed_data_ccr[i].Postal == str_slice_postal && unsealed_data_ccr[i].Term >= str_slice_term {
            Loan_Filtered.push(unsealed_data_ccr[i].Loan);
            Interest_Filtered.push(unsealed_data_ccr[i].Interest);
            Term_Filtered.push(unsealed_data_ccr[i].Term);
        }

    }

    println!("The privacy preserving summary statistics for the Loan variable is:");
    summary(&mut Loan_Filtered);
    println!("The privacy preserving summary statistics for the Interest variable is:");
    summary(&mut Interest_Filtered);
    println!("The privacy preserving summary statistics for the Term variable is:");
    summary(&mut Term_Filtered);
    sgx_status_t::SGX_SUCCESS
}

// Use case 2 - Credit health
#[no_mangle]
pub extern "C" fn credit_health() -> sgx_status_t {

    // Sealing is done as a separate command

    // Unseal if sealed
    println!("Unsealing the CCR database");
    println!();
    let unsealed_data_ccr = unseal_method_two_ccr().unwrap();
    println!();


    let mut Loan_2: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Income_2: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Sex_2: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Education_2: Vec<f64> = sgx_tstd::vec::Vec::new();

    let mut Sex_M: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Education_2_2: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Education_2_3: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Education_2_4: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Education_2_5: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Education_2_6: Vec<f64> = sgx_tstd::vec::Vec::new();

    let mut Loan0_50: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Income0_50: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Loan_To_Income0_50: Vec<f64> = sgx_tstd::vec::Vec::new();

    let mut Loan50_100: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Income50_100: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Loan_To_Income50_100: Vec<f64> = sgx_tstd::vec::Vec::new();

    let mut Loan100_150: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Income100_150: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Loan_To_Income100_150: Vec<f64> = sgx_tstd::vec::Vec::new();

    let mut Loan150_200: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Income150_200: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Loan_To_Income150_200: Vec<f64> = sgx_tstd::vec::Vec::new();

    let mut Loan200_250: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Income200_250: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Loan_To_Income200_250: Vec<f64> = sgx_tstd::vec::Vec::new();

    let mut Loan250_300: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Income250_300: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Loan_To_Income250_300: Vec<f64> = sgx_tstd::vec::Vec::new();

    let mut Loan300: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Income300: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut Loan_To_Income300: Vec<f64> = sgx_tstd::vec::Vec::new();

    for i in 0..unsealed_data_ccr.len() {

        Loan_2.push(unsealed_data_ccr[i].Loan);
        Income_2.push(unsealed_data_ccr[i].Income);

        Sex_2.push(unsealed_data_ccr[i].SEX);
        Education_2.push(unsealed_data_ccr[i].EDUCATION);

        if unsealed_data_ccr[i].SEX == 1.0 {
            Sex_M.push(1.0);
        } else {
            Sex_M.push(0.0);
        }

        if unsealed_data_ccr[i].EDUCATION == 2.0 {
            Education_2_2.push(1.0);
            Education_2_3.push(0.0);
            Education_2_4.push(0.0);
            Education_2_5.push(0.0);
            Education_2_6.push(0.0);
        } else if unsealed_data_ccr[i].EDUCATION == 3.0 {
            Education_2_2.push(0.0);
            Education_2_3.push(1.0);
            Education_2_4.push(0.0);
            Education_2_5.push(0.0);
            Education_2_6.push(0.0);
        } else if unsealed_data_ccr[i].EDUCATION == 4.0 {
            Education_2_2.push(0.0);
            Education_2_3.push(0.0);
            Education_2_4.push(1.0);
            Education_2_5.push(0.0);
            Education_2_6.push(0.0);
        } else if unsealed_data_ccr[i].EDUCATION == 5.0 {
            Education_2_2.push(0.0);
            Education_2_3.push(0.0);
            Education_2_4.push(0.0);
            Education_2_5.push(1.0);
            Education_2_6.push(0.0);
        } else if unsealed_data_ccr[i].EDUCATION == 6.0 {
            Education_2_2.push(0.0);
            Education_2_3.push(0.0);
            Education_2_4.push(0.0);
            Education_2_5.push(0.0);
            Education_2_6.push(1.0);
        } else {
            Education_2_2.push(0.0);
            Education_2_3.push(0.0);
            Education_2_4.push(0.0);
            Education_2_5.push(0.0);
            Education_2_6.push(0.0);
        }

        if (unsealed_data_ccr[i].Income >= 0.0) & (unsealed_data_ccr[i].Income < 50000.0) {
            Loan0_50.push(unsealed_data_ccr[i].Loan);
            Income0_50.push(unsealed_data_ccr[i].Income);
            Loan_To_Income0_50.push(unsealed_data_ccr[i].Loan/unsealed_data_ccr[i].Income);
        } else if (unsealed_data_ccr[i].Income >= 50000.0) & (unsealed_data_ccr[i].Income < 100000.0) {
            Loan50_100.push(unsealed_data_ccr[i].Loan);
            Income50_100.push(unsealed_data_ccr[i].Income);
            Loan_To_Income50_100.push(unsealed_data_ccr[i].Loan/unsealed_data_ccr[i].Income);
        } else if (unsealed_data_ccr[i].Income >= 100000.0) & (unsealed_data_ccr[i].Income < 150000.0) {
            Loan100_150.push(unsealed_data_ccr[i].Loan);
            Income100_150.push(unsealed_data_ccr[i].Income);
            Loan_To_Income100_150.push(unsealed_data_ccr[i].Loan/unsealed_data_ccr[i].Income);
        }  else if (unsealed_data_ccr[i].Income >= 150000.0) & (unsealed_data_ccr[i].Income < 200000.0) {
            Loan150_200.push(unsealed_data_ccr[i].Loan);
            Income150_200.push(unsealed_data_ccr[i].Income);
            Loan_To_Income150_200.push(unsealed_data_ccr[i].Loan/unsealed_data_ccr[i].Income);
        }  else if (unsealed_data_ccr[i].Income >= 200000.0) & (unsealed_data_ccr[i].Income < 250000.0) {
            Loan200_250.push(unsealed_data_ccr[i].Loan);
            Income200_250.push(unsealed_data_ccr[i].Income);
            Loan_To_Income200_250.push(unsealed_data_ccr[i].Loan/unsealed_data_ccr[i].Income);
        } else if (unsealed_data_ccr[i].Income >= 250000.0) & (unsealed_data_ccr[i].Income < 300000.0) {
            Loan250_300.push(unsealed_data_ccr[i].Loan);
            Income250_300.push(unsealed_data_ccr[i].Income);
            Loan_To_Income250_300.push(unsealed_data_ccr[i].Loan/unsealed_data_ccr[i].Income);
        } else {
            Loan300.push(unsealed_data_ccr[i].Loan);
            Income300.push(unsealed_data_ccr[i].Income);
            Loan_To_Income300.push(unsealed_data_ccr[i].Loan/unsealed_data_ccr[i].Income);
        }

    }

    println!("Income < R50 000:");
    println!();
    println!("The privacy preserving summary statistics for the Loan (Debt) variable is:");
    summary(&mut Loan0_50);
    println!("The privacy preserving summary statistics for the Income variable is:");
    summary(&mut Income0_50);
    println!("The privacy preserving summary statistics for the Debt-to-Income variable is:");
    summary(&mut Loan_To_Income0_50);

    println!("R50 000 <= Income < R100 000:");
    println!();
    println!("The privacy preserving summary statistics for the Loan (Debt) variable is:");
    summary(&mut Loan50_100);
    println!("The privacy preserving summary statistics for the Income variable is:");
    summary(&mut Income50_100);
    println!("The privacy preserving summary statistics for the Debt-to-Income variable is:");
    summary(&mut Loan_To_Income50_100);

    println!("R100 000 <= Income < R150 000:");
    println!();
    println!("The privacy preserving summary statistics for the Loan (Debt) variable is:");
    summary(&mut Loan100_150);
    println!("The privacy preserving summary statistics for the Income variable is:");
    summary(&mut Income100_150);
    println!("The privacy preserving summary statistics for the Debt-to-Income variable is:");
    summary(&mut Loan_To_Income100_150);

    println!("R150 000 <= Income < R200 000:");
    println!();
    println!("The privacy preserving summary statistics for the Loan (Debt) variable is:");
    summary(&mut Loan150_200);
    println!("The privacy preserving summary statistics for the Income variable is:");
    summary(&mut Income150_200);
    println!("The privacy preserving summary statistics for the Debt-to-Income variable is:");
    summary(&mut Loan_To_Income150_200);

    println!("R200 000 <= Income < R250 000:");
    println!();
    println!("The privacy preserving summary statistics for the Loan (Debt) variable is:");
    summary(&mut Loan200_250);
    println!("The privacy preserving summary statistics for the Income variable is:");
    summary(&mut Income200_250);
    println!("The privacy preserving summary statistics for the Debt-to-Income variable is:");
    summary(&mut Loan_To_Income200_250);

    println!("R250 000 <= Income < R300 000:");
    println!();
    println!("The privacy preserving summary statistics for the Loan (Debt) variable is:");
    summary(&mut Loan250_300);
    println!("The privacy preserving summary statistics for the Income variable is:");
    summary(&mut Income250_300);
    println!("The privacy preserving summary statistics for the Debt-to-Income variable is:");
    summary(&mut Loan_To_Income250_300);

    println!("Income >= R300 000:");
    println!();
    println!("The privacy preserving summary statistics for the Loan (Debt) variable is:");
    summary(&mut Loan300);
    println!("The privacy preserving summary statistics for the Income variable is:");
    summary(&mut Income300);
    println!("The privacy preserving summary statistics for the Debt-to-Income variable is:");
    summary(&mut Loan_To_Income300);

    if let Err(err) = simple_ols(Income_2.clone(), Loan_2.clone()) {
        println!("{}", err);
    }

    let mut input_vec_one = Income_2.clone();
    let mut input_vec_two = Education_2_2.clone();
    let mut input_vec_three = Education_2_3.clone();
    let mut input_vec_four = Education_2_4.clone();
    let mut input_vec_five = Education_2_5.clone();
    let mut input_vec_six = Education_2_6.clone();
    let mut input_vec_seven = Sex_M.clone();


    input_vec_one.append(&mut input_vec_two);
    input_vec_one.append(&mut input_vec_three);
    input_vec_one.append(&mut input_vec_four);
    input_vec_one.append(&mut input_vec_five);
    input_vec_one.append(&mut input_vec_six);
    input_vec_one.append(&mut input_vec_seven);

    let input_matrix_multiple = Matrix::new(7, input_vec_one.len() / 7, input_vec_one);

    //NB Transpose matrix
    if let Err(err) = multiple_ols(input_matrix_multiple.transpose(), Loan_2.clone()) {
        println!("{}", err);
    }

    sgx_status_t::SGX_SUCCESS
}

// 3 - Simple OLS example
#[no_mangle]
pub extern "C" fn simple_ols_eg() -> sgx_status_t {

    let json_simple_ols =  sgx_tstd::path::Path::new("../data/Wage.json");
    let file_simple_ols =  sgx_tstd::untrusted::fs::File::open(json_simple_ols).expect("file not found");
    let clients:sgx_tstd::vec::Vec<Client> = serde_json::from_reader(file_simple_ols).expect("Error while reading or parsing");

    let mut wage: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut educ: Vec<f64> = sgx_tstd::vec::Vec::new();

    for client in clients {
        wage.push(client.wage);
        educ.push(client.educ);
        // educ.push(client.ROE.parse::<f64>().unwrap());
    }

    println!("This is Example 2.4: Wage & Education from Wooldridge:");
    println!();

    println!("The summary statistics for the Educ variable:");
    summary(&mut educ);

    println!("The summary statistics for the Wage variable:");
    summary(&mut wage);

    if let Err(err) = simple_ols(educ, wage) {
        println!("{}", err);
    }

    sgx_status_t::SGX_SUCCESS
}

// 4 - Multiple OLS example
#[no_mangle]
pub extern "C" fn multiple_ols_eg() -> sgx_status_t {

    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    // Multiple linear regression example (OLS)

    let json_multiple_ols =  sgx_tstd::path::Path::new("../data/MLR.json");
    let file_multiple_ols =  sgx_tstd::untrusted::fs::File::open(json_multiple_ols).expect("file not found");
    let clients_multiple:sgx_tstd::vec::Vec<Client_Multiple> = serde_json::from_reader(file_multiple_ols).expect("Error while reading or parsing");

    let mut colGPA: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut hsGPA: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut ACT: Vec<f64> = sgx_tstd::vec::Vec::new();

    for client_multiple in clients_multiple {
        colGPA.push(client_multiple.colGPA);
        hsGPA.push(client_multiple.hsGPA);
        ACT.push(client_multiple.ACT);
    }

    println!("This is Example 3.1: Determinants of College GPA from Wooldridge:");
    println!();

    println!("The summary statistics for the colGPA variable:");
    summary(&mut colGPA);

    println!("The summary statistics for the hsGPA variable:");
    summary(&mut hsGPA);

    println!("The summary statistics for the ACT variable:");
    summary(&mut ACT);

    let mut input_vec_one = hsGPA.clone();
    let mut input_vec_two = ACT.clone();

    input_vec_one.append(&mut input_vec_two);
    let input_matrix = Matrix::new(2, input_vec_one.len() / 2, input_vec_one);

    //NB Transpose matrix
    if let Err(err) = multiple_ols(input_matrix.transpose(), colGPA) {
        println!("{}", err);
    }

    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    sgx_status_t::SGX_SUCCESS
}

// 5 - Time series regression examples
#[no_mangle]
pub extern "C" fn time_series_eg() -> sgx_status_t {

    // Multiple linear regression: Time series- Wooldridge 10.5

    let json_time =  sgx_tstd::path::Path::new("../data/Barium.json");
    let file_time =  sgx_tstd::untrusted::fs::File::open(json_time).expect("file not found");
    let clients_time:sgx_tstd::vec::Vec<Client_Time> = serde_json::from_reader(file_time).expect("Error while reading or parsing");

    let mut lchnimp: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut lgas: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut lrtwex: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut lchempi: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut befile6: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut affile6: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut afdec6: Vec<f64> = sgx_tstd::vec::Vec::new();

    for client in clients_time {
        lchnimp.push(client.lchnimp);
        lgas.push(client.lgas);
        lrtwex.push(client.lrtwex);
        lchempi.push(client.lchempi);
        befile6.push(client.befile6);
        affile6.push(client.affile6);
        afdec6.push(client.afdec6);
    }

    println!("This is Example 10.5: Antidumping Filings and Chemical Imports from Wooldridge:");
    println!();

    println!("The summary statistics for the lchnimp variable:");
    summary(&mut lchnimp);

    println!("The summary statistics for the lgas variable:");
    summary(&mut lgas);

    println!("The summary statistics for the lrtwex variable:");
    summary(&mut lrtwex);

    println!("The summary statistics for the lchempi variable:");
    summary(&mut lchempi);

    println!("The summary statistics for the befile6 variable:");
    summary(&mut befile6);

    println!("The summary statistics for the affile6 variable:");
    summary(&mut affile6);

    println!("The summary statistics for the afdec6 variable:");
    summary(&mut afdec6);

    let mut input_vec_one_time = lchempi.clone();
    let mut input_vec_two_time = lgas.clone();
    let mut input_vec_three_time = lrtwex.clone();
    let mut input_vec_four_time = befile6.clone();
    let mut input_vec_five_time = affile6.clone();
    let mut input_vec_six_time = afdec6.clone();

    input_vec_one_time.append(&mut input_vec_two_time);
    input_vec_one_time.append(&mut input_vec_three_time);
    input_vec_one_time.append(&mut input_vec_four_time);
    input_vec_one_time.append(&mut input_vec_five_time);
    input_vec_one_time.append(&mut input_vec_six_time);

    let input_matrix_time = Matrix::new(6, input_vec_one_time.len() / 6, input_vec_one_time);

    //NB Transpose matrix
    if let Err(err) = multiple_ols(input_matrix_time.transpose(), lchnimp) {
        println!("{}", err);
    }



    // Multiple linear regression: Time series- Wooldridge 10.11

    let json_time_month =  sgx_tstd::path::Path::new("../data/Barium_Month.json");
    let file_time_month =  sgx_tstd::untrusted::fs::File::open(json_time_month).expect("file not found");
    let clients_time_month:sgx_tstd::vec::Vec<Client_Time_Month> = serde_json::from_reader(file_time_month).expect("Error while reading or parsing");

    let mut lchnimp_month: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut lgas_month: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut lrtwex_month: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut lchempi_month: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut befile6_month: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut affile6_month: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut afdec6_month: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut feb: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut mar: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut apr: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut may: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut jun: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut jul: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut aug: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut sep: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut oct: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut nov: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut dec: Vec<f64> = sgx_tstd::vec::Vec::new();

    for client in clients_time_month {
        lchnimp_month.push(client.lchnimp);
        lgas_month.push(client.lgas);
        lrtwex_month.push(client.lrtwex);
        lchempi_month.push(client.lchempi);
        befile6_month.push(client.befile6);
        affile6_month.push(client.affile6);
        afdec6_month.push(client.afdec6);
        feb.push(client.feb);
        mar.push(client.mar);
        apr.push(client.apr);
        may.push(client.may);
        jun.push(client.jun);
        jul.push(client.jul);
        aug.push(client.aug);
        sep.push(client.sep);
        oct.push(client.oct);
        nov.push(client.nov);
        dec.push(client.dec);
    }

    println!("This is Example 10.11: Effects of Antidumping Filings from Wooldridge:");
    println!();

    let mut input_vec_one_time_month = lchempi_month.clone();
    let mut input_vec_two_time_month = lgas_month.clone();
    let mut input_vec_three_time_month = lrtwex_month.clone();
    let mut input_vec_four_time_month = befile6_month.clone();
    let mut input_vec_five_time_month = affile6_month.clone();
    let mut input_vec_six_time_month = afdec6_month.clone();
    let mut input_vec_seven_time_month = feb.clone();
    let mut input_vec_eight_time_month = mar.clone();
    let mut input_vec_nine_time_month = apr.clone();
    let mut input_vec_ten_time_month = may.clone();
    let mut input_vec_eleven_time_month = jun.clone();
    let mut input_vec_twelve_time_month = jul.clone();
    let mut input_vec_thirteen_time_month = aug.clone();
    let mut input_vec_fourteen_time_month = sep.clone();
    let mut input_vec_fifteen_time_month = oct.clone();
    let mut input_vec_sixteen_time_month = nov.clone();
    let mut input_vec_seventeen_time_month = dec.clone();

    input_vec_one_time_month.append(&mut input_vec_two_time_month);
    input_vec_one_time_month.append(&mut input_vec_three_time_month);
    input_vec_one_time_month.append(&mut input_vec_four_time_month);
    input_vec_one_time_month.append(&mut input_vec_five_time_month);
    input_vec_one_time_month.append(&mut input_vec_six_time_month);
    input_vec_one_time_month.append(&mut input_vec_seven_time_month);
    input_vec_one_time_month.append(&mut input_vec_eight_time_month);
    input_vec_one_time_month.append(&mut input_vec_nine_time_month);
    input_vec_one_time_month.append(&mut input_vec_ten_time_month);
    input_vec_one_time_month.append(&mut input_vec_eleven_time_month);
    input_vec_one_time_month.append(&mut input_vec_twelve_time_month);
    input_vec_one_time_month.append(&mut input_vec_thirteen_time_month);
    input_vec_one_time_month.append(&mut input_vec_fourteen_time_month);
    input_vec_one_time_month.append(&mut input_vec_fifteen_time_month);
    input_vec_one_time_month.append(&mut input_vec_sixteen_time_month);
    input_vec_one_time_month.append(&mut input_vec_seventeen_time_month);

    let input_matrix_time_month = Matrix::new(17, input_vec_one_time_month.len() / 17, input_vec_one_time_month);

    //NB Transpose matrix
    if let Err(err) = multiple_ols(input_matrix_time_month.transpose(), lchnimp_month) {
        println!("{}", err);
    }


    sgx_status_t::SGX_SUCCESS
}

//6 - Unsealing data example with SgxFile
#[no_mangle]
pub extern "C" fn panel_data_eg() -> sgx_status_t {
    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    // Multiple linear regression: Panel data - Wooldridge 13.1

    let json_panel =  sgx_tstd::path::Path::new("../data/fertil1.json");
    let file_panel =  sgx_tstd::untrusted::fs::File::open(json_panel).expect("file not found");
    let clients_panel:sgx_tstd::vec::Vec<Client_Panel> = serde_json::from_reader(file_panel).expect("Error while reading or parsing");

    let mut kids: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut educ: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut age: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut agesq: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut black: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut east: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut northcen: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut west: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut farm: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut othrural: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut town: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut smcity: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut y74: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut y76: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut y78: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut y80: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut y82: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut y84: Vec<f64> = sgx_tstd::vec::Vec::new();

    for client in clients_panel {
        kids.push(client.kids);
        educ.push(client.educ);
        age.push(client.age);
        agesq.push(client.agesq);
        black.push(client.black);
        east.push(client.east);
        northcen.push(client.northcen);
        west.push(client.west);
        farm.push(client.farm);
        othrural.push(client.othrural);
        town.push(client.town);
        smcity.push(client.smcity);
        y74.push(client.y74);
        y76.push(client.y76);
        y78.push(client.y78);
        y80.push(client.y80);
        y82.push(client.y82);
        y84.push(client.y84);
    }

    println!("This is Example 13.1: Women’s Fertility over Time from Wooldridge:");
    println!();

    let mut input_vec_one_panel = educ.clone();
    let mut input_vec_two_panel = age.clone();
    let mut input_vec_three_panel = agesq.clone();
    let mut input_vec_four_panel = black.clone();
    let mut input_vec_five_panel = east.clone();
    let mut input_vec_six_panel = northcen.clone();
    let mut input_vec_seven_panel = west.clone();
    let mut input_vec_eight_panel = farm.clone();
    let mut input_vec_nine_panel = othrural.clone();
    let mut input_vec_ten_panel = town.clone();
    let mut input_vec_eleven_panel = smcity.clone();
    let mut input_vec_twelve_panel = y74.clone();
    let mut input_vec_thirteen_panel = y76.clone();
    let mut input_vec_fourteen_panel = y78.clone();
    let mut input_vec_fifteen_panel = y80.clone();
    let mut input_vec_sixteen_panel = y82.clone();
    let mut input_vec_seventeen_panel = y84.clone();

    input_vec_one_panel.append(&mut input_vec_two_panel);
    input_vec_one_panel.append(&mut input_vec_three_panel);
    input_vec_one_panel.append(&mut input_vec_four_panel);
    input_vec_one_panel.append(&mut input_vec_five_panel);
    input_vec_one_panel.append(&mut input_vec_six_panel);
    input_vec_one_panel.append(&mut input_vec_seven_panel);
    input_vec_one_panel.append(&mut input_vec_eight_panel);
    input_vec_one_panel.append(&mut input_vec_nine_panel);
    input_vec_one_panel.append(&mut input_vec_ten_panel);
    input_vec_one_panel.append(&mut input_vec_eleven_panel);
    input_vec_one_panel.append(&mut input_vec_twelve_panel);
    input_vec_one_panel.append(&mut input_vec_thirteen_panel);
    input_vec_one_panel.append(&mut input_vec_fourteen_panel);
    input_vec_one_panel.append(&mut input_vec_fifteen_panel);
    input_vec_one_panel.append(&mut input_vec_sixteen_panel);
    input_vec_one_panel.append(&mut input_vec_seventeen_panel);

    let input_matrix_panel = Matrix::new(17, input_vec_one_panel.len() / 17, input_vec_one_panel);

    //NB Transpose matrix
    if let Err(err) = multiple_ols(input_matrix_panel.transpose(), kids) {
        println!("{}", err);
    }

    sgx_status_t::SGX_SUCCESS
}

//7 - Seal/ Unseal data example with SgxSealedData
#[no_mangle]
pub extern "C" fn seal_sgxdata_eg() -> sgx_status_t {

    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    // Sealing with SgxSealedData

    let json_sealing_method_one =  sgx_tstd::path::Path::new("../data/MLR.json");
    let file_sealing_method_one =  sgx_tstd::untrusted::fs::File::open(json_sealing_method_one).expect("file not found");
    let clients_sealing_one:sgx_tstd::vec::Vec<Client_Sealing_One> = serde_json::from_reader(file_sealing_method_one).expect("Error while reading or parsing");

    //create sealed buffer for the sealed ouput to go into
    let mut sealedcipher_vec: Vec<u8> = vec![0; SCRATCH_PAD_SIZE];
    let sealedcipher_vec_pointer: *mut u8 = &mut sealedcipher_vec[0];

    let sealed_data_tuple = match seal_method_one(clients_sealing_one, sealedcipher_vec_pointer) {
        Ok(x) => x,
        Err(ret) => {
            return ret
        },
    };

    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!



    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    // Unsealing with SgxSealedData

    let unsealed_data_one = match unseal_method_one(sealed_data_tuple.0, sealed_data_tuple.1) {
        Ok(x) => x,
        Err(ret) => {
            return ret
        },
    };

    // println!("The unsealed data from method one: {:?}", unsealed_data_one);

    let mut colGPA1: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut hsGPA1: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut ACT1: Vec<f64> = sgx_tstd::vec::Vec::new();
    for i in 0..unsealed_data_one.len() {
        colGPA1.push(unsealed_data_one[i].colGPA);
        hsGPA1.push(unsealed_data_one[i].hsGPA);
        ACT1.push(unsealed_data_one[i].ACT);
    }

    println!();
    println!("Unseal method one summary statistics:");
    println!();
    summary(&mut colGPA1);
    summary(&mut hsGPA1);
    summary(&mut ACT1);

    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    sgx_status_t::SGX_SUCCESS
}

//8 - Sealing data example with SgxFile
#[no_mangle]
pub extern "C" fn seal_sgxfile_eg() -> sgx_status_t {

    let json_sealing_method_two =  sgx_tstd::path::Path::new("../data/MLR.json");
    let file_sealing_method_two =  sgx_tstd::untrusted::fs::File::open(json_sealing_method_two).expect("file not found");
    let clients_sealing_two:sgx_tstd::vec::Vec<Client_Sealing_Two> = serde_json::from_reader(file_sealing_method_two).expect("Error while reading or parsing");
    seal_method_two(clients_sealing_two);

    sgx_status_t::SGX_SUCCESS
}

//9 - Unsealing data example with SgxFile
#[no_mangle]
pub extern "C" fn unseal_sgxfile_eg() -> sgx_status_t {
    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    // Unsealing with SgxFile

    //NB CHECK THIS (unwrap) -> Because this function may panic, its use is generally discouraged.
    // Instead, prefer to use pattern matching and handle the None case explicitly, or call
    // unwrap_or, unwrap_or_else, or unwrap_or_default.
    let unsealed_data_two = unseal_method_two().unwrap();

    let mut colGPA2: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut hsGPA2: Vec<f64> = sgx_tstd::vec::Vec::new();
    let mut ACT2: Vec<f64> = sgx_tstd::vec::Vec::new();
    for i in 0..unsealed_data_two.len() {
        colGPA2.push(unsealed_data_two[i].colGPA);
        hsGPA2.push(unsealed_data_two[i].hsGPA);
        ACT2.push(unsealed_data_two[i].ACT);
    }


    println!();
    println!("Unseal method two summary statistics:");
    println!();
    summary(&mut colGPA2);
    summary(&mut hsGPA2);
    summary(&mut ACT2);

    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

    sgx_status_t::SGX_SUCCESS
}

//A function that is used to seal the ccr database using SgxFile
//The function requires the data to be formatted as a Vec using the predefined struct: Entry
//The function returns a status
fn seal_method_two_ccr(unsealed_vector: sgx_tstd::vec::Vec<Entry>) -> sgx_status_t {
    let encoded_vec = serde_cbor::to_vec(&unsealed_vector).unwrap();
    let encoded_slice = encoded_vec.as_slice();
    // println!("Length of encoded slice for SgxFile: {}", encoded_slice.len());
    // println!("Encoded slice for SgxFile: {:?}", encoded_slice);

    match SgxFile::create("sealed_ccr.json") {
        Ok(mut f) => match f.write_all(encoded_slice) {
            Ok(()) => {
                println!("SgxFile write JSON file success!");
                println!("Data sealed successfully using SgxFile");
                sgx_status_t::SGX_SUCCESS
            }
            Err(x) => {
                println!("SgxFile write JSON file failed! {}", x);
                sgx_status_t::SGX_ERROR_UNEXPECTED
            }
        },
        Err(x) => {
            println!("SgxFile create file {} error {}", DATAFILE, x);
            sgx_status_t::SGX_ERROR_UNEXPECTED
        }
    }
}

//A function that is used to unseal the CCR database using SgxFile
//The function does not require any input
//The function returns an option containing a Vec of the struct: Entry
fn unseal_method_two_ccr() -> Option<Vec<Entry>> {
    let mut data_vec: Vec<u8> = Vec::new();

    let data_json_str : Option<Vec<Entry>> = match SgxFile::open("sealed_ccr.json") {
        Ok(mut f) => match f.read_to_end(&mut data_vec) {
            Ok(len) => {
                println!("Read {} bytes from Key file", len);
                println!("Data unsealed successfully");
                let entries_unsealing_two:sgx_tstd::vec::Vec<Entry> = serde_cbor::from_slice(&data_vec).unwrap();
                Some(entries_unsealing_two)
            }
            Err(x) => {
                println!("Read keyfile failed {}", x);
                None
            }
        },
        Err(_) => {
            println!("get_sealed_pcl_key cannot open keyfile, please check that key is provisioned successfully!");
            None
        }
    };

    return data_json_str;

}

//10 - Seal the CCR's data for the first time
#[no_mangle]
pub extern "C" fn seal_sgxfile_ccr() -> sgx_status_t {

    let json_sealing_ccr =  sgx_tstd::path::Path::new("../data/ccr.json");
    let file_sealing_ccr =  sgx_tstd::untrusted::fs::File::open(json_sealing_ccr).expect("file not found");
    let entries_sealing_ccr:sgx_tstd::vec::Vec<Entry> = serde_json::from_reader(file_sealing_ccr).expect("Error while reading or parsing");
    seal_method_two_ccr(entries_sealing_ccr);

    sgx_status_t::SGX_SUCCESS
}


// The function calculates the execution time of features
// One example is shown - the function was altered depending on the feature evaluated
// Loops save a lot of time - Evaluate each feature > 100 times to get a better estimate of execution times
#[no_mangle]
pub extern "C" fn ccr_poc() -> sgx_status_t {

    let start = Instant::now();
    let json_file_path = sgx_tstd::path::Path::new("../data/ExportJson_10.json");
    let file = sgx_tstd::untrusted::fs::File::open(json_file_path).expect("File not found");
    let dur = start.elapsed();
    println!("Time to open JSON is: {:?}", dur);

    let start1 = Instant::now();
    let users:sgx_tstd::vec::Vec<User> = serde_json::from_reader(file).expect("Error while reading or parsing");
    let dur1 = start1.elapsed();
    println!("Time to parse data to struct is: {:?}", dur1);

    let start2 = Instant::now();
    let mut field_test: sgx_tstd::vec::Vec<f64> = sgx_tstd::vec::Vec::new();
    for user in users {
        field_test.push(user.Field4.parse::<f64>().unwrap());
    }
    let dur2 = start2.elapsed();
    println!("Time to create vector is: {:?}", dur2);
    println!();

    let start3 = Instant::now();
    summary(&mut field_test);
    let dur3 = start3.elapsed();
    println!("Time to execute summary statistics is: {:?}", dur3);

    sgx_status_t::SGX_SUCCESS
}


fn to_sealed_log_for_slice<T: Copy + ContiguousMemory>(sealed_data: &SgxSealedData<[T]>, sealed_log: * mut u8, sealed_log_size: u32) -> Option<* mut sgx_sealed_data_t> {
    unsafe {
        sealed_data.to_raw_sealed_data_t(sealed_log as * mut sgx_sealed_data_t, sealed_log_size)
    }
}

fn from_sealed_log_for_slice<'a, T: Copy + ContiguousMemory>(sealed_log: * mut u8, sealed_log_size: u32) -> Option<SgxSealedData<'a, [T]>> {
    unsafe {
        SgxSealedData::<[T]>::from_raw_sealed_data_t(sealed_log as * mut sgx_sealed_data_t, sealed_log_size)
    }
}
