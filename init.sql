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

INSERT INTO accounts (name) values 
("Bob"),
("Steve"),
("Andy");

INSERT INTO activities (amount, timestamp, ownerAccountId, sourceAccountId, targetAccountId) VALUES 
(4200.0, "2020-12-29T01:28:20.071328", 3, 12, 3),
(2200.0, "2020-12-29T01:28:20.071328", 2, 12, 2),
(1300.0, "2020-12-29T01:28:20.071328", 1, 12, 1);
