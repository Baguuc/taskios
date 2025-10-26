CREATE TABLE projects (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50)
);

CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  title VARCHAR(50),
  description TEXT,
  done BOOLEAN NOT NULL,
  project_id INTEGER NOT NULL,
  FOREIGN KEY(project_id) REFERENCES projects(id)
);
