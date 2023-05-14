package main

import (
	"database/sql"
	"fmt"
	"log"

	_ "github.com/glebarez/go-sqlite"
)

func main() {
	// connect
	db, err := sql.Open("sqlite", "./test.db")
	if err != nil {
		log.Fatal(err)
	}

	// get SQLite version
	r := db.QueryRow("select sqlite_version()")
	var i interface{}
	r.Scan(&i)
	fmt.Println("DEBUG: ", i)

	if _, err = db.Exec(`
drop table if exists t;
create table t(i);
insert into t values(42), (314);
`); err != nil {
		panic(err)
	}

	rows, err := db.Query("select 3*i from t order by i;")
	if err != nil {
		panic(err)
	}

	for rows.Next() {
		var i int
		if err = rows.Scan(&i); err != nil {
			panic(err)
		}

		fmt.Println(i)
	}

	if err = rows.Err(); err != nil {
		panic(err)
	}

	if err = db.Close(); err != nil {
		panic(err)
	}
}
