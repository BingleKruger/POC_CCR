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

extern crate sgx_types;
extern crate sgx_urts;
extern crate sgx_crypto_helper;
extern crate serde_derive;
extern crate serde;
extern crate serde_json;


use sgx_types::*;
use sgx_urts::SgxEnclave;
use std::io::stdin;

static ENCLAVE_FILE: &'static str = "enclave.signed.so";

extern {

    fn ccr_poc(eid: sgx_enclave_id_t,
                     retval: *mut sgx_status_t
    ) -> sgx_status_t;

    fn lending_profiles(eid: sgx_enclave_id_t,
                        retval: *mut sgx_status_t,
                        some_string_postal: *const u8,
                        len_postal: usize,
                        some_string_term: *const u8,
                        len_term: usize
    ) -> sgx_status_t;

    fn credit_health(eid: sgx_enclave_id_t,
                     retval: *mut sgx_status_t
    ) -> sgx_status_t;

    fn simple_ols_eg(eid: sgx_enclave_id_t,
                     retval: *mut sgx_status_t
    ) -> sgx_status_t;

    fn multiple_ols_eg(eid: sgx_enclave_id_t,
                     retval: *mut sgx_status_t
    ) -> sgx_status_t;

    fn time_series_eg(eid: sgx_enclave_id_t,
                     retval: *mut sgx_status_t
    ) -> sgx_status_t;

    fn panel_data_eg(eid: sgx_enclave_id_t,
                     retval: *mut sgx_status_t
    ) -> sgx_status_t;

    fn seal_sgxdata_eg(eid: sgx_enclave_id_t,
                      retval: *mut sgx_status_t
    ) -> sgx_status_t;

    fn seal_sgxfile_eg(eid: sgx_enclave_id_t,
                      retval: *mut sgx_status_t
    ) -> sgx_status_t;

    fn unseal_sgxfile_eg(eid: sgx_enclave_id_t,
                      retval: *mut sgx_status_t
    ) -> sgx_status_t;

    fn seal_sgxfile_ccr(eid: sgx_enclave_id_t,
                         retval: *mut sgx_status_t
    ) -> sgx_status_t;


}

fn init_enclave() -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {secs_attr: sgx_attributes_t { flags:0, xfrm:0}, misc_select:0};
    SgxEnclave::create(ENCLAVE_FILE,
                       debug,
                       &mut launch_token,
                       &mut launch_token_updated,
                       &mut misc_attr)
}

