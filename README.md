# Hosted at [elostealo.felixblom.com](elostealo.felixblom.com)

# EloStealo
Chess can be fun to play with friends, but, due to the massive depth and many levels to the game, one player often ends up completely stomping the other when playing casually.
To level the playing field, me and my much weaker friend often play with [Elo Stealo](https://elostealo.com) rules.
Elo Stealo rules are handicaps of variable difficulty, unknown to your opponent, that you'll have to play around.
Since there is no website that hosts games with enforced Elo Stealo rules (elostealo.com only provides rules in text format), I decided to make one.
The elo of a Stealo rule is a guesstimate of the amount of elo a player loses while being forced to play with this rule. If both players enter their elo, EloStealo will assign both players rules that approximately level the playing field.

This is my 3-week final project for the academy part of the Software Engineering traineeship at Sogyo.

# How to run:
## Docker-compose:
The easiest way to run the project is in docker. Start it with ```docker compose up```.

Linux users may have to grant the postgres container ownership over its data volume first:
- Start only the database container with ```docker compose up postgres```
- Then run init-scripts/permissions.sh
- Stop the postgres container and start the entire app with ```docker compose up```

## Development
Make sure you have rustc and cargo installed. Instructions can be found [here](https://rustup.rs/). Make sure you have Node and npm installed.
Then,
- Start the database with ```docker compose up postgres```
- Start the backend server with ```cargo run```
- Navigate to the client folder, install dependencies with ```npm install``` and then run ```npm run build```
- Navigate to localhost:8080 to see the app.
- If you want to work on the client you can instead run ```npm run dev``` and navigate to localhost:5173 for hot reloading.
- After adding new database operations, make sure you have sqlx-cli installed, ``cargo install sqlx-cli``, and run ```cargo sqlx prepare``` to be able to containerize the app.

# Planned goals
- Add resigning and draw offers.
- Add clocks.
- Add signup, user profiles and personal elos.
- Add online matchmaking and adjust the rules' elo based on results.
- Add a cache to store active games to reduce database lookups
- Lower the amount of heap allocations in move_generator() for better performance, if benchmarks show that this is relevant.
