ps -ef | grep 'chainx -d data --chain=local --port 20001 --pruning archive --bootnodes=/ip4/127.0.0.1/tcp' | grep -v grep | awk '{print $2}' | xargs kill -9
rm -rf data
rm nohup.out
echo FLUSHALL | redis-cli
nohup ./chainx -d data --chain=local --port 20001 --pruning archive --bootnodes=/ip4/127.0.0.1/tcp/31125/p2p/QmWmAU9QGCWLXivuG5NyZqiMggrREq25Z6UxFdZBCimdai &
