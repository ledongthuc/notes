# Read metadata registry
data "docker_registry_image" "nginx" {
  name = "nginx:latest"
}

# Pull image
resource "docker_image" "nginx" {
  name         = "${data.docker_registry_image.nginx.name}"
  keep_locally = false # Delete image from docker local storage on destroy opration
  pull_triggers = ["${data.docker_registry_image.nginx.sha256_digest}"]
}

resource "docker_container" "nginx" {
  image = docker_image.nginx.latest
  name  = "tutorial"
  ports {
    internal = 80
    external = 8000
  }
}
