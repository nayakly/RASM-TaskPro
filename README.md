# RASM-TodoList
 Todolist powered by Rust, web3-based Authentication, Svelte, and MongoDB. It is built from scratch, with a responsive design and supports dark mode. 

## Setting Up
### 1. Clone/Download the Repository

### 2. Configure Environment Variables for the Server

Sign up for a free [MongoDB Atlas](https://www.mongodb.com/atlas/database) account. Create a .env file in the `/server` directory with the following variables to allow the server to interact with your MongoDB collection:

```
MONGODB_URI="YOUR_MONGODB_URI"
DB_NAME="YOUR_DB_NAME"
COLLECTION_NAME="YOUR_COLLECTION_NAME"
```

### 3.  Install Dependencies and Run the Server

Install the dependencies defined in `Cargo.toml` file using the following command:

```
cargo build
```

You can now run the web server with the following command:

```
cargo run
```

### 4. Configure Environment Variables for the App
To connect the app with the server, create a `.env` file in the `/app` directory with the following variable:

    VITE_SERVER_URL="YOUR_SERVER_URL"

### 5. Install Dependencies and Run the App

Install the app dependencies by running the following command:

```
npm install
```

Now, run the app on a development server with the following command:

```
npm run dev
```

Make sure you have [Metamask](https://metamask.io/) installed and listen to `Goerli` to interact with the dapp. If things go well, you should see this on your screen. 
