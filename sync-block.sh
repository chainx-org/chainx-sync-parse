ps -ef | grep 'chainx --dev -d data --port 20001 --bootnodes=/ip4/127.0.0.1/tcp' | grep -v grep | awk '{print $2}' | xargs kill -9
rm -rf data
rm nohup.out
echo FLUSHALL | redis-cli
nohup ./chainx --dev -d data --port 20001 --bootnodes=/ip4/127.0.0.1/tcp/30000/p2p/QmPrQbZVFkSd1Ffof1LGoLHF2Qq8jqoUsQNEmcAzwaE426 &
