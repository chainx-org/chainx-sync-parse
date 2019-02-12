CREATE TABLE blocks (
  height BIGINT  NOT NULL,
  prefix VARCHAR NOT NULL,
  value  JSON    NOT NULL,
  PRIMARY KEY (height, prefix)
);
