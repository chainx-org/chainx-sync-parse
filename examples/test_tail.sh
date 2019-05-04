#!/bin/bash

for i in {0..10000}
do
    echo "msgbus|height:[$i]|key:[key$i]|value:[value$i]" >> tail.log
    sleep 0.001
done
