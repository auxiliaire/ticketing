#!/usr/bin/env bash

# START SCRIPT FOR THE WHOLE TICKETING STACK

# Constants:

DEV_DIR="./dev"
MIGRATIONS_DIR="./extra_migrations/fang"
SAMPLE_ENV_FILE=".env.sample"
ENV_FILE=".env"
EXPECTED_NUMBER_OF_CONTAINERS=6

# Reusable functions:

hasInstalled() {
  if ! command -v "$1" >/dev/null 2>&1
  then
    return 1
  fi
}

# Checking requirements:

# Generating color palette
if [ "$(tput colors)" -ge 8 ]; then
  RED=$(tput setaf 1)
  GREEN=$(tput setaf 2)
  YELLOW=$(tput setaf 3)
  BLUE=$(tput setaf 4)
  WHITE=$(tput setaf 7)
  BOLD=$(tput bold)
  RESET=$(tput sgr0)
else
  RED=""
  GREEN=""
  YELLOW=""
  BLUE=""
  WHITE=""
  BOLD=""
  RESET=""
fi

printf "%sWelcome to %sTicketing in Rust%s\n" "$WHITE" "$BLUE" "$RESET"
sleep 2

printf "%sChecking requirements...%s\n" "$BOLD" "$RESET"

if ! hasInstalled "docker"
then
  printf "%sFATAL%s: Docker could not be found (try installing it)\n" "$RED" "$RESET"
  exit 1
else
  printf "Docker found ✅\n"
fi

if ! hasInstalled "cargo"
then
  printf "%sFATAL%s: Cargo could not be found (try to install Rust toolchain)\n" "$RED" "$RESET"
  exit 1
else
  printf "Cargo found ✅\n"
fi

if ! hasInstalled "diesel"
then
  printf "Diesel could not be found %s❌%s\n" "$RED" "$RESET"
  printf "Trying to install Diesel... "
  if ! cargo install diesel_cli --no-default-features --features "postgres sqlite mysql"; then
    printf "%sFAILED%s\n" "$RED" "$RESET"
    exit 1
  fi
  if ! hasInstalled "diesel"
  then
    printf "%sFAILED%s\n" "$RED" "$RESET"
    exit 1
  else
    printf "%sDONE%s\n" "$GREEN" "$RESET"
  fi
else
  printf "Diesel found ✅\n"
fi

if ! [[ -d $DEV_DIR ]]; then
  printf "%sFATAL%s: directory '%s' is missing (try pull)\n" "$RED" "$RESET" $DEV_DIR
  exit 1
fi

if ! [[ -e $ENV_FILE ]]; then
  echo "Creating environment... "
  if [[ -e $SAMPLE_ENV_FILE ]]; then
    cp $SAMPLE_ENV_FILE $ENV_FILE
    # shellcheck source=.env
    source $ENV_FILE
  else
    printf "%sFATAL%s: sample environment file '%s' is missing (try pull)\n" "$RED" "$RESET" $SAMPLE_ENV_FILE
    exit 1
  fi
  printf "%sDONE%s\n" "$GREEN" "$RESET"
  printf "%sConsider changing default passwords in .env file!%s\n" "$YELLOW" "$RESET"
fi

if ! [[ -v CLIENT_PORT ]]
then
  printf "%sFATAL%s: CLIENT_PORT environment variable not set (check .env file and/or source it)\n" "$RED" "$RESET"
  exit 1
else
  printf "CLIENT_PORT found ✅\n"
fi

if ! [[ -v POSTGRES_URL ]]
then
  printf "%sFATAL%s: POSTGRES_URL environment variable not set (check .env file and/or source it)\n" "$RED" "$RESET"
  exit 1
else
  printf "POSTGRES_URL found ✅\n"
fi

# Starting containers:

printf "%sStarting containers (Ctrl-C to abort)...%s " "$BOLD" "$RESET"

cd $DEV_DIR || exit 1
trap 'docker compose down' SIGINT; docker compose up --detach
until false
do
  number_of_running_containers=$(docker compose ps | grep -c "Up")
  if [[ $number_of_running_containers -eq $EXPECTED_NUMBER_OF_CONTAINERS ]]; then
    break
  fi
done
printf "%sDONE%s\n" "$GREEN" "$RESET"
cd .. || exit 1

# Running Diesel migration:

printf "%sRunning migrations...%s " "$BOLD" "$RESET"
if ! diesel migration run --database-url "$POSTGRES_URL" --migration-dir $MIGRATIONS_DIR; then
  printf "%sFAILED%s\n" "$RED" "$RESET"
  exit 1
fi
printf "%sDONE%s\n" "$GREEN" "$RESET"

printf "%sStartup requirements fulfilled ✅%s\n" "$GREEN" "$RESET"
printf "%sComponent requirements might still be lacking\n" "$YELLOW"
printf "Check the corresponding output for hints%s\n" "$RESET"

# Providing info:

echo "${BOLD}Starting Application backend & frontend...${RESET}"
printf "%sCtrl-C to exit both%s\n" "$YELLOW" "$RESET"
printf "%sThen 'docker compose down' to remove containers.%s\n" "$YELLOW" "$RESET"
sleep 2

# Starting dev backend and frontend:
(trap 'kill 0' SIGINT; (trap 'kill 0' SIGINT; cargo -Z unstable-options -C ./ watch -c -w src -x run) &\
 (cd frontend || exit; trunk serve --port="${CLIENT_PORT}") & wait)
