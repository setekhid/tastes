package main

import (
	"testing"

	"github.com/gocql/gocql"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func prepare(t *testing.T, cluster *gocql.ClusterConfig) {

	c := *cluster
	c.Keyspace = "system"

	session, err := c.CreateSession()
	require.NoError(t, err)
	defer session.Close()

	err = session.Query("create keyspace if not exists taste with replication = { 'class': 'SimpleStrategy', 'replication_factor': 1 };").Exec()
	require.NoError(t, err)

	err = session.Query("create table if not exists taste.tweet(timeline text, id UUID, text text, PRIMARY KEY(id));").Exec()
	require.NoError(t, err)

	err = session.Query("create index if not exists on taste.tweet(timeline);").Exec()
	require.NoError(t, err)
}

func TestBasicUsage(t *testing.T) {

	cluster := gocql.NewCluster("127.0.0.1")
	prepare(t, cluster)

	cluster.Keyspace = "taste"
	cluster.Consistency = gocql.Quorum

	session, err := cluster.CreateSession()
	require.NoError(t, err)
	defer session.Close()

	err = session.Query("INSERT INTO tweet (timeline, id, text) VALUES (?, ?, ?)", "me", gocql.TimeUUID(), "hello world").Exec()
	require.NoError(t, err)

	var id gocql.UUID
	var text string

	err = session.Query("SELECT id, text FROM tweet WHERE timeline = ? LIMIT 1", "me").Consistency(gocql.One).Scan(&id, &text)
	assert.NoError(t, err)

	t.Log(id, text)
}
