#!/bin/bash

#only one arg (the day you are creating for)
argv=($@)
argc=($#)

if [ $argc -eq 1 ]
then
  if [ -f ./src/day* ]
  then
    mv ./src/day* ./src/2024/
  fi
  cp ./src/templates/day_template.rs ./src/day${argv[0]}.rs
  cp ./src/templates/main_template.rs ./src/main.rs
  touch ./input/day${argv[0]}.txt
  sed -i ''s/dayx/day${argv[0]}/g'' ./src/main.rs
  sed -i ''s/dayx/day${argv[0]}/g'' ./src/day${argv[0]}.rs
else
  echo "incorrect number of args"
fi

