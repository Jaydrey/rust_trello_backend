
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone_number VARCHAR(20) NOT NULL,
    password TEXT NOT NULL,
    role VARCHAR(20) NOT NULL,
    date_of_birth TEXT,
    active BOOLEAN NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT
);
