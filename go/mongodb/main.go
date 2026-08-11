package main

import (
	"context"
	"fmt"

	"mongodbtest/tests"

	"go.mongodb.org/mongo-driver/v2/bson"
)

func main() {
	client := tests.GetMongoDBClient()
	defer func() {
		if err := client.Disconnect(context.TODO()); err != nil {
			panic(err)
		}
	}()

	var result bson.M
	if err := client.Database("admin").
		RunCommand(context.TODO(), bson.D{{Key: "ping", Value: 1}}).Decode(&result); err != nil {
		panic(err)
	}
	fmt.Println("Pinged your deployment. You successfully connected to MongoDB!")
}
