#!/bin/bash

#get the dir the script is located
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

docker run -d -v ${SCRIPT_DIR}/src:/home/ubuntu/c2rust thesis