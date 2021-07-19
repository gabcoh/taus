CREATE TABLE config (
       id INTEGER PRIMARY KEY NOT NULL,
       fill_version TEXT NOT NULL,
       asset_regex TEXT NOT NULL,
       github_user TEXT,
       github_repo TEXT
);
INSERT INTO config (fill_version, asset_regex) VALUES ("LATEST", "app-name-(linux|windows|max)");
