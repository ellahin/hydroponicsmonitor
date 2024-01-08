CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE devices (
  ID            uuid not null default uuid_generate_v4(),
  devicetype    text not null,
  accesskey     text not null,
  tags          json not null,
  PRIMARY KEY(ID)
);

CREATE TABLE logs (
  datetime      timestamptz not null default NOW(),
  deviceId      uuid not null,
  data          json,
  datastruct    text,
  FOREIGN KEY(deviceId) REFERENCES devices(ID)
);
