#!/usr/bin/env bash

Files=(
    'secret_mariadb_root_password.txt'
    'secret_mariadb_password.txt'
    'secret_postgres_password.txt'
    'secret_jwt_secret.txt'
    'secret_smtp_password.txt'
)

for i in "${Files[@]}"; do

    if [[ ! -e $i ]]; then
        openssl rand -base64 20 > $i
    fi

done
