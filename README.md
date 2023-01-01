# Database Manager
## Description

Database Manager is a rust, front-end, client-side application to quickly manage and view the state of a MySQL database. It was primarily developed to better manage the database behind HasinZaman.link.

More information about the project can be found at [hasinzaman.link/Database_Manager](hasinzaman.link/Database_Manager).

## Installation

### 1. Download

#### From Repository

Download the repository on your system

Use `Cargo run` to run the source code or `cargo build` to build an executable

#### From Release

Download and unzip one of the releases

### 2. Initialize Environmental Variables

The application requires certain environmental variables to be set to function.

```
DB_host : the host address of your database
DB_port : the port your database is hosted on
DB_name : the name of the database
DB_username : username of the user on the database
DB_password : password of the user on the database
```

## Manual

### general
 
The application can only be navigated through the commands inputted to the console.

 - All `letter`, `number` and `symbol` keys input characters into the console
 - `Enter` key is used to execute commands
 - `Down Arrow` key retrieves the previously executed command
 - `Up Arrow` key retrieves the next most recently executed command or draft command

### Schema Tab

The schema tab is used to see the architecture of the database

Any of the following commands open's the Schema Tab

 - `show *` command is used to show the definition of every table and view the database
 - `show views` command is used to show the definition of every view on the database
 - `show tables` command is used to show the definition of every table on the database
 - `show [table or view name]` command is used to show all the details relating to the definition of a specific table or view

Note: Font colour is used to refer to the primary key constraint, while highlighter colour is used to refer to the foreign key constraint. The same font and highlight colour refer to a primary-foreign key relation.

### Query Tab

The query tab is used to see the state of tuples in views and tables in the database

Only valid `SELECT` SQL commands are used to open the Query Tab

- `SELECT ...` commands define which tuples are viewed
- `next` command is used to get the next page of tuples
- `prev` command is used to get the previous page of tuples

ex. Show all the tuples that exist in the natural join of two tables

```SQL
SELECT * FROM table_1, table_2 WHERE table_1.id = table_2.id
```

ex. Show all tuples greater than 30

```SQL
SELECT * FROM table_1 WHERE table_1.n > 30
```

### Snapshot Tab

The snapshot tab is used to add, remove or rollback the database to a given point in the database's history

Only a `snapshot` command can open the Snapshot page

 - `snapshot` command opens the snapshot page
 - `add` command adds a new snapshot from the current database
 - `remove [row or name]` command removes the snapshot
 - `rollback [row or name]` command is used to rollback the database to the specified snapshot

## License
Distributed under the MIT License. See `LICENSE.md` for more information.
