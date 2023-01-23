CREATE TABLE "smart_folder" (
    "name"	TEXT NOT NULL UNIQUE,
    "path"	TEXT NOT NULL UNIQUE,
    "numberOfFiles"	INTEGER NOT NULL,
    PRIMARY KEY("path")
);