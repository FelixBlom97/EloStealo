## Not functional ATM

# IP
Chess can be fun to play with friends, but, due to the massive depth and many levels to the game, one player often ends up completely stomping the other when playing casually.
To level the playing field, me and my much weaker friend often play with [Elo Stealo](https://elostealo.com) rules.
Elo Stealo rules are random handicaps of variable difficulty, unknown to your opponent, that you'll have to play around.
Since there is no website that hosts games with enforced Elo Stealo rules (elostealo.com only provides rules in text format), I decided to make one.

This is my 3-week final project for the academy part of the Software Engineering traineeship at Sogyo.

## Build
Make sure you have rustc and cargo installed. Instructions can be found [here](https://rustup.rs/). Make sure you have Node and npm installed.
Then,
- Start the backend server with ```cargo run```
- Navigate to the client folder, install dependencies with ```npm install``` and then start the frontend server with ```npm run dev```

##  Planning and MoSCoW
- After week 1 I want to have a minimum viable product: a working LocalGame webapp with at least one elo stealo rule.
- Since your Elo Stealo rule should not be known to your opponent, there should be a large(ish) variety of rules available.
In week 2 I'll implement as many as possible.
- My goal for week 3 is to allow online play.
- If I had more time I'd improve the client, add clocks, implement matchmaking, and I'd base the elo penalties on past results (right now they are just guesstimates).

### Must: 
- A client with a form to start games.
- A chess game frontend.
- A backend that executes moves made at the front-end and determines possible follow-up moves.
- At least one Elo Stealo rule, selectable in the front-end and enforced in the back-end.
- A database to store the games.

### Should:
- A large selection of Elo Stealo rules, stored in a database.
- A field in the form to select Elo Stealo rules.
- A button to randomly select rules that fit given Elo's.
- A check that determines if the game has ended. If you have no moves due to your Elo Stealo rule you lose, stalemate is still a draw.

### Could:
- Online play.
- Resigning and draw offers.
- Clicking on a piece shows available moves (can be hard to remember in Elo Stealo).

### Won't:
- Online matchmaking.
- Elo Stealo penalties adjust based on past results. 
- Clocks.

## Software stack
The back-end is written in Rust.

For the front-end I used [React](https://react.dev/).

For my database I 

## Personal learning goals
Working in sprints and sticking to the current issue.

## Technical learning goals
Learn the basics of rust to build a backend game server.

Learn how to do online play on websockets.

# Goals for after the IP
### Properly deploy the app:
 - Make the app more production ready: better error handling, proper routing, integration testing
 - Learn how to use Docker and set up continuous deployment