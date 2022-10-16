# Database Manager

## Purpose

The Database Manager repo is an internal tool to better manage the state of projects, skills and dev-logs on [HasinZaman.link](http://hasinzaman.link).

Before the creation of this tool - the database tool in use has become inefficient, slow process and increasingly obtuse to use. The main issues can be categorized into three groups:

 1. Excessive Operations

The current database tool requires a hard reset, followed by reconstructing the database from scratch - in order to update the database. This was quite useful during the early development of the database. As, the low tuple count and frequent schema updates - ment it was more convenient to do a hard reset and reconstruction. Compared to updating/editing/adding/deleting limited number of tuples or updating tuples to abide by an updated schema.

However, as the schema becomes more stable and the number of tuples in relations balloon. The current database tool is excessively slow and not scalable.

Rather than fully destroying and reconstructing the database - the new database manager would add, edit, delete tuples, without the full reconstruction of database. The new database manager would only reconstruct the database from scratch when schemas are changed. Since, the schema have become more stable - the need for hard resets have become increasingly rare.

 2. Hard to View Database State

The current database tool has no ability to view the state of the Database. As a result, it is required to use a slow online database tool(MyPHPAdmin) provided by the database provider. Since the current database is structured to be similar to a graph of tags. It is quite difficult to see the relationship between tags. This is why, the new database should provide a couple viewing options to better conceptualize the database.

Firstly, the new tool should be able to easily show the relationship between schema. For instance, the colour highlighting of primary and foreign keys.

Secondly, the new tool should be easily filter tags by type, name and relationships. For instance, this would be useful in conceptualize the relationship between a project and skills; or the link of dev-logs; or even the similarly of skills (ex. the relationship between Web-Dev & HTML, CSS, JS, TS, etc.).

 3. Difficult to Execute Long and Single SQL Commands

The Current database tool sends commands by reading each line of a text file - in which, each line is an SQL command. This has the advantage of storing numerous commands in sequence in a single file.

However, this input system breaks down as longer and more complicated commands. For instance, the implementation of a dev-log would require multi-line command parsing - as the alternative to putting entire dev-logs in one line. Not to mention, queries with sub-queries are easier to read and debug when formatted on multiple lines. This is why, it is important for the new database tool to parse multi-line commands and long input attributes.

This input system also is excessive for short SQL commands. For instance an impromptu "SELECT * FROM Relation" call - takes too long to execute. As the command needs to be put in text file, saved, then opened by the database tool and then executed. The process can be sped up - if the database tool is able to receive SQL commands in the command prompt on run time.

## Design Documents
### Database Structure
| ![Database Entity Diagram](desgin_documentation//DB_Entity_Diagram.svg "Database Entity Diagram") |
|:--:|
| Database Entity Diagram |

 1. Tag
 2. Related
 3. Project
 4. Dev Log
### UI
| ![Database UI State Diagram](desgin_documentation//DBM_State_Diagram.svg "Database UI State Diagram") |
|:--:|
| Database Manager State Diagram |