package tests

import (
	"context"
	"testing"

	"github.com/stretchr/testify/require"
	"go.mongodb.org/mongo-driver/v2/bson"
)

func TestCollection(t *testing.T) {
	t.Parallel()

	client := GetMongoDBClient()
	defer func() {
		if err := client.Disconnect(context.TODO()); err != nil {
			panic(err)
		}
	}()
	database := client.Database("local_test_collection")
	defer database.Drop(context.Background())

	/* Create Collection */
	err := database.CreateCollection(context.Background(), "TestCollection")
	require.NoError(t, err)

	/* List Collection names */
	collectionNames, err := database.ListCollectionNames(context.Background(), bson.D{})
	require.NoError(t, err, "list collection names")
	require.Contains(t, collectionNames, "TestCollection")

	/* List Collections */
	collections, err := database.ListCollections(context.Background(), bson.D{})
	require.NoError(t, err, "list collections")
	foundCollection := false
	var collectionInfo bson.M
	for collections.Next(context.Background()) {
		err = collections.Decode(&collectionInfo)
		require.NoError(t, err)
		name, ok := collectionInfo["name"].(string)
		require.True(t, ok, "get name from collectionInfo")

		if name == "TestCollection" {
			foundCollection = true
			break
		}
	}
	if !foundCollection {
		require.Fail(t, "Don't found collection")
	}

	/* Drop collection */
	err = database.Collection("TestCollection").Drop(context.Background())
	require.NoError(t, err, "drop collection")
}
