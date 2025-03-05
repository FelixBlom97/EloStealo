CREATE TABLE IF NOT EXISTS games (
    id SERIAL PRIMARY KEY,
    game JSONB NOT NULL
)

CREATE TABLE IF NOT EXISTS rules (
    id SERIAL PRIMARY KEY,
    rule JSONB NOT NULL
)

INSERT INTO rules (rule) VALUES
    ('{"name":"None", "elo":0, "description":"Good old normal chess."}')