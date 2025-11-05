# Farmers

## Build

### Required software

To build the whole Farmers stack, you need a bunch of software already installed on your system.

The web app part of this project is built on Angular. For this, you need to install NodeJS & npm according to the
[official instructions](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm). Also make sure you have
Angular CLI installed.
[Here's](https://angular.dev/tools/cli/setup-local) how to do that.

The backend is built in Rust using the Rocket framework and Diesel for connection to the PostgreSQL database.
The easiest way to install and update the whole Rust toolchain is [rustup](https://rustup.rs/).
I recommend to also install [PostgreSQL](https://www.postgresql.org/) locally to allow compilation on your machine
directly instead of a container. This will make it more comfortable to build and run the server locally.
How you install PostgreSQL on your system depends on the distribution you are using. Most distributions come with a
package manager providing PostgreSQL.

If you want to run the Robot/Selenium frontend tests, you will need the [Robot Framework](https://robotframework.org/)
and the [SeleniumLibrary](https://robotframework.org/SeleniumLibrary/) installed either in your system directly or
in a python venv. I recomment the latter one as it does not leave unnecessary traces and clutter in your system.
You can create such a venv like this:

```shell
python -m venv robot-venv
echo '*' > robot-venv/.gitignore # prevent git from accidentally adding your venv 
source robot-venv/bin/activate
pip install robotframework robotframework-seleniumlibrary
```

To come back to your venv just run `source robot-venv/bin/activate` in the robot directory again.

### Build instructions

#### Web Frontend

Building the web frontend is very straight forward. All it should need are two commands inside the projects `web`
directory:

```shell
npm install # this installs all the required packages into the projects node_modules
ng build # compile typescript code and assemble dist
```

#### Server

The backend server is built with the normal `cargo build` command.

#### Map data

For now, the easiest way to get and assemble the required vector tiles for the map server is by using the provided
`download-mbtiles.sh` script. This defaults to downloading the area of Switzerland as it is of significant size but not
too big to take a long time. With a good internet connection and somewhat decent components in your system, this should
only take a few minutes. If you run into memory issues, you can simply decrease the amount of maximum memory given to
the java process by adjusting the `Xmx32g` argument to an appropriate number.

Alternatively you can also provide your own `.mbtiles` file under `map/martin/tiles.mbtiles`. In this case you are
responsible yourself to make sure that everything is compatible.

## Run

### Map server

The martin map server can be started by running `docker compose up` in the `map` directory. This should automatically
start the required PostgreSQL container and pick up the previously exported `tiles.mbtiles` file. By default, the web
interface is enabled on port `3000` but you can also choose to disable it in the command inside the
`docker-compose.yml`.

### Database

The project provides a `postgres-container.sh` script that can be used to easily start an instance of a PostgreSQL
container with some basic parameters. This is great for testing purposes, but I would not recomment to use it in a 
production environment. For that, you will have to either install PostgreSQL on the local machine, another machine
or create your own container that is managed by you to cater your specific needs. Create a user for the server and
add the connection to your configuration as described in the next chapter.

### Backend server

Before you run the backend server, make sure you created a `.env` file with all the required values or set them in your
environment. Adjust the URL in `ROCKET_DATABASES` to point to the PostgreSQL instance you want to use for application 
data. If you use the provided `postgres-container.sh` script, there is an option to overwrite this line automatically
and all you need to provide is a line starting with `ROCKET_DATABASES`. The `WEBAPP_PATH` is optional in your 
environment and the default output directory of the Angular build will be used if nothing is configured.

```dotenv
ROCKET_DATABASES={pgfarm={url="postgres://farmers:farmers@localhost:5432/farmers"}}
JWT_SECRET=asdf
WEBAPP_PATH=web/dist/farmers/browser
# Only needed for production builds, use your own value. Create for example like this:
# $ openssl rand -base64 32
ROCKET_SECRET_KEY=G3SWHLnyRydMHv+58E6dA/u/tGVVlFDe9jceWMKDHKY=
```

To configure your Rocket instance, you can change the contents of `Rocket.toml`. Some information about possible options
can be found in the [official documentation](https://rocket.rs/guide/v0.5/configuration/) of Rocket.

You don't have to create or migrate any database tables manually as Rocket will run all migrations automatically on
startup.

To start the server, you can either run `cargo run` (alternatively with `--release`) or execute the binary created by
`cargo build` earlier.

### Docker images

This project provides you with a Dockerfile that automatically build and packs the Backend server and webapp part into
one image. This is the easiest way to build and run the image:

```shell
docker build -t <IMAGE_NAME> .
docker run --env-file ./.env --network=host -t -i --rm <IMAGE_NAME>
```

This will build the whole server and webapp part into one convenient image and then runs it in an interactive terminal.
That makes it easy to stop with Ctrl-C. The `--rm` makes sure the container gets removed after use since we don't have
any need for persistent data inside the container because it only runs the server. With `--env-file` we provide the 
`.env` file explained earlier. Since the docker image will contain a release build of the server, make sure to set the
`ROCKET_SECRET_KEY` property here. To easily connect to the database server running elsewhere (at least not in the same
container), this command will use the host network.

Keep in mind that this is just an example and in a real scenario, you would probably want to have a more elaborate
setup containing all required containers in some kind of orchestration tool.

## Tests

Tests are somewhat limited so far as my main goal was to get something running fast and learn the basics of Angular,
creating and serving map tiles and also getting a bit more used to Rocket and Diesel. Running the rust tests comes with
a small caveat. Since the api tests each start a dedicated Rocket instance, you will most definitely run into problems
with the number of concurrent database connections. Maybe this will be fixed in the future by providing a different 
configuration or setup but for now, you will have to run tests with a flag: `cargo test -- --test-threads=1`. For the
tests to pass, the configured database must be running (here the temporary container from the provided script comes in
handy).

The project also comes with some rudimentary Robot browser tests. So far those are not doing anything more than just 
checking basic behavior like creating users, login and simple navigation. For those tests, the server and its database
must both be running. It does not matter if the server is running in debug or release mode but for easier analysis of
possible errors with requests, debug mode is recommended. The tests create some basic data in the database but also make 
sure to delete all of it again by the end. If there are test failures, some data might still be left in the database and
you will either have to clean it up yourself or just recreate the database container to start clean.
