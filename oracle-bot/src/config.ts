import path from "path";
// @ts-ignore
import { config } from "dotenv";
import { loadVar } from "./utils/loadVar";

config({ path: path.join(__dirname, "../.env") });

export const port = loadVar("PORT", true);

export const TOKEN = loadVar("TOKEN");
export const CHAT_ID = loadVar("CHAT_ID");
export const LOG_CHAT_ID = loadVar("LOG_CHAT_ID");
export const MONGO_URL = loadVar("MONGO_URL", true);
export const SEED = loadVar("SEED");
