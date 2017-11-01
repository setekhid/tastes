<?php

$db_file = $argv[1];
$csv_in = STDIN;

$db = new SQLite3($db_file);

// prepare table
$db->exec("drop table if exists ipv4_loc;");
$db->exec(
<<<sql_table_create
	create table ipv4_loc (
		start   integer,
		end     integer,
		country text,
		primary key (start)
	);
sql_table_create
);

$begin_stamp = time();
//$test_amount = 655350;

// transaction begin
$db->exec("begin transaction;");

// statement
$stmt = $db->prepare(
<<<sql_table_insert
	insert into ipv4_loc (start, end, country)
		values (?, ?, ?);
sql_table_insert
);

// translate
while (($row = fgetcsv($csv_in)) !== FALSE) {

	//if ((--$test_amount) <= 0)
	//	break;

	if (filter_var($row[0], FILTER_VALIDATE_IP, FILTER_FLAG_IPV4)) {

		$stmt->bindValue(1, ip2long($row[0]), SQLITE3_INTEGER);
		$stmt->bindValue(2, ip2long($row[1]), SQLITE3_INTEGER);
		$stmt->bindValue(3, $row[2], SQLITE3_TEXT);
		$result = $stmt->execute();
		$stmt->clear();
	}
}

// transaction end
$db->exec("commit transaction;");

$end_stamp = time();

// recycle
$stmt->close();
$db->close();

echo "totally cost " . ($end_stamp - $begin_stamp) . " seconds to insert\n";
