FROM rust:1.82 as builder

WORKDIR /usr/src/app
COPY . .

# Install OpenSSL development packages required for the build
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Build the application (standard dynamic build)
RUN cargo build --release

# Change from Debian to Ubuntu 22.04 which has GLIBC 2.35
FROM ubuntu:22.04

# Install OpenSSL and CA certificates for HTTPS plus additional required packages
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    libssl-dev \
    curl \
    netbase \
    tzdata \
    sqlite3 \
    procps \
    net-tools \
    nodejs \
    npm \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/it_learning_platform /app/
# Copy static files and templates
COPY --from=builder /usr/src/app/static /app/static
COPY --from=builder /usr/src/app/templates /app/templates
COPY --from=builder /usr/src/app/content /app/content

# Copy scripts
COPY --from=builder /usr/src/app/diagnostics.sh /app/
COPY --from=builder /usr/src/app/start.sh /app/
COPY --from=builder /usr/src/app/init-db.sh /app/
COPY --from=builder /usr/src/app/fallback-server.js /app/

# Make scripts executable
RUN chmod +x /app/diagnostics.sh /app/start.sh /app/init-db.sh

# Set environment variables
ENV HOST=0.0.0.0
ENV RUST_BACKTRACE=1
# If PORT is not set, a random port will be selected at runtime

# Create necessary directories
RUN mkdir -p /app/data && chmod 777 /app/data

# Initialize the database
RUN /app/init-db.sh

EXPOSE 8000-9000

CMD ["/app/start.sh"]
