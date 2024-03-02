CREATE TABLE IF NOT EXISTS deployments
(
    sc_name TEXT NOT NULL,
    deployer_address varchar(42) NOT NULL,
    deploy_date  TEXT NOT NULL,
    sc_address varchar(42) NOT NULL,
    network TEXT NOT NULL,
    fee TEXT NOT NULL,
    verified BOOL NOT NULL
);

CREATE TABLE IF NOT EXISTS keys 
(
    name TEXT NOT NULL PRIMARY KEY,
    path TEXT UNIQUE NOT NULL
);
