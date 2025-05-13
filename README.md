# IT Learning Platform

A modular web application built with Rust to help users learn IT skills from beginner to advanced levels.

## Features

- Modular design with separation of concerns
- Categorized lessons by difficulty level
- Course-based curriculum
- Responsive web design
- Docker containerization

## Prerequisites

- Rust and Cargo (for development)
- Docker and Docker Compose (for containerized deployment)

## Development Setup

1. Clone the repository
2. Install dependencies:
   ```
   cargo build
   ```
3. Run the development server:
   ```
   cargo run
   ```

## Docker Deployment

1. Build and start the container:
   ```
   docker-compose up -d
   ```

2. Find the mapped port for the application:
   ```
   docker ps
   ```
   Look for the port mapping like `0.0.0.0:XXXXX->8080/tcp` where XXXXX is your randomly assigned port.

3. Access the application at `http://localhost:XXXXX`

4. If needed, specify a custom port in the .env file:
   ```
   # In .env file
   PORT=8080       # Internal container port
   HOST_PORT=8888  # External host port
   ```

## Troubleshooting

If you encounter issues with the Docker container:

1. Run the diagnostics script inside the container:
   ```
   docker exec -it learn_it_it_learning_platform_1 /app/diagnostics.sh
   ```

2. Check container logs:
   ```
   docker logs learn_it_it_learning_platform_1
   ```

## Docker Cleanup

To stop and remove all Docker containers and images related to this project:

```
chmod +x docker-cleanup.sh
./docker-cleanup.sh
```

## Project Structure

- `/src`: Rust source code
  - `/modules`: Modular components (courses, lessons, users, auth)
- `/templates`: HTML templates using Tera templating engine
- `/static`: CSS, JavaScript, and other static assets
- `/content`: Lesson markdown files organized by difficulty level

## Adding Content

1. Create markdown files in the appropriate level directory under `/content`
2. Add course information to `/content/courses.json`
3. Restart the application

## License

MIT
