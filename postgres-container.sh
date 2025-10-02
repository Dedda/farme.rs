#!/usr/bin/env bash

validate_min_length() {
  _ARG=$1
  _MIN_LENGTH=$2
  _NAME=$3
  if [ ${#_ARG} -lt "$_MIN_LENGTH" ]; then
    echo "$_NAME must be at least $_MIN_LENGTH but was only ${#_ARG}"
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
DAEMON_FLAG=

while getopts eu:p:h:d:P:s:n:D FLAG
do
  case "${FLAG}" in
    e) WRITE_ENV=1;;
    u) USERNAME=${OPTARG};;
    p) PASSWORD=${OPTARG};;
    h) HOST=${OPTARG};;
    d) DATABASE=${OPTARG};;
    P) PORT=${OPTARG};;
    s) STORAGE=${OPTARG};;
    n) NAME=${OPTARG};;
    D) DAEMON_FLAG='-d';;
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
  echo 'port (-P) must be a number'; exit 1
fi

set +e

URL="postgres://${USERNAME}:${PASSWORD}@${HOST}:${PORT}/${DATABASE}"
echo "database url is ${URL}"

if [ -n "${WRITE_ENV}" ]; then
  echo 'updating env file...'
  sed -i "s/ROCKET_DATABASES.*/ROCKET_DATABASES={pgfarm={url=\"${URL}\"}}/" .env
#  echo "ROCKET_DATABASES={pgfarm={url=\"${URL}\"}}" > .env
fi

if [[ -z $STORAGE ]]; then
  docker run --name "$NAME" \
    -e POSTGRES_PASSWORD=$PASSWORD \
    -e POSTGRES_USER=$USERNAME \
    -e POSTGRES_DB=$DATABASE \
    -p $PORT:5432 \
    $DAEMON_FLAG \
    postgres
else
  docker run --name "$NAME" \
    -e POSTGRES_PASSWORD=$PASSWORD \
    -e POSTGRES_USER=$USERNAME \
    -e POSTGRES_DB=$DATABASE \
    -p $PORT:5432 \
    -v $STORAGE:/var/lib/postgresql/data
    $DAEMON_FLAG \
    postgres
fi
