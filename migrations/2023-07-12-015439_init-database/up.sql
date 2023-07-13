CREATE TABLE recipes (
  id INTEGER NOT NULL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  instructions_markdown TEXT NOT NULL,
  notes_markdown TEXT NOT NULL
);

-- Searching recipes by name is very common
CREATE INDEX recipes_name_index ON recipes (name);

CREATE TABLE ingredients (
  id INTEGER NOT NULL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  notes_markdown TEXT NOT NULL
);

CREATE TABLE recipe_ingredients (
  recipe_id INTEGER NOT NULL,
  ingredient_id INTEGER NOT NULL,
  display_order INTEGER NOT NULL,
  quantity VARCHAR(255) NOT NULL, -- Text is simpler than trying to store all units
  notes_markdown TEXT NOT NULL,

  PRIMARY KEY (recipe_id, ingredient_id),
  FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
  FOREIGN KEY (ingredient_id) REFERENCES ingredients(id) ON DELETE RESTRICT
);

CREATE TABLE tags (
  id VARCHAR(255) NOT NULL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description_markdown TEXT NOT NULL
);

CREATE TABLE recipe_tags (
  recipe_id INTEGER NOT NULL,
  tag_id VARCHAR(255) NOT NULL,
  PRIMARY KEY (recipe_id, tag_id),
  FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
  FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);