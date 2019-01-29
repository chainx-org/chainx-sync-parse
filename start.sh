rm -rf log
rm nohup.out
ps -ef | grep chainx-sub-parse | grep -v grep | awk '{print $2}' | xargs kill -9
nohup ./chainx-sub-parse &
