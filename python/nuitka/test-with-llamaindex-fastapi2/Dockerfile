# --- Builder stage ---
FROM python:3.11-slim AS builder

WORKDIR /app

# Install build dependencies and Nuitka
RUN apt-get update && \
    apt-get install -y --no-install-recommends gcc g++ python3-dev libffi-dev libssl-dev patchelf && \
    pip install --no-cache-dir nuitka fastapi uvicorn[standard] uv

# Copy project files
COPY main.py ./
COPY pyproject.toml ./

# Install project dependencies
RUN uv sync
RUN pip install -e .

# Compile the FastAPI app with Nuitka
RUN python -c "import site; print(site.getsitepackages())"
RUN nuitka --standalone --onefile --include-data-dir=./.venv/lib/python3.11/site-packages/llama_index=llama_index --include-package=main --include-package=llama_index --include-package=fastapi --include-package=uvicorn --output-dir=dist main.py

# --- Runtime stage ---
FROM python:3.11-slim AS runtime

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/dist/main.bin ./main.bin

# Expose the FastAPI port
EXPOSE 8000

# Run the compiled binary
CMD ["./main.bin"] 