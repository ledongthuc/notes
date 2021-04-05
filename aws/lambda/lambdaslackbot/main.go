package main

import (
	"encoding/base64"
	"net/url"
	"os"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/sirupsen/logrus"
)

func HandleRequest(request events.APIGatewayProxyRequest) (events.APIGatewayProxyResponse, error) {
	logrus.SetFormatter(&logrus.JSONFormatter{})
	logrus.WithFields(logrus.Fields{
		"env":               os.Environ(),
		"request_id":        request.RequestContext.RequestID,
		"method":            request.HTTPMethod,
		"path":              request.Path,
		"res":               request.Resource,
		"body":              request.Body,
		"is_base64_encoded": request.IsBase64Encoded,
	}).Info("New request")

	body := request.Body
	if request.IsBase64Encoded {
		b, err := base64.URLEncoding.DecodeString(request.Body)
		if err != nil {
			logrus.Error(err)
			return events.APIGatewayProxyResponse{
				Body:       "Fail to process: decode request: " + err.Error(),
				StatusCode: 200,
			}, nil
		}
		body = string(b)
	}

	arguments, err := url.ParseQuery(body)
	if err != nil {
		return events.APIGatewayProxyResponse{
			Body:       "Fail to process: parse request: " + err.Error(),
			StatusCode: 200,
		}, nil
	}
	logrus.WithFields(logrus.Fields{
		"arguments": arguments,
	}).Info("Arguments")

	msg := awsInsatncesMsg()
	return events.APIGatewayProxyResponse{
		Body:       msg,
		StatusCode: 200,
	}, nil
}

func main() {
	lambda.Start(HandleRequest)
}
func awsInsatncesMsg() string {
	return "Hello guy\nNice to meet you<br/>Good bye"
}
