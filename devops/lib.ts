import axios from "axios";
import { Wallet, SecretNetworkClient, CodeInfoResponse, QueryContractsByCodeResponse, fromUtf8 } from "secretjs";
import fs from "fs";
import assert from "assert";
import * as Storage from '../contract/storage/schema';
import * as App from '../contract/application/schema';

export type WalletClient = {
  wallet: Wallet,
  client: SecretNetworkClient,
  chainId: string,
};
export type ContractInfo = {
  address: string,
  hash: string,
};
export type Config = {
  path: string,
  name: string,
  endpoint: string,
  chainId: string,
  mnemonic: string[],
  storageInfo?: {
    address: string,
    hash: string,
  },
  appInfo?: {
    address: string,
    hash: string,
  },
  gas: {
    storeStorage: number | null,
    instantiateStorage: number | null,
    storeApp: number | null,
    instantiateApp: number | null,
    setApplications: number | null,
  }
};
export const loadConfig = async (path: string): Promise<Config> => {
  const check = (o:any, k:string, t:string, nullable:boolean) => {
    if (! o.hasOwnProperty(k) || o[k] == null) {
      if (!nullable) {
        throw new Error(`${k} should be exist`);
      }
    } else {
      if (t === 'array') {
        if (!Array.isArray(o[k])) {
          throw new Error(`invalid type of ${k}: ${t} but ${typeof(o[k])}`);
        }
      } else {
        if (typeof(o[k]) !== t) {
          throw new Error(`invalid type of ${k}: ${t} but ${typeof(o[k])}`);
        }
      }
    }
    return true;
  };
  const d = JSON.parse(fs.readFileSync(path, 'utf8')) as any;
  check(d, "name", "string", false);
  check(d, "endpoint", "string", false);
  check(d, "chainId", "string", false);
  check(d, "mnemonic", "array", false);
  check(d, "gas", "object", false);
  check(d.gas, "storeStorage", "number", true);
  check(d.gas, "instantiateStorage", "number", true);
  check(d.gas, "storeApp", "number", true);
  check(d.gas, "instantiateApp", "number", true);
  check(d.gas, "createStorage", "number", true);
  if (d.hasOwnProperty("storageInfo")) {
    check(d.storageInfo, "address", "string", true);
    check(d.storageInfo, "hash", "string", true);
  }
  if (d.hasOwnProperty("appInfo")) {
    check(d.appInfo, "address", "string", true);
    check(d.appInfo, "hash", "string", true);
  }
  d.path = path;
  return d as Config;
}
export const loadContractInfo = async (path0: string): Promise<{storageInfo: ContractInfo, appInfo: ContractInfo}|null> => {
  const path = `${path0}.deploy.json`;
  if (!fs.existsSync(path)) {
    return null;
  }
  const d = JSON.parse(fs.readFileSync(path, 'utf8')) as any;
  return {
    storageInfo: {
      address: d.storageInfo.address,
      hash: d.storageInfo.hash,
    } as ContractInfo,
    appInfo: {
      address: d.appInfo.address,
      hash: d.appInfo.hash,
    } as ContractInfo,
  };
}


// Returns a client with which we can interact with secret network
const get_wc = async (config: Config, mnemonic: string): Promise<WalletClient> => {
  const wallet = new Wallet(mnemonic);
  const accAddress = wallet.address;
  const client = await SecretNetworkClient.create({
    // Create a client to interact with the network
    grpcWebUrl: config.endpoint,
    chainId: config.chainId,
    wallet: wallet,
    walletAddress: accAddress,
  });

  //console.log(`Initialized client with wallet address: ${accAddress}`);
  return {
    wallet,
    client,
    chainId: config.chainId,
  } as WalletClient;
};
export const initializeClient = async (config: Config): Promise<WalletClient[]> => {
  return Promise.all(config.mnemonic.map(async s => get_wc(config, s)))
};
export const dummyClients = async (config: Config): Promise<WalletClient[]> => {
  const data = [
    {"name":"u1","type":"local","address":"secret12uen7y0ll42y887qgccr0g37rtga7p8vhqjdgx","pubkey":"{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"AzGFVviKoe0ZHq+t1f+mw/LHjMBWVgYydrwedtosuA1r\"}","mnemonic":"omit siren result bomb click junior shoe cream horror spoil okay wood purity siren extend hen benefit snake frame battle reflect moon merit undo"},
    {"name":"u2","type":"local","address":"secret1kqx2vyz47gr2tuulkvaf0cvn43wt4kphy8qsc6","pubkey":"{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"A7WRCgpMpNpHMPRWz8SmKxIxyM+tkXxZLan95KRpk+uA\"}","mnemonic":"pioneer second husband paper catch wolf federal rib list scan clown retreat length town runway enable change second load object dress industry amateur master"},
    {"name":"u3","type":"local","address":"secret1eescgdd7n9emlygpg46shyhq74s3xvmmwqgpyt","pubkey":"{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"AjAS/8JeHc1Stck6JPrrMkZiiqiuUmndxBqGE10akON+\"}","mnemonic":"budget post bunker this scout clarify salon glance cannon decorate short initial admit dog vehicle bitter female reunion typical toe lottery glow grace muffin"},
  ];
  return Promise.all(data.map(async d => get_wc(config, d.mnemonic)));
};

