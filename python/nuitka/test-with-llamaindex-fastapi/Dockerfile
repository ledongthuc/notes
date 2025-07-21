# Stage 1: Build Python environment and binary
FROM python:3.11.12-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    gcc \
    binutils \
    patchelf \
    musl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install uv (fast Python dependency manager & builder)
COPY --from=ghcr.io/astral-sh/uv:0.7.19 /uv /uvx /bin/

# Setup app directory
WORKDIR /app
COPY . .

# Install dependencies into isolated venv using uv
RUN uv sync
RUN uv pip freeze > requirements.txt
RUN uv pip install -r requirements.txt

# OPTIONAL: Use Nuitka to compile Python to binary
RUN python -m pip install nuitka
RUN python -m pip install uvicorn
RUN python -m nuitka --standalone --onefile --output-dir=dist --include-module=main --include-module=uvicorn main.py

# Alternatively: just use uv’s build system
# RUN uv pip install . && uv compile ...

# Stage 2: minimal runtime image
FROM python:3.11.12-slim

# Bring in binary from builder
COPY --from=builder /app/dist/main.bin /app/main

# If using venv instead of static binary
# COPY --from=builder /app/.venv /app/.venv
# COPY --from=builder /app/main.py /app/

# Set entrypoint
ENTRYPOINT ["/app/main"]

# Or if using venv:
# ENTRYPOINT ["/app/.venv/bin/python", "/app/main.py"]
