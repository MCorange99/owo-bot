#!/usr/bin/bash

git pull origin main
sudo docker buildx build -t owo_bot .
sudo docker run -d -l owo_bot -v ./config:/app/config owo_bot