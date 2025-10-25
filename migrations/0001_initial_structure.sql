CREATE TABLE projects (
  id INTEGER PRIMARY KEY,
  name VARCHAR(50)
);

CREATE TABLE tasks (
  id INTEGER PRIMARY KEY,
  title VARCHAR(50),
  description TEXT,
  done BOOLEAN
);
