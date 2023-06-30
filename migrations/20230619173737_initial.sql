-- Create the users table
CREATE TABLE users (
    uuid UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp
);

-- Create an index on the users table for the name column
CREATE INDEX idx_users_name ON users (name);

-- Create the stacks table
CREATE TABLE stacks (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(500),
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    user_uuid UUID NOT NULL REFERENCES users (uuid) ON DELETE CASCADE
);

-- Create an index on the stacks table for the name column
CREATE INDEX idx_stacks_name ON stacks (name);

-- Create the technologies table
CREATE TABLE technologies (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    description VARCHAR(500),
    purpose VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp
);

-- Create an index on the technologies table for the name column
CREATE INDEX idx_technologies_name ON technologies (name);

-- Create the join table for the relationship between stacks and technologies
CREATE TABLE stack_technology (
    stack_id INTEGER REFERENCES stacks (id) ON DELETE CASCADE,
    technology_id INTEGER REFERENCES technologies (id) ON DELETE CASCADE,
    PRIMARY KEY (stack_id, technology_id)
);
