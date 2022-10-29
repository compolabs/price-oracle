import { TDataEntry } from "./makeNodeRequest";

export const getStateByKey = (values: TDataEntry[], key: string) =>
  values.find((state) => state.key === key)?.value;
