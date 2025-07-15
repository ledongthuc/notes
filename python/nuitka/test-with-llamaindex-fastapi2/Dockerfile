# --- Builder stage ---
FROM python:3.11-slim AS builder

WORKDIR /app

# Install build dependencies and Nuitka
RUN apt-get update && \
    apt-get install -y --no-install-recommends gcc g++ python3-dev libffi-dev libssl-dev patchelf && \
    pip install --no-cache-dir nuitka fastapi uvicorn[standard]

# Copy project files
COPY main.py ./
COPY pyproject.toml ./

# Compile the FastAPI app with Nuitka
RUN nuitka --standalone --onefile --include-package=main --include-package=fastapi --include-package=uvicorn --output-dir=dist main.py

# --- Runtime stage ---
FROM python:3.11-slim AS runtime

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/dist/main.bin ./main.bin

# Expose the FastAPI port
EXPOSE 8000

# Run the compiled binary
CMD ["./main.bin"] 