import { InlineKeyboardButton } from "node-telegram-bot-api";

export const sleep = (ms: number) =>
  new Promise((resolve) => setTimeout(resolve, ms));

export const createInlineButton = (
  text: string,
  key: string | number,
  callback_data?: any
): InlineKeyboardButton => ({
  text,
  callback_data: JSON.stringify({ key, data: callback_data }),
});
