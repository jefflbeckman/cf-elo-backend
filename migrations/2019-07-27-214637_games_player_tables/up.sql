CREATE TABLE "players" (
  "id" serial PRIMARY KEY,
  "steam_id" varchar UNIQUE NOT NULL,
  "elo" int NOT NULL
);

CREATE TABLE "games" (
  "id" serial PRIMARY KEY,
  "upload_time" timestamp NOT NULL,
  "map_version" varchar NOT NULL,
  "good_guys_won" bool NOT NULL
);

CREATE TABLE "races" (
  "id" serial PRIMARY KEY,
  "desc" varchar UNIQUE NOT NULL
);

CREATE TABLE "games_players_link" (
  "id" serial PRIMARY KEY,
  "game_id" int NOT NULL,
  "player_id" int NOT NULL,
  "race_id" int NOT NULL
);

ALTER TABLE "games_players_link" ADD FOREIGN KEY ("race_id") REFERENCES "races" ("id");

ALTER TABLE "games_players_link" ADD FOREIGN KEY ("game_id") REFERENCES "games" ("id");

ALTER TABLE "games_players_link" ADD FOREIGN KEY ("player_id") REFERENCES "players" ("id");
