#!/bin/bash

work_path="/home/koushiro/Code/Work/bin"
name="cqx"
telemetry_option="--no-telemetry"
${work_path}/chainx --name=${name} -d ${work_path}/data --log=error,msgbus=info --port 20001 --pruning archive \
    --rpc-port=8098 --ws-port=8099 --rpc-external --ws-external --no-grandpa \
    --syncing-execution=Native ${telemetry_option} &>> ${work_path}/log/sync.log &
