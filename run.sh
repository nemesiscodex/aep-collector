#!/bin/sh

# exit when any command fails
set -e

echo "Waiting ${WAIT} seconds..."
sleep ${WAIT}

echo "Running migrations..."
/bin/sqlx migrate run
echo "Migrations finished."

echo "Starting app"
/app/aep-collector