import BigNumber from 'bignumber.js';
import * as lib from './lib.js';

const main = async (argv: string[]) => {
console.log(argv);
  if (argv[2] == null) {
    console.log(`${argv[1]} <config file>`);
    return;
  }
  const [config, wc] = await lib.load(argv[2]);

  const r = await lib.getScrtBalance(wc);
  const uscrt = new BigNumber(r);
  const scrt = uscrt.div(1000000);
  console.log(`${uscrt.toString()}uscrt`);
  console.log(`${scrt.toString()}scrt`);
};
main(process.argv);
