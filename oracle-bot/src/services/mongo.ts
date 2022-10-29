import { MONGO_URL } from "../config";
const mongoose = require("mongoose");

export const initMongo = () =>
  mongoose
    .connect(MONGO_URL, {
      useNewUrlParser: true,
      useCreateIndex: true,
      useUnifiedTopology: true,
    })
    .then(() => {
      console.log("\nConnected to MongoDB ✅  ");
      /** ready to use. The `mongoose.connect()` promise resolves to undefined. */
    })
    .catch(() => {
      console.log(
        `❌  MongoDB connection error. Please make sure MongoDB is running.`
      );
      // process.exit();
    });
