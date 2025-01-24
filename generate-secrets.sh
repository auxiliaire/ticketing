#!/usr/bin/env bash

Files=(
    'secret_mariadb_root_password.txt'
    'secret_mariadb_password.txt'
    'secret_postgres_password.txt'
    'secret_jwt_secret.txt'
    'secret_smtp_password.txt'
)

K8sFiles=(
    'mariadb-root-password-secret.yaml'
    'mariadb-password-secret.yaml'
    'postgres-password-secret.yaml'
    'jwt-secret-secret.yaml'
    'smtp-password-secret.yaml'
)

SAMPLE_EXT=".sample"

echo "Generating secrets..."

for ((i = 0 ; i < ${#Files[@]} ; i++)); do

    if [[ ! -e ${Files[$i]} ]]; then
        SECRET=`openssl rand -base64 20`
        SECRET_ENC=`echo -n ${SECRET} | base64`
        echo $SECRET > ${Files[$i]}
        sed -r "s,<GENERATED_SECRET>,$SECRET_ENC,g" ./k8s/${K8sFiles[$i]}${SAMPLE_EXT} > ./k8s/${K8sFiles[$i]}
        echo "...${Files[$i]}, ${K8sFiles[$i]}"
    fi

done

echo "Done!"
