#!/bin/bash

CUR_DATETIME="`date +%Y-%m-%d-%H`"
LOG_NAME="sync.log.$CUR_DATETIME"
echo "Current log name: $LOG_NAME"
./chainx -d data --log=error,msgbus=info --name=cqx --port 20001 --pruning archive \
    --rpc-port=8098 --ws-port=8099 --rpc-external --ws-external --no-grandpa \
    --syncing-execution=Native --no-telemetry &>> log/$LOG_NAME