fn main() {
    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}! [+]", r.geteid());
            r
        },
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        },
    };

    let mut exit_string = String::from("Continue");
    println!("Welcome to the CCR POC Application");
    while exit_string == "Continue" {
        println!();
        println!("The queries supported by this application are:");
        println!("1 - Use case 1: Lending profiles");
        println!("2 - Use case 2: Credit health");
        println!("3 - Simple OLS example");
        println!("4 - Multiple OLS example");
        println!("5 - Multiple linear regression: Time series examples");
        println!("6 - Multiple linear regression: Panel data example");
        println!("7 - Seal/ Unseal data example with SgxSealedData");
        println!("8 - Sealing data example with SgxFile");
        println!("9 - Unsealing data example with SgxFile");
        println!("10 - Sealing the CCR database for the first time");
        println!();

        let mut buffer = String::new();
        println!("Please enter a number to run the specific query: (Otherwise type 'Exit')");

        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end();
                if parsed == "Exit" {
                    exit_string = String::from("Exit");
                } else if parsed == "Time" {
                    println!();
                    println!("Function for calculating execution times of features");
                    let mut retval = sgx_status_t::SGX_SUCCESS;
                    let result = unsafe {
                        ccr_poc(enclave.geteid(),
                                      &mut retval,
                        )
                    };
                    match result {
                        sgx_status_t::SGX_SUCCESS => {},
                        _ => {
                            println!("[-] ECALL Enclave Failed {}!", result.as_str());
                            return;
                        }
                    }
                } else if parsed == "1" {
                    println!("Use case 1 - Lending profiles selected");
                    let ccr_bool: bool = std::path::Path::new("sealed_ccr.json").exists();
                    if ccr_bool {
                        println!("Please enter the postal code");
                        let mut buffer_postal = String::new();
                        match stdin().read_line(&mut buffer_postal) {
                            Ok(_) => {
                                let parsed_postal = buffer_postal.trim_end();
                                let mut buffer_term = String::new();
                                println!("Please enter the minimum term in years");
                                match stdin().read_line(&mut buffer_term) {
                                    Ok(_) => {
                                        let parsed_term = buffer_term.trim_end();
                                        let mut retval_use_1 = sgx_status_t::SGX_SUCCESS;
                                        let input_postal = String::from(parsed_postal);
                                        let input_term = String::from(parsed_term);
                                        let result_use_1 = unsafe {
                                            lending_profiles(enclave.geteid(),
                                                             &mut retval_use_1,
                                                             input_postal.as_ptr() as *const u8,
                                                             input_postal.len(),
                                                             input_term.as_ptr() as *const u8,
                                                             input_term.len()
                                            )
                                        };
                                        match result_use_1 {
                                            sgx_status_t::SGX_SUCCESS => {},
                                            _ => {
                                                println!("[-] ECALL Enclave Failed {}!", result_use_1.as_str());
                                                return;
                                            }
                                        }
                                    },
                                    Err(_) => {
                                        println!("[-] Error: Input is invalid");
                                    }
                                }
                            },
                            Err(_) => {
                                println!("[-] Error: Input is invalid");
                            }
                        }
                    } else {
                        println!("[-] Error: Sealed CCR database is missing");
                    }
                } else if parsed == "2" {
                    println!("Use case 2: Credit health selected");
                    println!();
                    let ccr_bool: bool = std::path::Path::new("sealed_ccr.json").exists();
                    if ccr_bool {
                        let mut retval_use_2 = sgx_status_t::SGX_SUCCESS;
                        let result_use_2 = unsafe {
                            credit_health(enclave.geteid(),
                                          &mut retval_use_2,
                            )
                        };
                        match result_use_2 {
                            sgx_status_t::SGX_SUCCESS => {},
                            _ => {
                                println!("[-] ECALL Enclave Failed {}!", result_use_2.as_str());
                                return;
                            }
                        }
                    } else {
                        println!("[-] Error: Sealed CCR database is missing");
                    }
                } else if parsed == "3" {
                    println!("Simple OLS example selected");
                    let mut retval_use_3 = sgx_status_t::SGX_SUCCESS;
                    let result_use_3 = unsafe {
                        simple_ols_eg(enclave.geteid(),
                                      &mut retval_use_3,
                        )
                    };
                    match result_use_3 {
                        sgx_status_t::SGX_SUCCESS => {},
                        _ => {
                            println!("[-] ECALL Enclave Failed {}!", result_use_3.as_str());
                            return;
                        }
                    }
                } else if parsed == "4" {
                    println!("Multiple OLS example selected");
                    let mut retval_use_4 = sgx_status_t::SGX_SUCCESS;
                    let result_use_4 = unsafe {
                        multiple_ols_eg(enclave.geteid(),
                                      &mut retval_use_4,
                        )
                    };
                    match result_use_4 {
                        sgx_status_t::SGX_SUCCESS => {},
                        _ => {
                            println!("[-] ECALL Enclave Failed {}!", result_use_4.as_str());
                            return;
                        }
                    }
                } else if parsed == "5" {
                    println!("Multiple linear regression: Time series examples selected");
                    let mut retval_use_5 = sgx_status_t::SGX_SUCCESS;
                    let result_use_5 = unsafe {
                        time_series_eg(enclave.geteid(),
                                      &mut retval_use_5,
                        )
                    };
                    match result_use_5 {
                        sgx_status_t::SGX_SUCCESS => {},
                        _ => {
                            println!("[-] ECALL Enclave Failed {}!", result_use_5.as_str());
                            return;
                        }
                    }
                } else if parsed == "6" {
                    println!("Multiple linear regression: Panel data examples selected");
                    let mut retval_use_6 = sgx_status_t::SGX_SUCCESS;
                    let result_use_6 = unsafe {
                        panel_data_eg(enclave.geteid(),
                                       &mut retval_use_6,
                        )
                    };
                    match result_use_6 {
                        sgx_status_t::SGX_SUCCESS => {},
                        _ => {
                            println!("[-] ECALL Enclave Failed {}!", result_use_6.as_str());
                            return;
                        }
                    }
                } else if parsed == "7" {
                    println!("Seal/ Unseal data example with SgxSealedData selected");
                    let mut retval_use_7 = sgx_status_t::SGX_SUCCESS;
                    let result_use_7 = unsafe {
                        seal_sgxdata_eg(enclave.geteid(),
                                      &mut retval_use_7,
                        )
                    };
                    match result_use_7 {
                        sgx_status_t::SGX_SUCCESS => {},
                        _ => {
                            println!("[-] ECALL Enclave Failed {}!", result_use_7.as_str());
                            return;
                        }
                    }
                } else if parsed == "8" {
                    println!("Sealing data example with SgxFile selected");
                    let mut retval_use_8 = sgx_status_t::SGX_SUCCESS;
                    let result_use_8 = unsafe {
                        seal_sgxfile_eg(enclave.geteid(),
                                        &mut retval_use_8,
                        )
                    };
                    match result_use_8 {
                        sgx_status_t::SGX_SUCCESS => {},
                        _ => {
                            println!("[-] ECALL Enclave Failed {}!", result_use_8.as_str());
                            return;
                        }
                    }
                } else if parsed == "9" {
                    let ccr_bool: bool = std::path::Path::new("sealed.json").exists();
                    if ccr_bool {
                        println!("Unsealing data example with SgxFile selected");
                        let mut retval_use_9 = sgx_status_t::SGX_SUCCESS;
                        let result_use_9 = unsafe {
                            unseal_sgxfile_eg(enclave.geteid(),
                                              &mut retval_use_9,
                            )
                        };
                        match result_use_9 {
                            sgx_status_t::SGX_SUCCESS => {},
                            _ => {
                                println!("[-] ECALL Enclave Failed {}!", result_use_9.as_str());
                                return;
                            }
                        }
                    } else {
                        println!("[-] Error: Sealed CCR database is missing");
                    }
                } else if parsed == "10" {
                    println!("Sealing the CCR database for the first time");
                    let mut retval_use_10 = sgx_status_t::SGX_SUCCESS;
                    let retval_use_10 = unsafe {
                        seal_sgxfile_ccr(enclave.geteid(),
                                        &mut retval_use_10,
                        )
                    };
                    match retval_use_10 {
                        sgx_status_t::SGX_SUCCESS => {},
                        _ => {
                            println!("[-] ECALL Enclave Failed {}!", retval_use_10.as_str());
                            return;
                        }
                    }
                } else {
                    println!("[-] Error: Invalid operation selected");
                }
            },
            Err(_) => {
                println!("[-] Error: Input is invalid'");
            }
        }
    }

    println!("[+] Terminating the CCR POC Application");
    enclave.destroy();
}
