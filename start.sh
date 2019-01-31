ps -ef | grep chainx-sub-parse | grep -v grep | awk '{print $2}' | xargs kill -9
rm -rf log
rm nohup.out
nohup ./chainx-sub-parse &
