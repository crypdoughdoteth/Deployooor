CREATE TABLE IF NOT EXISTS deployments
(
    sc_name TEXT NOT NULL,
    deployer_address varchar(42) NOT NULL,
    deploy_date  TEXT NOT NULL,
    sc_address varchar(42) NOT NULL,
    network TEXT NOT NULL
);
