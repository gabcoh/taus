CREATE TABLE targets (
       id INTEGER PRIMARY KEY NOT NULL,
       target TEXT UNIQUE NOT NULL,
       regex TEXT NOT NULL
);
CREATE TABLE target_version_mappings (
       target_id INTEGER NOT NULL,
       current_version TEXT NOT NULL,
       update_version TEXT NOT NULL,
       PRIMARY KEY (target_id, current_version)
       FOREIGN KEY(target_id) REFERENCES targets(id)
);
