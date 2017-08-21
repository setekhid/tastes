package main

import (
	"context"
	"github.com/stretchr/testify/require"
	"gopkg.in/olivere/elastic.v5"
	"testing"
)

func TestBasicTaste(t *testing.T) {

	bgctx := context.Background()

	client, err := elastic.NewSimpleClient(
		elastic.SetURL("http://localhost:9201", "http://localhost:9202"),
		elastic.SetBasicAuth("elastic", "changeme"),
	)
	require.NoError(t, err)

	exists, err := client.IndexExists("twitter").Do(bgctx)
	require.NoError(t, err)

	if !exists {
		_, err = client.CreateIndex("twitter").Do(context.Background())
		require.NoError(t, err)
	}

	type Tweet struct {
		User    string `json:"user"`
		Message string `json:"message"`
	}

	tweet := Tweet{
		User:    "oliwho",
		Message: "take five",
	}

	_, err = client.Index().
		Index("twitter").
		Type("tweet").
		Id("1").
		BodyJson(tweet).
		Refresh("true").
		Do(context.Background())
	require.NoError(t, err)

	term_query := elastic.NewTermQuery("user", "oliwho")
	search_result, err := client.Search().
		Index("twitter").
		Query(term_query).
		Sort("user", true).
		From(0).Size(10).
		Pretty(true).
		Do(context.Background())
	require.NoError(t, err)

	t.Logf("query took %d milliseconds\n", search_result.TookInMillis)

	// TODO & FIXME this file is modified from https://git.io/v5fFK, but broken
}
