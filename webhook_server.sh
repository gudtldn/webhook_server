#!/bin/bash

process_name="webhook_server"
log_file="./nohup.out"
working_directory="./"

if [ "$1" == "start" ]; then
    # "start" 인자일 경우 지정한 디렉토리로 이동 후 프로세스가 이미 실행 중인지 확인
    cd "$working_directory" || exit

    if pgrep -f "$process_name" > /dev/null; then
        echo "$process_name 프로세스가 이미 실행 중입니다. 종료 후 다시 시도하세요."
        exit 1
    fi

    # 프로세스가 실행 중이 아니라면 nohup으로 백그라운드 실행
    nohup ./target/release/webhook_server > "$log_file" 2>&1 &
    echo "webhook_server를 백그라운드에서 실행했습니다. 로그 파일: $log_file"
elif [ "$1" == "stop" ] || [ "$1" == "kill" ]; then
    # "stop" 또는 "kill" 인자일 경우 프로세스 종료
    pid=$(ps -ef | grep "$process_name" | grep -v grep | awk '{print $2}')

    if [ -z "$pid" ]; then
        echo "$process_name 프로세스를 찾을 수 없습니다."
    else
        kill -15 "$pid"  # 먼저 안전한 종료 시도 (SIGTERM)
        sleep 5  # 5초 대기
        if ps -p "$pid" > /dev/null; then
            kill -9 "$pid"  # 강제 종료 (SIGKILL)
            echo "$process_name 프로세스를 강제 종료했습니다. (PID: $pid)"
        else
            echo "$process_name 프로세스를 종료했습니다. (PID: $pid)"
        fi
    fi
else
    echo "올바른 인자를 입력하세요: start, stop, kill"
fi