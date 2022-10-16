# Database Manager

## Summary

The Database Manager repo is an internal tool to better manage the state of projects, skills and dev-logs on [HasinZaman.link](http://hasinzaman.link).

Before the creation of this tool - the database would be updated in a inefficient and slow process. In which, a script would delete the entire DB. Followed by executing a series of SQL commands to generate a new DB. However, this process has a couple flaws. 

 1. Excessive Operations

The current database tool requires a hard reset, followed by reconstructing the database from scratch - in order to update the database. This was quite useful during the early development of the database. As, the low tuple count and frequent schema updates - ment it was more convenient to do a hard reset and reconstruction. Compared to updating/editing/adding/deleting limited number of tuples or updating tuples to abide by an updated schema.

However, as the schema becomes more stable and the number of tuples in relations balloon. The current database tool is excessively slow and not scalable.

Rather than fully destroying and reconstructing the database - the new database manager would add, edit, delete tuples, without the full reconstruction of database. The new database manager would only reconstruct the database from scratch when schemas are changed. Since, the schema have become more stable - the need for hard resets have become increasingly rare.

 2. Hard to View Database State

 3. Difficult to Execute Complicated or Long Tuples

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