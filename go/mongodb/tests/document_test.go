package tests

import (
	"context"
	"testing"

	"github.com/stretchr/testify/require"
	"go.mongodb.org/mongo-driver/v2/bson"
)

func TestDocument(t *testing.T) {
	t.Parallel()

	client := GetMongoDBClient()
	defer func() {
		if err := client.Disconnect(context.TODO()); err != nil {
			panic(err)
		}
	}()
	database := client.Database("local_test_database")
	defer database.Drop(context.Background())

	collection := database.Collection("local_test_collection")
	defer collection.Drop(context.Background())

	type Book struct {
		Title  string
		Author string
	}

	/* Insert */
	doc := Book{
		Title:  "Nothing",
		Author: "Thuc",
	}
	result, err := collection.InsertOne(context.Background(), doc)
	require.NoError(t, err, "err insert")
	require.True(t, result.Acknowledged, "Ack insert")
	require.NotNil(t, result.InsertedID, "InsertedID")

	/* Insert Many */
	docs := []Book{
		{
			Title:  "Nothing 2",
			Author: "Thuc",
		},
		{
			Title:  "Nothing 3",
			Author: "Thuc",
		},
	}
	resultMany, err := collection.InsertMany(context.Background(), docs)
	require.NoError(t, err, "err insert many")
	require.True(t, resultMany.Acknowledged, "Ack insert many")
	require.Len(t, resultMany.InsertedIDs, 2, "InsertedIDs")

	/* Update many */
	updatedResult, err := collection.UpdateMany(context.Background(), bson.D{{Key: "author", Value: "Thuc"}}, bson.M{"$set": bson.E{Key: "author", Value: "Thuc Updated"}})
	require.NoError(t, err, "err update many")
	require.Equal(t, int64(3), updatedResult.ModifiedCount)

	/* Find */
	c, err := collection.Find(context.Background(), bson.D{{Key: "author", Value: "Thuc"}})
	require.NoError(t, err, "err find")
	defer c.Close(context.Background())
	var foundBooks []Book
	err = c.All(context.Background(), &foundBooks)
	require.NoError(t, err, "err find.All()")
	require.Len(t, foundBooks, 3, "number of found books")
}
