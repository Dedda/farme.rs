#!/usr/bin/env bash

usage() {
cat <<EOF
Usage: $0 [options]

-h              show this help and abort
-e              Write database connection URL to .env file.
-u <USERNAME>   Username for database user
-p <PASSWORD>   Password for database user
-H <HOST>       Hostname used in connection URL
-d <DATABASE>   Name used for database
-P <PORT>       Database port for container to publish to host
-s <PATH>       Path to be mounted as volume for postgres data
-n <NAME>       Name of container
-N              Max connections. Will write N-3 to the config to leave some room for extra connections.
-D              Run container as daemon
-t              Test mode. Delete container after stopping
EOF
}

validate_min_length() {
  local ARG=$1
  local MIN_LENGTH=$2
  local NAME=$3
  if [ ${#ARG} -lt "$MIN_LENGTH" ]; then
    echo "$NAME must be at least $MIN_LENGTH but was only ${#ARG}"
    echo
    usage
    exit 1
  fi
}

USERNAME='farmers'
PASSWORD='farmers'
HOST='localhost'
DATABASE='farmers'
PORT=5432
STORAGE=
NAME='pgfarm'
MAX_CONNECTIONS=100
DAEMON_FLAG=
TEST_FLAG=

while getopts eu:p:H:d:P:s:n:Dth FLAG
do
  case "${FLAG}" in
    e) WRITE_ENV=1;;
    u) USERNAME=${OPTARG};;
    p) PASSWORD=${OPTARG};;
    H) HOST=${OPTARG};;
    d) DATABASE=${OPTARG};;
    P) PORT=${OPTARG};;
    s) STORAGE=${OPTARG};;
    n) NAME=${OPTARG};;
    D) DAEMON_FLAG='-d';;
    t) TEST_FLAG='--rm';;
    h) usage && exit 0;;
    *) ;;
  esac
done

set -e

validate_min_length "$USERNAME" 3 'USERNAME (-u)'
validate_min_length "$PASSWORD" 3 'PASSWORD (-p)'
validate_min_length "$HOST" 3 'HOST (-h)'
validate_min_length "$DATABASE" 3 'DATABASE (-d)'
validate_min_length "$NAME" 2 'NAME (-n)'

if ! [[ $PORT =~ ^[0-9]+$ ]]; then
  echo 'port (-P) must be a number';
  echo
  usage
  exit 1
fi

set +e

URL="postgres://${USERNAME}:${PASSWORD}@${HOST}:${PORT}/${DATABASE}"
echo "database url is ${URL}"

if [ -n "${WRITE_ENV}" ]; then
  echo 'updating env file...'
  sed  -i "s|^ROCKET_DATABASES.*$|ROCKET_DATABASES={pgfarm={url=\"${URL}\",pool_size=$(($MAX_CONNECTIONS-3))}}|" .env
fi

if [[ -z $STORAGE ]]; then
  docker run --name "$NAME" \
    -e POSTGRES_PASSWORD=$PASSWORD \
    -e POSTGRES_USER=$USERNAME \
    -e POSTGRES_DB=$DATABASE \
    -p $PORT:5432 \
    $DAEMON_FLAG \
    $TEST_FLAG \
    postgres \
    postgres -N $MAX_CONNECTIONS
else
  docker run --name "$NAME" \
    -e POSTGRES_PASSWORD=$PASSWORD \
    -e POSTGRES_USER=$USERNAME \
    -e POSTGRES_DB=$DATABASE \
    -p $PORT:5432 \
    -v $STORAGE:/var/lib/postgresql/data \
    $DAEMON_FLAG \
    $TEST_FLAG \
    postgres \
    postgres -N $MAX_CONNECTIONS
fi
