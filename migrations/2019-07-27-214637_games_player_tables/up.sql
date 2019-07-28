CREATE TABLE "players" (
  "id" int PRIMARY KEY,
  "steam_id" varchar UNIQUE NOT NULL,
  "elo" int NOT NULL
);

CREATE TABLE "games" (
  "id" int PRIMARY KEY,
  "upload_time" timestamp,
  "map_version" varchar,
  "winner" bool
);

CREATE TABLE "races" (
  "id" int PRIMARY KEY,
  "desc" varchar
);

CREATE TABLE "games_players_link" (
  "id" int PRIMARY KEY,
  "game_id" int,
  "player_id" int,
  "race_id" int,
  "leaver" int
);

ALTER TABLE "games_players_link" ADD FOREIGN KEY ("race_id") REFERENCES "races" ("id");

ALTER TABLE "games_players_link" ADD FOREIGN KEY ("game_id") REFERENCES "games" ("id");

ALTER TABLE "games_players_link" ADD FOREIGN KEY ("player_id") REFERENCES "players" ("id");
