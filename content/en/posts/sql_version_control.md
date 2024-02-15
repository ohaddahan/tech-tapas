+++
title = 'How to embed version control in SQL'
date = 2024-02-14T09:22:33+02:00
draft = false
+++

![sql meme](https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/d7e8b24c-e490-4c11-1f5d-62ae8191fc00/public)

## Background

In one of the projects I worked on, I was managing users balances.
Similar to how a bank account works.

Obviously given the nature of the data, I wanted to ensure maximum safety and ability to detect and recover from issues.
I was looking for a way to track any change to any database row.

## Existing solutions

I was familiar with some [ORM](https://en.wikipedia.org/wiki/Object%E2%80%93relational_mapping) based solutions
such as [paper_trail](https://github.com/paper-trail-gem/paper_trail).
But I no longer use [ORM](https://en.wikipedia.org/wiki/Object%E2%80%93relational_mapping) and prefer query builders.

## Solution

After some research I found a great
article [History Tracking with Postgres](https://www.thegnar.com/blog/history-tracking-with-postgres)
that had just what I was looking for.

### Trigger function

```sql
CREATE TABLE archives
(
    id          uuid               DEFAULT gen_random_uuid() PRIMARY KEY,
    sid         BIGSERIAL NOT NULL UNIQUE,
    table_name  TEXT      NOT NULL,
    record_type TEXT      NOT NULL,
    record_id   uuid      NOT NULL,
    operation   TEXT      NOT NULL,
    new_values  JSONB,
    old_values  JSONB,
    most_recent BOOLEAN   NOT NULL,
    created_at  timestamp NOT NULL DEFAULT NOW()
);

CREATE INDEX archives_table_name ON archives (table_name);
CREATE INDEX archives_record_type ON archives (record_type);
CREATE INDEX archives_record_id ON archives (record_id);
CREATE INDEX archives_operation ON archives (operation);
CREATE INDEX archives_created_at ON archives (created_at);
CREATE INDEX archives_most_recent ON archives (most_recent);
CREATE INDEX archives_table_name_most_recent ON archives (table_name, most_recent);

CREATE FUNCTION make_archive_of_changes() RETURNS TRIGGER
    LANGUAGE 'plpgsql'
AS
$$
BEGIN
    UPDATE archives
    SET most_recent = FALSE
    WHERE table_name = TG_TABLE_NAME
      AND most_recent = TRUE
      AND record_type = record_type
      AND record_id = (
        CASE
            WHEN TG_OP = 'DELETE'
                THEN OLD.id
            ELSE NEW.id
            END
        );


    IF TG_OP = 'INSERT' THEN
        INSERT INTO archives (table_name, record_type, record_id, operation, new_values, most_recent, created_at)
        VALUES (TG_TABLE_NAME, TG_ARGV[0], NEW.id, TG_OP, row_to_json(NEW), TRUE, now());
        RETURN NEW;

    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO archives (table_name, record_type, record_id, operation, new_values, old_values, most_recent,
                              created_at)
        VALUES (TG_TABLE_NAME, TG_ARGV[0], NEW.id, TG_OP, row_to_json(NEW), row_to_json(OLD), TRUE, now());
        RETURN NEW;

    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO archives (table_name, record_type, record_id, operation, old_values, most_recent, created_at)
        VALUES (TG_TABLE_NAME, TG_ARGV[0], OLD.id, TG_OP, row_to_json(OLD), TRUE, now());
        RETURN OLD;

    END IF;
END;
$$;
```

### Sample usage

```sql
CREATE TABLE users
(
    id         uuid PRIMARY KEY,
    username   TEXT        NOT NULL UNIQUE,
    created_at timestamptz NOT NULL
);

CREATE INDEX user_created_at ON users (created_at);
CREATE INDEX users_username ON users (username);

CREATE TRIGGER trg_make_archive_of_changes_for_users
    AFTER INSERT OR DELETE OR UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('User');



```

The way that it works we create a `TRIGGER` that will run after every `INSERT`/`DELETE`/`UPDATE` on the `users` table.
The `TRIGGER` will call the `make_archive_of_changes` function with the `record_type` as an argument.

We can see in `make_archive_of_changes` each `action` is handled differently.

* `DELETE` inserts the old values into the `archives` table.
* `INSERT` inserts the new values into the `archives` table without `old_values`.
* `UPDATE` inserts the new and old values into the `archives` table with `old_values`.

Using `row_to_json` we can store any `table` scheme, there is no need to update it on `migrations`.

This sleek and elegant solution allows me to easily backtrack and find any issues if needed, detect their origin and fix
as needed.

I personally use it with [sqlx](https://github.com/jmoiron/sqlx) , it's the first migration and I
make sure to add the initialization on every new table.

```sql
CREATE TRIGGER trg_make_archive_of_changes_for_users
    AFTER INSERT OR DELETE OR UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('User');
```

## References

* [paper_trail](https://github.com/paper-trail-gem/paper_trail)
* [History Tracking with Postgres](https://www.thegnar.com/blog/history-tracking-with-postgres)