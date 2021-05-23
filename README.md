# Planetoid

Planetoid is a toy project to demonstrate and learn several technologies.
The goal is to create a little multiplayer asteriod game clone.
* Server side is a Quarkus application using websockets derived from the chat example. The goal of this application is currently to pass messages between clients.
* Client side is a Rust application using macroquad framework. As well it was derived from the asteroid example, but refactored in a more object oriented code. It can be compiled as:
    * A native application that will use websockets (tungstenite) to share game data. Only Linux has been tested so far, but it should run on Windows/MacOs as well.
    * A wasm32 application, that can be run in a browser. Currently websockets are not implemented but the game can be played in solo mode.

This project is in an early stage so a lot of features are missing and need to be implemented. However, as stated at the beginning, the goal is not to propose a real game but a demo to explain and share about these technologies.

## Authors

- [@Uggla](https://www.github.com/Uggla)


## Screenshots

Native application:
![App Screenshot](https://via.placeholder.com/468x300?text=App+Screenshot+Here)

Running the wasm application into Firefox:

## Run Locally

Clone the project

```bash
  git clone https://link-to-project
```

Go to the project directory

```bash
  cd my-project
```

Install dependencies

```bash
  npm install
```

Start the server

```bash
  npm run start
```


<!-- ## Installation -->

<!-- Install my-project with npm -->

<!-- ```bash -->
<!--   npm install my-project -->
<!--   cd my-project -->
<!-- ``` -->

