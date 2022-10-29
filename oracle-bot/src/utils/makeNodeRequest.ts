import axios from "axios";

const testnetNodes = ["https://nodes-testnet.wavesnodes.com"];

const mainnetNodes = [
  "https://nodes-puzzle.wavesnodes.com",
  "https://wavesducks.wavesnodes.com",
  "https://nodes.swop.fi",
  "https://nodes.wavesnodes.com",
];

export const NODE_URL = mainnetNodes[0];

interface IParams {
  chainId?: "T" | "W";
  postData?: any;
}

export type TDataEntry = { key: string; type: string; value: string };

export interface INodeResponse<T = TDataEntry[]> {
  data: T;
}

const makeNodeRequest = async (
  request: string,
  params?: IParams
): Promise<any> => {
  const nodes =
    params?.chainId == null || params.chainId === "W"
      ? mainnetNodes
      : testnetNodes;
  return new Promise(async (resolve, reject) => {
    let nodeIndex = 0;
    let success = false;
    while (!success) {
      const url = nodes[nodeIndex] + request;
      try {
        const response = await (params?.postData == null
          ? axios.get(url)
          : axios.post(url, params.postData));
        success = true;
        resolve(response);
      } catch (reason) {
        if (nodeIndex === nodes.length - 1) {
          success = true;
          reject(reason);
        } else {
          nodeIndex = nodeIndex + 1;
        }
      }
    }
  });
};

export default makeNodeRequest;
