ps -ef | grep chainx-sync-parse | grep -v grep | awk '{print $2}' | xargs kill -9
rm -rf log
rm nohup.out
nohup ./chainx-sync-parse &
