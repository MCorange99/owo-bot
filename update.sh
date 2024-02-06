#!/usr/bin/bash

git pull origin main
sudo docker buildx build -t owo_bot .
sudo docker run -d --name owo_bot -v ./config:/app/config owo_bot