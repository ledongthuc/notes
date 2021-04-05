provider "aws" {
  region = "ap-southeast-2"
}

resource "aws_iam_role" "iam_for_lambda" {
  name = "iam_for_lambda"

  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": ""
    }
  ]
}
EOF
}

data "aws_iam_policy" "AWSLambdaBasicExecutionRole" {
  arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

resource "aws_iam_role_policy_attachment" "iam_for_lambda_cloudwatch" {
  role       = aws_iam_role.iam_for_lambda.name
  policy_arn = data.aws_iam_policy.AWSLambdaBasicExecutionRole.arn
}

locals {
  path = "/dbbackup"
  function_name = "FakeChimera2"
}

resource "aws_lambda_function" "test_lambda" {
  function_name = local.function_name
  role          = aws_iam_role.iam_for_lambda.arn

  package_type = "Image"
  image_uri = "270431303334.dkr.ecr.ap-southeast-2.amazonaws.com/hello-world:latest"
}

resource "aws_cloudwatch_log_group" "example" {
  name              = "/aws/lambda/${local.function_name}"
  retention_in_days = 7
}

resource "aws_apigatewayv2_api" "api" {
  name          = "api-fake-chimera"
  protocol_type = "HTTP"
  target        = aws_lambda_function.test_lambda.arn
  route_key = "POST ${local.path}"
}

resource "aws_lambda_permission" "apigw" {
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.test_lambda.arn
  principal     = "apigateway.amazonaws.com"
  source_arn = "${aws_apigatewayv2_api.api.execution_arn}/*/*"
}

output "base_url" {
  value = "${aws_apigatewayv2_api.api.api_endpoint}${local.path}"
}
