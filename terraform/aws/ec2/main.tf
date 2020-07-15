provider "aws" {
  profile = "default"
  region = var.region
}

resource "aws_security_group" "allow_ssh" {
  name        = "allow_ssh"
  description = "Allow ssh inbound traffic"

  ingress {
    description = "TLS from VPC"
    from_port   = 22
    to_port     = 22
    protocol    = "TCP"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "allow_ssh"
  }
}


resource "aws_key_pair" "example" {
  key_name   = "examplekey"
  public_key = file("~/.ssh/example.pub")
}

resource "aws_instance" "example" {
  key_name      = aws_key_pair.example.key_name
  ami           = "ami-04590e7389a6e577c"
  instance_type = "t2.micro"
  vpc_security_group_ids = ["${aws_security_group.allow_ssh.id}"]

  connection {
    type        = "ssh"
    user        = "ec2-user"
    private_key = file("~/.ssh/example")
    host        = self.public_ip
  }

  provisioner "remote-exec" {
    inline = [
      "sudo amazon-linux-extras enable nginx1.12",
      "sudo yum -y install nginx",
      "sudo systemctl start nginx"
    ]
  }
}
