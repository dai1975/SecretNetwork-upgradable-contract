import fs from "fs";
import * as lib from './lib.js';
import * as App from '../contract/application/schema';
import * as assert from "assert";

const setData = async (wc: lib.WalletClient, info: lib.ContractInfo, key: string, value: number, authz: App.Authz) => {
  const gas = 240000; //220803
  const permit = await lib.createPermit(wc, info.address);
  await lib.exec(wc, info, { set: {
    key,
    permit,
    value,
    authz,
  } }, gas);
}

const getData = async (wc: lib.WalletClient, info: lib.ContractInfo, key: string): Promise<number|null> => {
  const permit = await lib.createPermit(wc, info.address);
  const r = await lib.query(wc, info, { get: {
    key,
    permit,
  } }) as App.QueryAnswer;
  //console.log('getData=', r);
  return r.value;
}

const main = async (argv: string[]) => {
  if (argv[2] == null) {
    console.log(`${argv[1]} <config file>`);
    return;
  }
  const [config, wc] = await lib.load(argv[2]);
  const info = await lib.loadContractInfo(argv[2]);
  if (info == null) { throw new Error("deploy info file not found"); }
  const dummy = await lib.dummyClients(config);

  console.log(info);
  const key = "test";

  if (await getData(dummy[0], info.appInfo, `${key}-1`) == null) {
    await setData(wc[0], info.appInfo, `${key}-1`, 31, "p_u_b_l_i_c");
    await setData(wc[0], info.appInfo, `${key}-2`, 52, "p_r_i_v_a_t_e");
    await setData(wc[0], info.appInfo, `${key}-3`, 73, { p_r_o_t_e_c_t_e_d: dummy[0].wallet.address });
  }

  // cannot overwrite
  assert.rejects(setData(wc[0], info.appInfo, `${key}-1`, 0, "p_u_b_l_i_c"), {
    name: 'Error',
  });

  assert.equal(await getData(dummy[0], info.appInfo, `${key}-1`), '31');
  assert.rejects(getData(dummy[0], info.appInfo, `${key}-2`), {
    name: 'Error',
    message: 'Generic error: unauthorized'
  });
  assert.equal(await getData(dummy[0], info.appInfo, `${key}-3`), '73');
  assert.rejects(getData(dummy[1], info.appInfo, `${key}-3`), {
    // dummy[1] is not authz for key-3
    name: 'Error',
    message: 'Generic error: unauthorized'
  });
};

main(process.argv);
