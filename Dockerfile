# Use a Rust image as the base image
FROM rust:latest

ENV DATABASE_URL = "mysql://test_user:test_password@mysql:3306/test_db"

# Set the working directory inside the container
WORKDIR /usr/src/migration

# Copy the entire local Rust project into the container
COPY ./migration .

RUN git clone https://github.com/vishnubob/wait-for-it.git

# Build the Rust project
RUN cargo build --release

# Specify the default command to run when the container starts
CMD ["./wait-for-it/wait-for-it.sh", "mysql:8082", "--", "cargo", "run", "--manifest-path", "Cargo.toml", "--", "refresh", "-u", "$DATABASE_URL" ]