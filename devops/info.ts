import BigNumber from 'bignumber.js';
import * as lib from './lib.js';

const main = async (argv: string[]) => {
console.log(argv);
  if (argv[2] == null) {
    console.log(`${argv[1]} <config file>`);
    return;
  }
  const [config, wc] = await lib.load(argv[2]);

  const codes = await lib.listCodes(wc);
  for (let i=1; i<5; ++i) {
    if (codes.length < i) { break; }
    const c = codes[codes.length-i];
    const codeId = parseInt(c.codeId);
    console.log(c);
    const contracts = await lib.listContracts(wc, codeId);
    console.log(contracts);
  }
};
main(process.argv);
