uvicorn main:app --port 8106

# Build Docker image
docker build -t test-with-llamaindex-fastapi:latest .

# Run Docker container
docker run -p 8106:8106 -e OPENAI_API_KEY=$OPENAI_API_KEY test-with-llamaindex-fastapi
