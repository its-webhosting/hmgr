#!/bin/bash

# Check if the --file option is provided
if [[ $# -eq 0 || "$1" != "--file" ]]; then
    echo "Usage: $0 --file <path_to_file>"
    exit 1
fi

# Get the path to the file
file_path=$2

# Check if the file exists
if [ ! -f "$file_path" ]; then
    echo "File $file_path does not exist."
    exit 1
fi

# Continuously monitor the file for changes
while true; do
    # Get the timestamp of the file
    previous_timestamp=$(stat -c %Y "$file_path")

    # Wait for the file to change
    inotifywait -q -e modify "$file_path"

    # Get the updated timestamp of the file
    current_timestamp=$(stat -c %Y "$file_path")

    # Check if the file was actually modified
    if [ "$current_timestamp" != "$previous_timestamp" ]; then
        # Append the content of the file to /etc/hosts
        cat "$file_path" >> /etc/hosts
        echo "File $file_path updated. Appended its content to /etc/hosts."
    fi
done
