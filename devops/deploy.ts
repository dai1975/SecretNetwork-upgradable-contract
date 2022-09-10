import fs from "fs";
import * as lib from './lib.js';

const deploy_storage = async (wc: lib.WalletClient, config: lib.Config): Promise<lib.ContractInfo> => {
  const storageInfo = await lib.deployAndInstantiateContract(
    `${config.name}-storage`,
    wc.client,
    "../contract/storage/contract.wasm.gz",
    //"../storage-contract/contract.wasm.gz",
    {},
    config.gas.storeStorage,
    config.gas.instantiateStorage,
  );
  return storageInfo;
}

const deploy_app = async (wc: lib.WalletClient, config: lib.Config, storageInfo: lib.ContractInfo): Promise<lib.ContractInfo> => {
  const appInfo = await lib.deployAndInstantiateContract(
    `${config.name}-app`,
    wc.client,
    "../contract/application/contract.wasm.gz",
    {
      storage_address: storageInfo.address,
      storage_codehash: storageInfo.hash,
    },
    config.gas.storeApp,
    config.gas.instantiateApp,
  );
  return appInfo;
};

const setup = async (wc: lib.WalletClient, config: lib.Config, storageInfo: lib.ContractInfo, appInfo: lib.ContractInfo) => {
  const msg = {
    set_applications: { applications: [appInfo.address] },
  };
  await lib.exec(wc, storageInfo, msg, config.gas.setApplications);
}

const deploy = async (config: lib.Config, storageInfo0: lib.ContractInfo|null, wc: lib.WalletClient) => {
  const storageInfo = storageInfo0 ?? await deploy_storage(wc, config);
  const appInfo = await deploy_app(wc, config, storageInfo);
  await setup(wc, config, storageInfo, appInfo);
  const out = {
    storageInfo: storageInfo,
    appInfo: appInfo,
  };
  console.log("deployed: ", out);
  fs.writeFileSync(`${config.path}.deploy.json`, JSON.stringify(out));
};

const main = async (argv: string[]) => {
  if (argv[2] == null) {
    console.log(`${argv[1]} <config file>`);
    return;
  }
  const [config, wc] = await lib.load(argv[2]);
  const info = await lib.loadContractInfo(argv[2]);

  await deploy(config, (info==null)?null: info.storageInfo, wc[0]);
};
main(process.argv);
