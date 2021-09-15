# POC_CCR

Credit to https://github.com/apache/incubator-teaclave-sgx-sdk, without which this project would not be possible.

This POC Application runs on Intel SGX using the Rust SGX SDK.

This file serves as a guide for setting up the application on a local machine and running the project in simulation mode.

# Requirements

* [Docker](https://www.docker.com/)
* [Rust SGX SDK](https://github.com/apache/incubator-teaclave-sgx-sdk)
* Compatible machine: The code was developed on a 2014 Macbook Pro
* IDE: The code was developed with [CLION](https://www.jetbrains.com/clion/), but [VSCode](https://code.visualstudio.com/) also works

# Setup and Installation

The setup will cover the steps to setup the application on a MacOS machine that does not natively support Intel SGX. The application will run a simulated version of Intel SGX using a docker container. Note that the initial setup may take some time depending on your computer and internet connection.

1. The first step is to install the [Rust SGX SDK specifically for simulation mode](https://github.com/apache/incubator-teaclave-sgx-sdk#use-simulation-mode-for-non-sgx-enabled-machine-includes-macos)

* Pull the docker container: `$ docker pull baiduxlab/sgx-rust`
* Start a docker with the Rust SGX SDK `$ docker run -v /your/path/to/rust-sgx:/root/sgx -ti baiduxlab/sgx-rust`
  * Remember to change `/your/path/to` to the path where you downloaded the docker container

2. Validate that the install is working:
* Navigate to `~/sgx/samplecode/helloworld`
  * set the `SGX_MODE` to `SW` in Makefile
  * Replace `SGX_MODE ?= HW` with `SGX_MODE ?= SW`
  * OR run `export SGX_MODE=SW` in your terminal
* Test if the sample code is working:

  `root@docker:~/sgx/samplecode/helloworld# make`
  
  `root@docker:~/sgx/samplecode/helloworld# cd bin`
  
  `root@docker:~/sgx/samplecode/helloworld/bin# ./app`
  
  * Consult the [Rust SGX SDK](https://github.com/apache/incubator-teaclave-sgx-sdk) page if any errors occur, they have detailed setup instructions.

3. Once you have verified that the Rust SGX SDK is working:
* Navigate to the samplecode folder in the Rust SGX SDK: e.g. `cd /your/path/to/rust-sgx/samplecode/`
* Clone this repository into the samplecode folder

4. Download the data for the application:
* Download the compressed file containing all the data from [Google Drive](https://drive.google.com/file/d/1MTeFmfN4V02Uyb2WOoi4xtI4TG5UBw8J/view?usp=sharing)
* Unzip the compressed file and extract all the json files to `/your/path/to/rust-sgx/samplecode/POC_CCR/data/`

5. Running the POC Application
* Remember to start the docker container, consult Step 1.
* Navigate to the correct folder: `root@docker:~# cd sgx/samplecode/POC_CCR`
* Run `export SGX_MODE=SW` in your terminal

  `root@docker:~/sgx/samplecode/POC_CCR# make`
  
  `root@docker:~/sgx/samplecode/POC_CCR# cd bin`
  
  `root@docker:~/sgx/samplecode/POC_CCR/bin# ./app`


