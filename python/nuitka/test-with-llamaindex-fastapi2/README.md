# FastAPI Nuitka Dockerized App

This project uses a multistage Dockerfile to build a FastAPI application into a standalone binary using Nuitka. The final Docker image is minimal and only includes the essential runtime dependencies.

## Building the Image

To build the Docker image, run the following command:

```sh
docker build -t fastapi-nuitka-app .
```

## Running the Image

To run the Docker image, use the following command:

```sh
docker run -p 8000:8000 fastapi-nuitka-app
```

This will start the FastAPI application and expose it on port 8000.

If you need further tweaks (e.g., static files, environment variables, or additional dependencies), let me know!
```

</rewritten_file>