export const load = async(config_path: string): Promise<[Config, WalletClient[]]> => {
  const config = await loadConfig(config_path);
  const wc = await initializeClient(config);
  return [config, wc];
};

// Stores and instantiaties a new contract in our network
export const deployAndInstantiateContract = async (
  label: string,
  client: SecretNetworkClient,
  contractPath: string,
  initMsg: object,
  storeGas: number|null,
  instantiateGas: number|null,
): Promise<ContractInfo> => {
  const now = new Date();
  const nowString = now.toISOString();
  const wasmCode = fs.readFileSync(contractPath);
  console.log("Uploading contract: " + label);

  const uploadReceipt = await client.tx.compute.storeCode(
    {
      wasmByteCode: wasmCode,
      sender: client.address,
      source: "",
      builder: "",
    },
    {
      gasLimit: storeGas ?? 5000000,
    }
  );

  if (uploadReceipt.code !== 0) {
    console.log(
      `Failed to get code id: ${JSON.stringify(uploadReceipt.rawLog)}`
    );
    throw new Error(`Failed to upload contract`);
  }
  console.log(`gasUsed: ${uploadReceipt.gasUsed}`);

  const codeIdKv = uploadReceipt.jsonLog![0].events[0].attributes.find(
    (a: any) => {
      return a.key === "code_id";
    }
  );

  const codeId = Number(codeIdKv!.value);
  console.log("Contract codeId: ", codeId);

  const contractCodeHash = await client.query.compute.codeHash(codeId);
  console.log(`Contract hash: ${contractCodeHash}`);

  const contract = await client.tx.compute.instantiateContract(
    {
      sender: client.address,
      codeId,
      initMsg,
      codeHash: contractCodeHash,
      label: `${label} ${nowString}`, // The label should be unique for every contract, add random string in order to maintain uniqueness
    },
    {
      gasLimit: instantiateGas ?? 1000000,
    }
  );

  if (contract.code !== 0) {
    throw new Error(
      `Failed to instantiate the contract with the following error: ${contract.rawLog}`
    );
  }
  console.log(`gasUsed: ${contract.gasUsed}`);

  const contractAddress = contract.arrayLog!.find(
    (log) => log.type === "message" && log.key === "contract_address"
  )!.value;

  console.log(`Contract address: ${contractAddress}`);

  return {
    address: contractAddress,
    hash: contractCodeHash,
  } as ContractInfo;
};

export const getScrtBalance = async (wc: WalletClient): Promise<string> => {
  let balanceResponse = await wc.client.query.bank.balance({
    address: wc.client.address,
    denom: "uscrt",
  });
  return balanceResponse.balance!.amount;
};

export const listCodes = async (wc: WalletClient): Promise<CodeInfoResponse[]> => {
  const r = await wc.client.query.compute.codes();
  //console.log(r);
  return r;
};
export const listContracts = async (wc: WalletClient, codeId: number): Promise<QueryContractsByCodeResponse> => {
  const r = await wc.client.query.compute.contractsByCode(codeId);
  return r;
};

export const createPermit = async (wc: WalletClient, addr: string): Promise<Storage.PermitFor_Permissions> => {
  const keplr = false;
  const permit = await wc.client.utils.accessControl.permit.sign(
    wc.wallet.address,
    wc.chainId,
    "",
    [addr],
    ["access" as any],
    keplr
  ) as any;
  //console.log('sig', permit.signature.signature);
  // test sig. decode b64 and modify
  //permit.signature.signature = 'xxx'+permit.signature.signature;
  return permit;
};

export const exec = async (wc: WalletClient, contract: ContractInfo, msg: object, gas: number|null): Promise<string> => {
  const args = {
    sender: wc.wallet.address,
    contractAddress: contract.address,
    codeHash: contract.hash,
    msg
  };
  const gasLimit = await (async (g: number|null) => {
    if (g == null) {
      console.log("simulate...", args);
      const r0 = await wc.client.tx.compute.executeContract.simulate(args);
      //console.log('simulate: ', r0);
      if (r0.gasInfo == null) { throw "unexpected gas used"; }
      console.log("gasInfo: ", r0.gasInfo);
      return parseInt(r0.gasInfo.gasUsed) + 10000;
    } else {
      return g;
    }
  })(gas);

  console.log("exec: ", msg);
  const r = await wc.client.tx.compute.executeContract(args, { gasLimit })
  if (r.rawLog.startsWith('failed')) {
    throw new Error(r.rawLog);
  }
  //console.log(r);
  const ret = fromUtf8(r.data[0]);

  return ret;
}

export const query = async (wc: WalletClient, contract: ContractInfo, msg: object): Promise<any> => {
  const args = {
    contractAddress: contract.address,
    codeHash: contract.hash,
    query: msg,
  };
  console.log("query: ", msg);
  const r = await wc.client.query.compute.queryContract(args) as string;
  //console.log(r);
  if (typeof(r) === 'string') {
    const err = 'Generic error: Querier system error: Cannot parse response: expected value at line 1 column 1 in: ';
    if (r.startsWith(err)) { throw new Error(r.substring(err.length)); }
    else throw new Error(r);
  }
  return r;
}
