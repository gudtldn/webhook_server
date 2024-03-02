#!/bin/bash

process_name="webhook_server"
log_file="./nohup.out"
working_directory="./"

if [ "$1" == "start" ]; then
    cd "$working_directory" || exit

    if pgrep -f "$process_name" | grep -v $$ > /dev/null; then
        echo "$process_name 프로세스가 이미 실행 중입니다. 종료 후 다시 시도하세요."
        exit 1
    fi

    nohup ./target/release/webhook_server > "$log_file" 2>&1 &
    echo "webhook_server를 백그라운드에서 실행했습니다. 로그 파일: $log_file"
elif [ "$1" == "stop" ] || [ "$1" == "kill" ]; then
    pid=$(ps -ef | grep "$process_name" | grep -v grep | awk '{print $2}')

    if [ -z "$pid" ]; then
        echo "$process_name 프로세스를 찾을 수 없습니다."
    else
        IFS=' ' read -ra pid_array <<< "$pid"
        for id in "${pid_array[@]}"; do
            kill -15 "$id"
            sleep 5
            if ps -p "$id" > /dev/null; then
                kill -9 "$id"
                echo "$process_name 프로세스를 강제 종료했습니다. (PID: $id)"
            else
                echo "$process_name 프로세스를 종료했습니다. (PID: $id)"
            fi
        done
    fi
else
    echo "올바른 인자를 입력하세요: start, stop, kill"
fi
