CREATE TABLE IF NOT EXISTS games (
    id VARCHAR(11) PRIMARY KEY,
    game BYTEA NOT NULL,
    white VARCHAR(50) NULL,
    black VARCHAR(50) NULL,
    white_id UUID NULL,
    black_id UUID NULL,
    elo_white INTEGER NOT NULL,
    elo_black INTEGER NOT NULL,
    rule_id_white INTEGER NOT NULL,
    rule_id_black INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS rules (
    id INTEGER PRIMARY KEY,
    name VARCHAR(60) NOT NULL,
    elo INTEGER NOT NULL,
    description VARCHAR(120) NOT NULL
);