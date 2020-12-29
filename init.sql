CREATE TABLE activities (
  id INTEGER PRIMARY KEY NOT NULL,
  amount double NOT NULL,
  timestamp DATETIME NOT NULL,
  ownerAccountId INT NOT NULL,
  sourceAccountId INT NOT NULL,
  targetAccountId INT NOT NULL
); 

CREATE TABLE accounts (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT
);
