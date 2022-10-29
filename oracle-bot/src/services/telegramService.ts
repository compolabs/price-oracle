import TelegramBot from "node-telegram-bot-api";
import { CHAT_ID, LOG_CHAT_ID, TOKEN } from "../config";

class TelegramService {
  telegram: TelegramBot;
  constructor() {
    this.telegram = new TelegramBot(TOKEN, { polling: true });
    this.setupListeners();
  }
  private setupListeners = () => {
    this.telegram.onText(/\/id/, async ({ chat: { id } }) => {
      await this.telegram
        .sendMessage(id, String(id))
        .catch(() => console.log(`❗️cannot send message to ${id}`));
    });
  };
  log = (msg: string) => this.telegram.sendMessage(LOG_CHAT_ID, msg);
  groupMessage = (msg: string) => this.telegram.sendMessage(CHAT_ID, msg);
}
export default TelegramService;
