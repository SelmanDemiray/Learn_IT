# Use Node for build
FROM node:20.12.2 AS build
WORKDIR /app
COPY package.json package-lock.json* ./
RUN npm install --legacy-peer-deps
COPY . .
RUN npm run build

# Use a lightweight server for static files
FROM node:20.12.2-slim AS prod
WORKDIR /app
RUN npm install -g serve
COPY --from=build /app/dist ./dist
EXPOSE 3000
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s CMD curl -f http://localhost:3000/ || exit 1
CMD ["serve", "-s", "dist", "-l", "3000"]
