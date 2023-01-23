CREATE TABLE "metadata" (
    "hash"	TEXT NOT NULL UNIQUE,
    "name"	TEXT NOT NULL,
    "path"	TEXT NOT NULL UNIQUE,
    "contentType"	TEXT NOT NULL,
    "status"   TEXT NOT NULL,
    "timestampCreated"	INTEGER NOT NULL,
    "timestampModified"	INTEGER NOT NULL,
    "extension"	TEXT,
    "tags"	TEXT,
    "notes"	TEXT,
    "width"	INTEGER,
    "height"	INTEGER,
    "duration"	INTEGER,
    PRIMARY KEY("hash")
);