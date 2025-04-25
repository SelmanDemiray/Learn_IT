# IT Beginner Academy

A modern, interactive web platform for learning IT fundamentals from scratch.

## Features

- **Step-by-step IT lessons** with quizzes, projects, and fun facts
- **Progress tracking** and completion badges
- **Interactive IT Dictionary** with search
- **Responsive design** (mobile/tablet/desktop)
- **Dockerized deployment** for easy setup
- **Dark mode** and accessibility features

## Getting Started

### Local Development

```sh
npm install
npm run dev
```

Visit [http://localhost:3000](http://localhost:3000) in your browser.

### Docker Deployment

Build and run with Docker Compose (random host port):

```sh
docker compose up --build
```

After startup, check the output for the mapped port (e.g., `0.0.0.0:49153->3000/tcp`).  
Open `http://localhost:<port>` in your browser.

### Build for Production

```sh
npm run build
npm run start
```

## Contributing

Pull requests welcome! Please lint and format your code:

```sh
npm run lint
npm run format
```

## License

MIT
