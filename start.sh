#!/bin/bash
set +e
CUR_PATH=$(cd `dirname $0`; pwd)
cd ${CUR_PATH}/admintool/
./setup.sh
./admintool.sh

start_node() {
    id=$1
    cd ${CUR_PATH}/admintool/release/node${id}
    mkdir -p db
    export DATA_PATH=${PWD}/db
    RUST_LOG=chit nohup ./chit 2>&1 > log &
}

start_all () {
    start_node 0
    start_node 1
    start_node 2
    start_node 3
}

echo "###start nodes..."
start_all
echo `date`
echo "###Chit start OK"
exit 0
