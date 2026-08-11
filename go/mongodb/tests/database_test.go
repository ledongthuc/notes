package tests

import (
	"context"
	"testing"

	"github.com/stretchr/testify/require"
	"go.mongodb.org/mongo-driver/v2/bson"
)

func TestDatabase(t *testing.T) {
	t.Parallel()

	client := GetMongoDBClient()
	defer func() {
		if err := client.Disconnect(context.TODO()); err != nil {
			panic(err)
		}
	}()

	/* List database names */
	databaseNames, err := client.ListDatabaseNames(context.Background(), bson.D{})
	require.NoError(t, err, "error ListDatabaseNames")
	require.Contains(t, databaseNames, "local", "List database name")

	/* List databases */
	databases, err := client.ListDatabases(context.Background(), bson.D{})
	require.NoError(t, err, "error ListDatabases")
	require.Greater(t, len(databases.Databases), 0, "number of databases")
}
