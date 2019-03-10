#!/bin/bash
power=25
while :
do
    echo "trying elf attack power $power"
    python3 day15.py $power 
    rc=$? 
    if [[ $rc == 0 ]]; then 
        echo "attack power $power was sufficient!"
        exit 0
    fi

    ((power++))
done
