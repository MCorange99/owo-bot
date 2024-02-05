#!/usr/bin/bash

git pull origin main
sudo docker buildx build -t owo_bot .