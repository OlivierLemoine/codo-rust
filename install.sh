#!/bin/bash

if [ ! "`whoami`" = "root" ]
then
    echo "\nPlease run script as root."
    exit 1
fi

apt install curl gcc mysql-server libmysqlclient-dev -y
mysql < base.sql
echo "run \"curl https://sh.rustup.rs -sSf | sh\" to install rust"