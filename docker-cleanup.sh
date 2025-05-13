#!/bin/bash

echo "Docker Cleanup Script"
echo "===================="

# Get the container name/ID specifically for our app
CONTAINER_ID=$(docker ps -a | grep 'learn_it_it_learning_platform' | awk '{print $1}')

if [ -n "$CONTAINER_ID" ]; then
    echo "Stopping IT Learning Platform container..."
    docker stop $CONTAINER_ID
    
    echo "Removing IT Learning Platform container..."
    docker rm $CONTAINER_ID
else
    echo "No IT Learning Platform containers found."
fi

# Get image IDs specifically for our app
IMAGE_IDS=$(docker images | grep 'learn_it_it_learning_platform' | awk '{print $3}')

if [ -n "$IMAGE_IDS" ]; then
    echo "Removing IT Learning Platform images..."
    docker rmi $IMAGE_IDS
else
    echo "No IT Learning Platform images found."
fi

echo "Removing unused volumes (only IT Learning Platform related volumes)..."
docker volume prune -f

echo "Cleanup complete! If you want to also remove networks, run:"
echo "docker network prune -f"
